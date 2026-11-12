use crate::RUSTLINGS_DIR;
use anyhow::{Error, Result, bail};
use serde::Deserialize;
use std::path::Path;

const EXERCISES_DIR: &str = "exercises/";
const SOLUTIONS_DIR: &str = "solutions/";

#[derive(Debug, Deserialize)]
pub struct RawInfo {
    // [[exercises]]
    #[serde(default)]
    exercises: Vec<RawExercise>,
}

#[derive(Debug)]
pub struct Info {
    pub exercises: Vec<Exercise>,
}

impl TryFrom<RawInfo> for Info {
    type Error = Error;

    fn try_from(value: RawInfo) -> Result<Self, Self::Error> {
        let exercises = value
            .exercises
            .into_iter()
            .map(Exercise::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self { exercises })
    }
}

impl Info {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let toml = std::fs::read_to_string(path)?;
        let raw: RawInfo = toml::from_str(&toml)?;

        raw.try_into()
    }
}

#[derive(Debug, Deserialize)]
struct RawExercise {
    /// 练习文件名（不含 .rs）
    name: String,

    #[serde(default)]
    dir: Option<String>,

    #[serde(default = "default_true")]
    test: bool,

    #[serde(default)]
    strict_clippy: bool,

    #[serde(default)]
    hint: Option<String>,

    #[serde(default)]
    skip_check_unsolved: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug)]
pub struct Exercise {
    /// 练习名
    /// 即 没有 `.rs` 后缀的练习文件名
    pub name: String,

    /// 题目内容
    pub content: String,

    /// 答案内容
    pub solution: String,

    /// 提示内容
    pub hint: Option<String>,

    /// Rustlings 期望练习中包含测试并运行它们。
    ///
    /// 你可以通过将 `test` 设置为 `false` 来选择性地禁用测试（默认值为 `true`）。
    /// 在这种情况下，只要练习成功编译，就会被视为完成。
    pub need_test: bool,

    /// Rustlings 始终会对练习运行 Clippy。
    ///
    /// 你可以选择将 `strict_clippy` 设置为 `true`（默认值为 `false`），以便仅在没有剩余警告时
    /// 才将练习视为完成。
    pub strict_clippy: bool,

    /// Rustlings 会检查未解决的练习。
    ///
    /// 你可以选择将 `skip_check_unsolved` 设置为 `true`（默认值为 `false`），以跳过对未解决练习的检查。
    pub need_check: bool,

    /// 练习分类
    ///
    /// 这决定了将练习归于侧边栏中的哪一组
    ///
    /// # NOTE
    /// 如果 [`RawExercise`] 的字段 `dir` 为 `Some`，则值为 `dir`；否则单独为一组，值为 `name`
    pub group: String,
}

impl TryFrom<RawExercise> for Exercise {
    type Error = Error;

    fn try_from(value: RawExercise) -> Result<Self, Self::Error> {
        let name = value.name.trim().to_string();
        if name.is_empty() {
            bail!("name shouldn't be empty!");
        }

        let root = Path::new(RUSTLINGS_DIR);
        let exercises_dir = root.join(EXERCISES_DIR);
        let solutions_dir = root.join(SOLUTIONS_DIR);

        let filename = Path::new(&name).with_added_extension("rs");
        let exercise_file = match value.dir {
            Some(ref p) => exercises_dir.join(p).join(&filename),
            None => exercises_dir.join(&filename),
        };
        let solution_file = match value.dir {
            Some(ref p) => solutions_dir.join(p).join(&filename),
            None => solutions_dir.join(&filename),
        };
        let content = std::fs::read_to_string(exercise_file)?;
        let solution = std::fs::read_to_string(solution_file)?;

        let group = match value.dir {
            Some(dir) => dir.trim().to_string(),
            None => name.clone(),
        };

        Ok(Self {
            name,
            content,
            solution,
            group,
            hint: value.hint,
            need_test: value.test,
            strict_clippy: value.strict_clippy,
            need_check: !value.skip_check_unsolved,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_raw_info() {
        let toml_text = r#"
[[exercises]]
name = "intro1"
dir = "00_intro"
test = false
hint = """
在终端键入 `n` 进入下一个练习。
键入 `n` 后可能还需要回车。"""
skip_check_unsolved = true

[[exercises]]
name = "intro2"
dir = "00_intro"
test = false
hint = """
提供打印功能的宏是 `printline!` 吗？看看编译器提示了什么。"""
"#;

        let info: RawInfo = toml::from_str(toml_text).unwrap();
        assert_eq!(info.exercises.len(), 2);

        let e0 = &info.exercises[0];
        assert_eq!(e0.name, "intro1");
        assert_eq!(e0.dir.as_deref(), Some("00_intro"));
        assert!(!e0.test);
        assert!(e0.skip_check_unsolved);

        let e1 = &info.exercises[1];
        assert!(e1.strict_clippy == false);

        assert!(e0.hint.as_ref().unwrap().starts_with("在终端键入"));
    }

    #[test]
    fn test_ex_from_raw() {
        let raw = RawExercise {
            name: "intro1".into(),
            dir: Some("00_intro".into()),
            test: false,
            strict_clippy: false,
            hint: Some("这是一个提示".into()),
            skip_check_unsolved: true,
        };
        let ex = Exercise::try_from(raw).unwrap();
        assert_eq!(ex.group, "00_intro");
        assert!(!ex.content.is_empty());
        assert!(!ex.solution.is_empty());
        assert!(!ex.need_test);
        assert!(!ex.need_check);
        assert!(!ex.strict_clippy);
    }
}
