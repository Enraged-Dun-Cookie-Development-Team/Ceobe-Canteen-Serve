use std::{fs, path::Path};

use crate::payloads::ErrorCfg;

/// 为每个 `[[kind]]` 生成一个 Wiki 页面，加上索引页和侧边栏
pub fn generate_wiki_markdown(
    config_path: impl AsRef<Path>, output_dir: impl AsRef<Path>,
) -> std::io::Result<()> {
    let cfg = fs::read_to_string(config_path.as_ref())
        .expect("Error Config File Not Exist");
    let err_cfg: ErrorCfg =
        toml::from_str(&cfg).expect("Invalid Config File Format");

    let out = output_dir.as_ref();
    fs::create_dir_all(out)?;

    let mut sidebar_lines = Vec::new();
    let mut index_rows = Vec::new();

    for kind in &err_cfg.kind {
        let page_name = format!("Error-{}", kind.ident);
        let page_file = format!("{}.md", page_name);

        // --- 生成分类页面 ---
        let mut page = String::new();
        page.push_str(&format!("# {} ({})\n\n", kind.description, kind.mark));
        page.push_str(&format!(
            "- 默认 HTTP 状态码: {}\n\n",
            kind.default_status_code.as_u16()
        ));

        page.push_str("| 错误码 | 错误名称 | 描述 | HTTP 状态码 |\n");
        page.push_str("|--------|----------|------|-------------|\n");

        for (i, err) in kind.error.iter().enumerate() {
            let code = (i as u16) + 1;
            let err_code = format!("{}{:04x}", kind.mark, code);
            let http_code =
                err.http_code.unwrap_or(kind.default_status_code).as_u16();

            page.push_str(&format!(
                "| `{}` | `{}` | {} | {} |\n",
                err_code, err.ident, err.description, http_code
            ));

            // 收集索引行
            index_rows.push(format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                err_code, err.ident, err.description, http_code, kind.ident
            ));
        }

        page.push('\n');
        fs::write(out.join(&page_file), page)?;

        sidebar_lines
            .push(format!("- [{}]({})", kind.description, page_file));
    }

    // --- 生成索引页 ---
    let mut index = String::new();
    index.push_str("# 错误码索引\n\n");
    index.push_str("所有错误码汇总表：\n\n");
    index.push_str("| 错误码 | 错误名称 | 描述 | HTTP 状态码 | 所属分类 |\n");
    index.push_str("|--------|----------|------|-------------|----------|\n");
    for row in &index_rows {
        index.push_str(row);
    }
    index.push('\n');
    fs::write(out.join("Error-Codes-Index.md"), index)?;

    // --- 生成侧边栏 ---
    let mut sidebar = String::from("# 错误码文档\n\n");
    sidebar.push_str("## 分类\n\n");
    sidebar.push_str("- [错误码索引](Error-Codes-Index.md)\n");
    for line in &sidebar_lines {
        sidebar.push_str(line);
        sidebar.push('\n');
    }
    fs::write(out.join("_Sidebar.md"), sidebar)?;

    Ok(())
}
