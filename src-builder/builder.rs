use crate::RUSTLINGS_DIR;
use crate::exercise::Exercise;
use anyhow::{Context, Result, bail};
use itertools::Itertools;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::LazyLock,
};

static BUILD_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var_os("BUILD_DIR")
        .unwrap_or("build/".into())
        .into()
});
const EXERCISE_TEMPLATE: &str = "src-builder/ex_template.mdx";
const TEMPLATE: &str = "src-builder/template.mdx";

pub fn build(exercises: &[Exercise]) -> Result<()> {
    std::fs::create_dir_all(BUILD_DIR.as_path())
        .with_context(|| format!("creating output directory {}", BUILD_DIR.display()))?;

    let grouped_exs = exercises.iter().into_group_map_by(|ex| ex.group.clone());

    for (group, exs) in grouped_exs {
        let mut group_template = String::new();
        for ex in &exs {
            let ex_template = build_exercise_template(ex)?;
            group_template.push('\n');
            group_template.push_str(&ex_template);
        }

        let filename = Path::new(&group).with_added_extension("mdx");
        let output_path = BUILD_DIR.join(filename);

        // `group` is only a presentation key. We should never infer README path from it.
        // Use the README explicitly resolved from the exercise metadata instead.
        let readme_path = group_readme_path(&group, &exs)?;
        let readme = match readme_path {
            Some(path) => {
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("reading group README {}", path.display()))?;
                clean_markdown(&content)
            }
            None => String::new(),
        };

        let template = build_template(&group, &group, &readme, &group_template)?;
        let mut mdx = File::create(&output_path)
            .with_context(|| format!("creating generated page {}", output_path.display()))?;
        mdx.write_all(template.as_bytes())
            .with_context(|| format!("writing generated page {}", output_path.display()))?;
    }

    Ok(())
}

/// Return the README path associated with a group, rejecting inconsistent metadata.
fn group_readme_path<'a>(group: &str, exercises: &[&'a Exercise]) -> Result<Option<&'a Path>> {
    let Some(first) = exercises.first() else {
        return Ok(None);
    };

    let path = first.readme_path.as_deref();
    if exercises
        .iter()
        .any(|exercise| exercise.readme_path.as_deref() != path)
    {
        bail!("exercises in group {group} refer to different README paths");
    }

    Ok(path)
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

fn build_template(title: &str, desc: &str, readme: &str, exs_temp: &str) -> Result<String> {
    let template = std::fs::read_to_string(TEMPLATE)
        .with_context(|| format!("reading page template {TEMPLATE}"))?;
    Ok(template
        .replace("%{{TITLE}}%", title)
        .replace("%{{DESCRIPTION}}%", desc)
        .replace("%{{README}}%", readme)
        .replace("%{{EXERCISES}}%", exs_temp))
}

fn build_exercise_template(ex: &Exercise) -> Result<String> {
    let template = std::fs::read_to_string(EXERCISE_TEMPLATE)
        .with_context(|| format!("reading exercise template {EXERCISE_TEMPLATE}"))?;
    let hint = ex.hint.as_deref().unwrap_or("暂无提示");

    Ok(template
        .replace("%{{NAME}}%", &ex.name)
        .replace(
            "%{{TEST_LABEL}}%",
            if ex.need_test {
                "需要测试"
            } else {
                "无需测试"
            },
        )
        .replace(
            "%{{CHECK_LABEL}}%",
            if ex.need_check {
                "需要检查"
            } else {
                "无需检查"
            },
        )
        .replace(
            "%{{CLIPPY_LABEL}}%",
            if ex.strict_clippy {
                "严厉 clippy"
            } else {
                "友好 clippy"
            },
        )
        .replace("%{{CONTENT}}%", &ex.content)
        .replace("%{{SOLUTION}}%", &ex.solution)
        .replace("%{{HINT}}%", hint)
        .replace(
            "%{{EXERCISE_FILE_PATH}}%",
            &build_code_block_title(&ex.path),
        )
        .replace(
            "%{{SOLUTION_FILE_PATH}}%",
            &build_code_block_title(&ex.solution_path),
        ))
}

/// 构建代码块标题
///
/// 去除代码文件路径中的 [`RUSTLINGS_DIR`] 前缀，并统一分隔符 `\` 为 `/`
fn build_code_block_title(path: &Path) -> String {
    path.strip_prefix(RUSTLINGS_DIR.as_path())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn exercise_with_readme(readme_path: Option<PathBuf>) -> Exercise {
        Exercise {
            name: "example".into(),
            path: PathBuf::from("exercises/example.rs"),
            solution_path: PathBuf::from("solutions/example.rs"),
            group: "example".into(),
            readme_path,
            content: String::new(),
            solution: String::new(),
            hint: None,
            need_test: true,
            strict_clippy: false,
            need_check: true,
        }
    }

    #[test]
    fn group_without_directory_has_no_readme() {
        let exercise = exercise_with_readme(None);
        assert_eq!(group_readme_path("example", &[&exercise]).unwrap(), None);
    }

    #[test]
    fn group_uses_explicit_readme_path_not_group_name() {
        let path = PathBuf::from("exercises/source-dir/README.md");
        let exercise = exercise_with_readme(Some(path.clone()));
        assert_eq!(
            group_readme_path("display-group", &[&exercise]).unwrap(),
            Some(path.as_path())
        );
    }

    #[test]
    fn group_rejects_conflicting_readme_paths() {
        let first = exercise_with_readme(Some(PathBuf::from("one/README.md")));
        let second = exercise_with_readme(Some(PathBuf::from("two/README.md")));
        assert!(group_readme_path("group", &[&first, &second]).is_err());
    }
}
