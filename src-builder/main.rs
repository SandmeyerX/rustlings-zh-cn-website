mod builder;
mod exercise;
use builder::build;
use exercise::Info;
use std::path::Path;

pub const RUSTLINGS_DIR: &str = "../rustlings-zh-cn/";
const INFO_FILE: &str = "info.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustlings_dir = Path::new(RUSTLINGS_DIR);
    let info_file = rustlings_dir.join(INFO_FILE);

    let exercises = Info::parse_file(info_file)?.exercises;
    build(&exercises).unwrap();

    Ok(())
}
