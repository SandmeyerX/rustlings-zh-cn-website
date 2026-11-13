mod builder;
mod exercise;
use builder::build;
use exercise::Info;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static RUSTLINGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var_os("RUSTLINGS_DIR")
        .unwrap_or("../rustlings-zh-cn/".into())
        .into()
});
const INFO_FILE: &str = "info.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info_file = RUSTLINGS_DIR.join(INFO_FILE);

    let exercises = Info::parse_file(info_file)?.exercises;
    build(&exercises).unwrap();

    Ok(())
}
