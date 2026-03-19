//! 构建脚本：扫描 references/ 目录，生成嵌入式文档映射

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("embedded_refs.rs");
    let mut out = fs::File::create(&dest_path)?;

    let refs_dir = Path::new("references");
    println!("cargo:rerun-if-changed=references");

    // 收集所有 .md 文件
    let mut entries: Vec<(String, String)> = Vec::new();
    if refs_dir.is_dir() {
        collect_md_files(refs_dir, refs_dir, &mut entries)?;
    }

    // 生成 phf 风格的 match 函数
    writeln!(out, "/// 根据 `service/apiname` 路径获取嵌入的 API 文档")?;
    writeln!(out, "#[must_use]")?;
    writeln!(out, "#[allow(clippy::too_many_lines)]")?;
    writeln!(
        out,
        "pub fn get_embedded_doc(key: &str) -> Option<&'static str> {{"
    )?;
    writeln!(out, "    match key {{")?;

    for (key, rel_path) in &entries {
        // 使用 include_str! 嵌入文件内容
        let abs_path = refs_dir.join(rel_path);
        let abs_str = abs_path
            .canonicalize()?
            .to_string_lossy()
            .replace('\\', "/");
        writeln!(
            out,
            "        \"{key}\" => Some(include_str!(\"{abs_str}\")),"
        )?;
    }

    writeln!(out, "        _ => None,")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    // 生成列出所有服务名的函数
    let mut services: Vec<String> = entries
        .iter()
        .filter_map(|(key, _)| key.split('/').next().map(std::string::ToString::to_string))
        .collect();
    services.sort();
    services.dedup();

    writeln!(out)?;
    writeln!(out, "/// 列出所有 API 服务分类")?;
    writeln!(out, "#[must_use]")?;
    writeln!(
        out,
        "pub fn list_embedded_services() -> &'static [&'static str] {{"
    )?;
    write!(out, "    &[")?;
    for s in &services {
        write!(out, "\"{s}\", ")?;
    }
    writeln!(out, "]")?;
    writeln!(out, "}}")?;

    // 生成列出所有 key 的函数
    writeln!(out)?;
    writeln!(out, "/// 列出所有嵌入的文档 key")?;
    writeln!(out, "#[must_use]")?;
    writeln!(
        out,
        "pub fn list_embedded_keys() -> &'static [&'static str] {{"
    )?;
    write!(out, "    &[")?;
    for (key, _) in &entries {
        write!(out, "\"{key}\", ")?;
    }
    writeln!(out, "]")?;
    writeln!(out, "}}")?;

    Ok(())
}

/// 递归收集 .md 文件，key 格式为 "service/apiname"（无 .md 后缀）
fn collect_md_files(
    base: &Path,
    dir: &Path,
    entries: &mut Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut items: Vec<_> = fs::read_dir(dir)?.filter_map(Result::ok).collect();
    items.sort_by_key(std::fs::DirEntry::file_name);

    for entry in items {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(base, &path, entries)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            let rel = path.strip_prefix(base)?;
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            let key = rel_str.strip_suffix(".md").unwrap_or(&rel_str);
            entries.push((key.to_string(), rel_str.clone()));
        }
    }

    Ok(())
}
