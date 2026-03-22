FROM rust:latest

ARG GIT_CLIFF_VERSION=2.12.0

# 配置 rustup 使用 tuna 镜像源（加速下载工具链组件）
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

# 预装 CI 检查所需组件
RUN rustup component add rustfmt clippy
RUN cargo install cargo-deny --locked

# release-prepare 与 tag release 仍在 CNB 侧使用 git-cliff
RUN set -eux; \
    tmpdir="$(mktemp -d)"; \
    archive="${tmpdir}/git-cliff.tar.gz"; \
    curl -sSfL "https://github.com/orhun/git-cliff/releases/download/v${GIT_CLIFF_VERSION}/git-cliff-${GIT_CLIFF_VERSION}-x86_64-unknown-linux-gnu.tar.gz" -o "${archive}"; \
    tar -xzf "${archive}" -C "${tmpdir}"; \
    binary="$(find "${tmpdir}" -type f -name git-cliff | head -n 1)"; \
    test -n "${binary}"; \
    install "${binary}" /usr/local/bin/git-cliff; \
    rm -rf "${tmpdir}"

# 配置 cargo 镜像源（国内加速）
RUN printf '\
[source.crates-io]\n\
replace-with = "mirror"\n\
\n\
[source.mirror]\n\
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"\n\
\n\
[registries.mirror]\n\
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"\n\
\n\
[net]\n\
git-fetch-with-cli = true\n\
' >> ${CARGO_HOME}/config.toml

# 使用固定的 target 目录，CI 构建时复用预热后的 Linux 依赖
ENV CARGO_TARGET_DIR=/cargo-target

WORKDIR /tmp/deps
COPY Cargo.toml Cargo.lock build.rs ./
COPY crates/cnb-api/Cargo.toml crates/cnb-api/
COPY crates/cnb-core/Cargo.toml crates/cnb-core/
COPY crates/cnb-tui/Cargo.toml crates/cnb-tui/
COPY crates/cnb-chat/Cargo.toml crates/cnb-chat/build.rs crates/cnb-chat/

# 创建空源文件和必要目录（仅用于依赖解析和预热）
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs \
    && mkdir -p crates/cnb-api/src && touch crates/cnb-api/src/lib.rs \
    && mkdir -p crates/cnb-core/src && touch crates/cnb-core/src/lib.rs \
    && mkdir -p crates/cnb-tui/src && touch crates/cnb-tui/src/lib.rs \
    && mkdir -p crates/cnb-chat/src && touch crates/cnb-chat/src/lib.rs \
    && mkdir -p crates/cnb-chat/references

RUN cargo check --workspace --all-targets --target x86_64-unknown-linux-gnu \
    && cargo test --workspace --no-run --target x86_64-unknown-linux-gnu

# 清理工作区 crate 的编译产物，保留外部依赖缓存
RUN find /cargo-target -type d \( \
      -name "cnb-*" -o -name "cnb_*" -o -name "git-cnb-*" \
    \) -exec rm -rf {} + 2>/dev/null; \
    find /cargo-target -type f \( \
      -name "cnb-*" -o -name "cnb_*" -o -name "libcnb*" -o -name "git-cnb*" \
    \) -delete 2>/dev/null; \
    true

# 清理临时项目文件
RUN rm -rf /tmp/deps
WORKDIR /
