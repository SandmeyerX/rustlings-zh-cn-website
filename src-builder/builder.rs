use crate::RUSTLINGS_DIR;
use crate::exercise::Exercise;
use anyhow::Result;
use itertools::Itertools;
use std::{fs::File, io::Write, path::Path};

const BUILD_DIR: &str = "build/";
const EXERCISE_TEMPLATE: &str = "src-builder/ex_template.mdx";
const TEMPLATE: &str = "src-builder/template.mdx";

pub fn build(exercises: &[Exercise]) -> Result<()> {
    let build_dir = Path::new(BUILD_DIR);
    std::fs::create_dir_all(build_dir)?;

    let grouped_exs = exercises.iter().into_group_map_by(|ex| ex.group.clone());

    for (group, exs) in grouped_exs {
        let mut group_template = String::new();
        for ex in exs {
            let ex_template = build_exercise_template(ex);
            group_template.push('\n');
            group_template.push_str(&ex_template);
        }

        let filename = Path::new(&group).with_added_extension("mdx");
        let mut mdx = File::create(build_dir.join(filename))?;

        // HACK: 不应该假设 `group` 就是练习所在 dir
        // 重构成更好的写法
        let readme_path = Path::new(RUSTLINGS_DIR)
            .join("exercises")
            .join(&group)
            .join("README.md");
        let readme = clean_markdown(&std::fs::read_to_string(readme_path)?);
        // END ---

        let template = build_template(&group, &group, &readme, &group_template);
        mdx.write_all(&template.into_bytes())?;
    }

    Ok(())
}

/// 将每级标题都降一级
fn clean_markdown(md: &str) -> String {
    let mut out = String::with_capacity(md.len() + 16);
    for line in md.lines() {
        let t = line.trim_start();
        if t.starts_with('#') {
            // HACK: 没有正确处理第 6 级标题
            // 标准 Markdown（CommonMark）最多 6 级标题
            out.push('#');
            out.push_str(t);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    // 去除末尾一个换行
    out.pop();

    out
}

fn build_template(title: &str, desc: &str, readme: &str, exs_temp: &str) -> String {
    let template = std::fs::read_to_string(TEMPLATE).unwrap();
    template
        .replace("%{{TITLE}}%", title)
        .replace("%{{DESCRIPTION}}%", desc)
        .replace("%{{README}}%", readme)
        .replace("%{{EXERCISES}}%", exs_temp)
}

fn build_exercise_template(ex: &Exercise) -> String {
    let template = std::fs::read_to_string(EXERCISE_TEMPLATE).unwrap();
    let hint = ex.hint.clone().unwrap_or("暂无提示".into());

    template
        .replace("%{{NAME}}%", &ex.name)
        .replace("%{{TEST_LABEL}}%", {
            if ex.need_test {
                "需要测试"
            } else {
                "无需测试"
            }
        })
        .replace("%{{CHECK_LABEL}}%", {
            if ex.need_check {
                "需要检查"
            } else {
                "无需检查"
            }
        })
        .replace("%{{CLIPPY_LABEL}}%", {
            if ex.strict_clippy {
                "严厉 clippy"
            } else {
                "友好 clippy"
            }
        })
        .replace("%{{CONTENT}}%", &ex.content)
        .replace("%{{SOLUTION}}%", &ex.solution)
        .replace("%{{HINT}}%", &hint)
        .replace(
            "%{{EXERCISE_FILE_PATH}}%",
            &build_code_block_title(&ex.path),
        )
        .replace(
            "%{{SOLUTION_FILE_PATH}}%",
            &build_code_block_title(&ex.solution_path),
        )
}

/// 构建代码块标题
///
/// 去除代码文件路径中的 [`RUSTLINGS_DIR`] 前缀，并统一分隔符 `\` 为 `/`
fn build_code_block_title(path: &Path) -> String {
    path.strip_prefix(RUSTLINGS_DIR)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
