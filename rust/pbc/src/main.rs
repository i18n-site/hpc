use std::path::PathBuf;

use aok::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pbc", about = "protobuf to rust crate")]
struct Args {
  /// 工作目录路径
  #[arg(short = 'w', long = "workdir", default_value = ".")]
  workdir: PathBuf, // 使用 PathBuf 类型
}

fn main() -> Result<()> {
  let args = Args::parse();
  let workdir = args.workdir;
  pbc::run(workdir)
}
