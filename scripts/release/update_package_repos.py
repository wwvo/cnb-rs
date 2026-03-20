#!/usr/bin/env python3
"""Update Homebrew and Scoop package repositories for a published release."""

from __future__ import annotations

import argparse
import base64
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.parse
import urllib.request
from pathlib import Path


RELEASE_TAG_PATTERN = re.compile(
    r"^v[0-9]+\.[0-9]+\.[0-9]+(-(alpha|beta)(\.[0-9]+)?)?$"
)
SHA256SUM_LINE_PATTERN = re.compile(r"^([0-9a-f]{64}) [ *](.+)$")

GITHUB_REPO_DEFAULT = "wwvo/cnb-rs"
GITHUB_WEB_ENDPOINT_DEFAULT = "https://github.com"
CNB_WEB_ENDPOINT_DEFAULT = "https://cnb.cool"
HOMEBREW_REPO_SLUG_DEFAULT = "wwvo/cnb-rs/homebrew-cnb-rs"
SCOOP_REPO_SLUG_DEFAULT = "wwvo/cnb-rs/scoop-cnb-rs"
HOMEBREW_FORMULAE = {
    "cnb-rs.rb": (
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu",
    ),
    "cnb-rs-musl.rb": (
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-unknown-linux-musl",
        "x86_64-unknown-linux-musl",
    ),
}


class UpdateError(RuntimeError):
    """Raised when the package repo update workflow cannot continue."""


def log(message: str) -> None:
    print(f"[update-package-repos] {message}", flush=True)


def normalize_base_url(value: str) -> str:
    return value.rstrip("/")


def asset_stem(release_tag: str, target: str) -> str:
    return f"cnb-rs-{release_tag}-{target}"


def asset_filename(release_tag: str, target: str, extension: str) -> str:
    return f"{asset_stem(release_tag, target)}.{extension}"


def github_asset_url(
    github_web_endpoint: str,
    github_repo: str,
    release_tag: str,
    filename: str,
) -> str:
    base_url = normalize_base_url(github_web_endpoint)
    return f"{base_url}/{github_repo}/releases/download/{release_tag}/{filename}"


def cnb_clone_url(cnb_web_endpoint: str, repo_slug: str) -> str:
    parsed = urllib.parse.urlparse(normalize_base_url(cnb_web_endpoint))
    if not parsed.scheme or not parsed.netloc:
        raise UpdateError(f"Invalid CNB web endpoint: {cnb_web_endpoint}")

    netloc = parsed.netloc
    path_prefix = parsed.path.rstrip("/")
    path = f"{path_prefix}/{repo_slug}.git"
    return urllib.parse.urlunparse((parsed.scheme, netloc, path, "", "", ""))


def git_auth_env(token: str | None) -> dict[str, str] | None:
    if not token:
        return None

    credential = base64.b64encode(f"cnb:{token}".encode("utf-8")).decode("ascii")
    env = os.environ.copy()
    env["GIT_CONFIG_COUNT"] = "1"
    env["GIT_CONFIG_KEY_0"] = "http.extraheader"
    env["GIT_CONFIG_VALUE_0"] = f"AUTHORIZATION: basic {credential}"
    return env


def require_cnb_token(token: str | None) -> str:
    if not token:
        raise UpdateError("CNB_TOKEN is required when --push is set.")
    return token


def run_cmd(
    args: list[str],
    *,
    cwd: Path | None = None,
    capture_output: bool = False,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    location = f" (cwd={cwd})" if cwd else ""
    log(f"Run: {' '.join(args)}{location}")
    return subprocess.run(
        args,
        cwd=cwd,
        env=env,
        check=True,
        text=True,
        capture_output=capture_output,
    )


def git_output(repo_dir: Path, *args: str) -> str:
    completed = run_cmd(["git", *args], cwd=repo_dir, capture_output=True)
    return completed.stdout.strip()


def ensure_clean_worktree(repo_dir: Path) -> None:
    status = git_output(repo_dir, "status", "--short")
    if status:
        raise UpdateError(
            f"Repository {repo_dir} has uncommitted changes; refusing to push on top of a dirty worktree."
        )


def prepare_repo(
    *,
    repo_dir: Path | None,
    repo_slug: str,
    cnb_web_endpoint: str,
    cnb_token: str | None,
    push: bool,
    work_root: Path | None,
) -> Path:
    if repo_dir is None:
        if work_root is None:
            raise UpdateError("Missing work root for repository clone.")
        repo_dir = work_root / Path(repo_slug).name

    clone_url = cnb_clone_url(cnb_web_endpoint, repo_slug)
    git_env = git_auth_env(cnb_token) if push else None

    if repo_dir.exists():
        if not (repo_dir / ".git").exists():
            raise UpdateError(f"{repo_dir} exists but is not a git repository.")
    else:
        repo_dir.parent.mkdir(parents=True, exist_ok=True)
        run_cmd(["git", "clone", clone_url, str(repo_dir)], env=git_env)

    if push:
        ensure_clean_worktree(repo_dir)
        run_cmd(["git", "remote", "set-url", "origin", clone_url], cwd=repo_dir)
        run_cmd(["git", "checkout", "main"], cwd=repo_dir)
        run_cmd(["git", "pull", "--ff-only", "origin", "main"], cwd=repo_dir, env=git_env)

    return repo_dir


def fetch_text(url: str, *, attempts: int = 5, timeout: int = 30) -> str:
    request = urllib.request.Request(
        url,
        headers={"User-Agent": "cnb-rs package repo updater"},
        method="GET",
    )
    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                return response.read().decode("utf-8")
        except Exception as exc:  # pragma: no cover - network failure path
            last_error = exc
            if attempt == attempts:
                break
            wait_seconds = attempt * 2
            log(
                f"Fetch failed for {url} ({exc}); retrying in {wait_seconds}s "
                f"[attempt {attempt}/{attempts}]"
            )
            time.sleep(wait_seconds)

    raise UpdateError(f"Failed to fetch {url}: {last_error}")


def resolve_latest_release_tag(github_web_endpoint: str, github_repo: str) -> str:
    latest_url = f"{normalize_base_url(github_web_endpoint)}/{github_repo}/releases/latest"
    request = urllib.request.Request(
        latest_url,
        headers={"User-Agent": "cnb-rs package repo updater"},
        method="HEAD",
    )

    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            resolved_url = response.geturl().rstrip("/")
    except Exception as exc:  # pragma: no cover - network failure path
        raise UpdateError(
            f"Failed to resolve latest stable release tag from {latest_url}: {exc}"
        ) from exc

    release_tag = resolved_url.rsplit("/", 1)[-1].strip()
    if not release_tag or release_tag == "latest":
        raise UpdateError(
            f"Failed to parse latest stable release tag from redirect target: {resolved_url}"
        )

    return release_tag


def parse_checksums(contents: str) -> dict[str, str]:
    checksums: dict[str, str] = {}
    for line in contents.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        match = SHA256SUM_LINE_PATTERN.fullmatch(stripped)
        if not match:
            raise UpdateError(f"Invalid sha256sum line: {line}")
        digest, filename = match.groups()
        checksums[filename] = digest
    return checksums


def require_checksum(checksums: dict[str, str], filename: str) -> str:
    digest = checksums.get(filename)
    if digest is None:
        raise UpdateError(f"Missing checksum for required release asset: {filename}")
    return digest


def replace_formula_block(
    content: str,
    *,
    target: str,
    new_url: str,
    new_sha256: str,
) -> str:
    pattern = re.compile(
        rf'(?P<indent>\s*)url\s+"[^"]*/cnb-rs-v?[^"/]+-{re.escape(target)}\.tar\.gz"\n'
        rf'(?P=indent)sha256\s+"[0-9a-f]{{64}}"',
        re.MULTILINE,
    )

    def replacement(match: re.Match[str]) -> str:
        indent = match.group("indent")
        return f'{indent}url "{new_url}"\n{indent}sha256 "{new_sha256}"'

    updated, count = pattern.subn(replacement, content, count=1)
    if count != 1:
        raise UpdateError(
            f"Failed to update Homebrew formula block for target {target}; expected 1 match, got {count}."
        )
    return updated


def update_homebrew_repo(
    repo_dir: Path,
    *,
    release_tag: str,
    github_web_endpoint: str,
    github_repo: str,
    checksums: dict[str, str],
) -> None:
    formula_dir = repo_dir / "Formula"

    for formula_name, targets in HOMEBREW_FORMULAE.items():
        formula_path = formula_dir / formula_name
        content = formula_path.read_text(encoding="utf-8")

        for target in targets:
            filename = asset_filename(release_tag, target, "tar.gz")
            digest = require_checksum(checksums, filename)
            url = github_asset_url(
                github_web_endpoint,
                github_repo,
                release_tag,
                filename,
            )
            content = replace_formula_block(
                content,
                target=target,
                new_url=url,
                new_sha256=digest,
            )

        formula_path.write_text(content, encoding="utf-8", newline="\n")


def load_manifest(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_manifest(path: Path, manifest: dict[str, object]) -> None:
    path.write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def maybe_backup_manifest(path: Path, current_version: str, release_version: str) -> str | None:
    if not current_version or current_version == release_version:
        return None

    backup_path = path.with_name(f"{path.stem}-{current_version}{path.suffix}")
    if not backup_path.exists():
        shutil.copy2(path, backup_path)
        log(f"Created Scoop backup manifest: {backup_path.name}")
    else:
        log(f"Scoop backup manifest already exists: {backup_path.name}")

    return backup_path.stem


def update_architecture_entry(
    entry: dict[str, object],
    *,
    github_web_endpoint: str,
    github_repo: str,
    release_tag: str,
    target: str,
    extension: str,
    digest: str,
) -> None:
    filename = asset_filename(release_tag, target, extension)
    entry["url"] = github_asset_url(
        github_web_endpoint,
        github_repo,
        release_tag,
        filename,
    )
    entry["hash"] = digest
    entry["extract_dir"] = asset_stem(release_tag, target)


def update_scoop_repo(
    repo_dir: Path,
    *,
    release_tag: str,
    release_version: str,
    github_web_endpoint: str,
    github_repo: str,
    checksums: dict[str, str],
) -> list[str]:
    bucket_dir = repo_dir / "bucket"
    manifest_stems_to_check = ["cnb-rs", "cnb-rs-msvc", "cnb-rs-gnu"]

    for manifest_name in ("cnb-rs.json", "cnb-rs-msvc.json", "cnb-rs-gnu.json"):
        manifest_path = bucket_dir / manifest_name
        manifest = load_manifest(manifest_path)
        current_version = str(manifest.get("version", "")).strip()
        backup_stem = maybe_backup_manifest(manifest_path, current_version, release_version)
        if backup_stem is not None:
            manifest_stems_to_check.append(backup_stem)

        manifest["version"] = release_version
        architecture = manifest.setdefault("architecture", {})
        if not isinstance(architecture, dict):
            raise UpdateError(f"{manifest_path.name} has an invalid architecture field.")

        if manifest_name in {"cnb-rs.json", "cnb-rs-msvc.json"}:
            bit64 = architecture.setdefault("64bit", {})
            arm64 = architecture.setdefault("arm64", {})
            if not isinstance(bit64, dict) or not isinstance(arm64, dict):
                raise UpdateError(f"{manifest_path.name} has an invalid MSVC architecture layout.")

            update_architecture_entry(
                bit64,
                github_web_endpoint=github_web_endpoint,
                github_repo=github_repo,
                release_tag=release_tag,
                target="x86_64-pc-windows-msvc",
                extension="zip",
                digest=require_checksum(
                    checksums,
                    asset_filename(release_tag, "x86_64-pc-windows-msvc", "zip"),
                ),
            )
            update_architecture_entry(
                arm64,
                github_web_endpoint=github_web_endpoint,
                github_repo=github_repo,
                release_tag=release_tag,
                target="aarch64-pc-windows-msvc",
                extension="zip",
                digest=require_checksum(
                    checksums,
                    asset_filename(release_tag, "aarch64-pc-windows-msvc", "zip"),
                ),
            )
        else:
            bit64 = architecture.setdefault("64bit", {})
            if not isinstance(bit64, dict):
                raise UpdateError(f"{manifest_path.name} has an invalid GNU architecture layout.")

            update_architecture_entry(
                bit64,
                github_web_endpoint=github_web_endpoint,
                github_repo=github_repo,
                release_tag=release_tag,
                target="x86_64-pc-windows-gnu",
                extension="zip",
                digest=require_checksum(
                    checksums,
                    asset_filename(release_tag, "x86_64-pc-windows-gnu", "zip"),
                ),
            )

            optional_filename = asset_filename(
                release_tag,
                "aarch64-pc-windows-gnullvm",
                "zip",
            )
            optional_digest = checksums.get(optional_filename)
            if optional_digest is None:
                architecture.pop("arm64", None)
                log(
                    "Optional asset "
                    f"{optional_filename} is missing; removed Scoop GNU arm64 entry."
                )
            else:
                arm64 = architecture.setdefault("arm64", {})
                if not isinstance(arm64, dict):
                    raise UpdateError(
                        f"{manifest_path.name} has an invalid GNU arm64 architecture layout."
                    )
                update_architecture_entry(
                    arm64,
                    github_web_endpoint=github_web_endpoint,
                    github_repo=github_repo,
                    release_tag=release_tag,
                    target="aarch64-pc-windows-gnullvm",
                    extension="zip",
                    digest=optional_digest,
                )

        write_manifest(manifest_path, manifest)

    return sorted(set(manifest_stems_to_check))


def run_homebrew_validations(repo_dir: Path) -> None:
    run_cmd([sys.executable, "bin/validate_ci.py"], cwd=repo_dir)
    for formula in ("cnb-rs", "cnb-rs-musl"):
        run_cmd([sys.executable, "bin/check.py", formula, "--url-only"], cwd=repo_dir)
    for formula in ("cnb-rs", "cnb-rs-musl"):
        run_cmd([sys.executable, "bin/check.py", formula, "--hash-only"], cwd=repo_dir)


def run_scoop_validations(repo_dir: Path, manifest_stems: list[str]) -> None:
    run_cmd([sys.executable, "bin/validate_ci.py"], cwd=repo_dir)
    for manifest_stem in manifest_stems:
        run_cmd(
            [sys.executable, "bin/check.py", manifest_stem, "--url-only"],
            cwd=repo_dir,
        )
    for manifest_stem in manifest_stems:
        run_cmd(
            [sys.executable, "bin/check.py", manifest_stem, "--hash-only"],
            cwd=repo_dir,
        )


def git_has_changes(repo_dir: Path) -> bool:
    return bool(git_output(repo_dir, "status", "--short"))


def git_identity_env() -> dict[str, str]:
    env = os.environ.copy()
    env.update(
        {
            "GIT_AUTHOR_NAME": "github-actions[bot]",
            "GIT_AUTHOR_EMAIL": "41898282+github-actions[bot]@users.noreply.github.com",
            "GIT_COMMITTER_NAME": "github-actions[bot]",
            "GIT_COMMITTER_EMAIL": "41898282+github-actions[bot]@users.noreply.github.com",
        }
    )
    return env


def commit_repo(repo_dir: Path, *, paths_to_add: list[str], message: str) -> bool:
    if not git_has_changes(repo_dir):
        log(f"No changes detected in {repo_dir}; skip commit.")
        return False

    env = git_identity_env()
    run_cmd(["git", "add", *paths_to_add], cwd=repo_dir, env=env)
    if not git_has_changes(repo_dir):
        log(f"No staged changes remained in {repo_dir}; skip commit.")
        return False

    run_cmd(["git", "commit", "-m", message], cwd=repo_dir, env=env)
    return True


def push_repo(repo_dir: Path, *, cnb_token: str) -> None:
    run_cmd(["git", "push", "origin", "main"], cwd=repo_dir, env=git_auth_env(cnb_token))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Update Homebrew and Scoop package repositories for a release.",
    )
    parser.add_argument(
        "--release-tag",
        help="Published stable tag, e.g. v0.11.1. Omit to use the latest stable GitHub release.",
    )
    parser.add_argument("--work-root", type=Path, help="Directory used for package repo clones")
    parser.add_argument("--homebrew-repo-dir", type=Path, help="Existing Homebrew repo checkout")
    parser.add_argument("--scoop-repo-dir", type=Path, help="Existing Scoop repo checkout")
    parser.add_argument("--github-repo", default=os.environ.get("GITHUB_REPOSITORY", GITHUB_REPO_DEFAULT))
    parser.add_argument(
        "--github-web-endpoint",
        default=os.environ.get("GITHUB_SERVER_URL", GITHUB_WEB_ENDPOINT_DEFAULT),
    )
    parser.add_argument(
        "--cnb-web-endpoint",
        default=os.environ.get("CNB_WEB_ENDPOINT", CNB_WEB_ENDPOINT_DEFAULT),
    )
    parser.add_argument(
        "--homebrew-repo-slug",
        default=HOMEBREW_REPO_SLUG_DEFAULT,
    )
    parser.add_argument(
        "--scoop-repo-slug",
        default=SCOOP_REPO_SLUG_DEFAULT,
    )
    parser.add_argument(
        "--push",
        action="store_true",
        help="Commit and push package repo updates to origin/main.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    github_web_endpoint = normalize_base_url(args.github_web_endpoint)
    cnb_web_endpoint = normalize_base_url(args.cnb_web_endpoint)
    release_tag = args.release_tag or resolve_latest_release_tag(
        github_web_endpoint,
        args.github_repo,
    )
    log(f"Using release tag {release_tag}")

    if not RELEASE_TAG_PATTERN.fullmatch(release_tag):
        raise UpdateError(
            "Release tag must match vX.Y.Z, vX.Y.Z-alpha.N, or vX.Y.Z-beta.N."
        )
    if "-alpha" in release_tag or "-beta" in release_tag:
        raise UpdateError(
            f"Package repo automation only supports stable releases; got prerelease tag {release_tag}."
        )

    release_version = release_tag.removeprefix("v")
    cnb_token = os.environ.get("CNB_TOKEN")

    work_root = args.work_root
    if work_root is None and (args.homebrew_repo_dir is None or args.scoop_repo_dir is None):
        work_root = Path(tempfile.mkdtemp(prefix="cnb-rs-package-repos-"))
        log(f"Created temporary work root: {work_root}")
    elif work_root is not None:
        work_root.mkdir(parents=True, exist_ok=True)

    sha256sum_url = github_asset_url(
        github_web_endpoint,
        args.github_repo,
        release_tag,
        "sha256sum.txt",
    )
    log(f"Fetch release checksums from {sha256sum_url}")
    checksums = parse_checksums(fetch_text(sha256sum_url))

    homebrew_repo_dir = prepare_repo(
        repo_dir=args.homebrew_repo_dir,
        repo_slug=args.homebrew_repo_slug,
        cnb_web_endpoint=cnb_web_endpoint,
        cnb_token=cnb_token,
        push=args.push,
        work_root=work_root,
    )
    scoop_repo_dir = prepare_repo(
        repo_dir=args.scoop_repo_dir,
        repo_slug=args.scoop_repo_slug,
        cnb_web_endpoint=cnb_web_endpoint,
        cnb_token=cnb_token,
        push=args.push,
        work_root=work_root,
    )

    log("Update Homebrew formulae")
    update_homebrew_repo(
        homebrew_repo_dir,
        release_tag=release_tag,
        github_web_endpoint=github_web_endpoint,
        github_repo=args.github_repo,
        checksums=checksums,
    )
    run_homebrew_validations(homebrew_repo_dir)

    log("Update Scoop manifests")
    scoop_manifest_stems = update_scoop_repo(
        scoop_repo_dir,
        release_tag=release_tag,
        release_version=release_version,
        github_web_endpoint=github_web_endpoint,
        github_repo=args.github_repo,
        checksums=checksums,
    )
    run_scoop_validations(scoop_repo_dir, scoop_manifest_stems)

    if not args.push:
        log("Dry run finished without push.")
        log(f"Homebrew repo dir: {homebrew_repo_dir}")
        log(f"Scoop repo dir: {scoop_repo_dir}")
        return 0

    push_cnb_token = require_cnb_token(cnb_token)

    homebrew_committed = commit_repo(
        homebrew_repo_dir,
        paths_to_add=["Formula"],
        message=f"chore(release): update formulae for {release_tag}",
    )
    scoop_committed = commit_repo(
        scoop_repo_dir,
        paths_to_add=["bucket"],
        message=f"chore(release): update manifests for {release_tag}",
    )

    if homebrew_committed:
        push_repo(homebrew_repo_dir, cnb_token=push_cnb_token)
    else:
        log("Homebrew repo already up to date; skip push.")

    if scoop_committed:
        push_repo(scoop_repo_dir, cnb_token=push_cnb_token)
    else:
        log("Scoop repo already up to date; skip push.")

    log("Package repository update finished.")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except UpdateError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
