use std::{fs, fs::remove_dir_all, io, io::Write, path::Path};

use aok::{OK, Result};
use pb_jelly_gen::GenProtos;
use tempfile::tempdir;
use walkdir::WalkDir;

fn insert_before_first_brace(input: &str, insert: &str) -> String {
  if let Some(pos) = input.find(';') {
    let pos = pos + 1;
    let mut result = String::new();
    result.push_str(&input[..pos]); // 插入 '{' 所在行之前的内容
    result.push_str(insert); // 插入 'xxx'
    result.push_str(&input[pos..]); // 插入 '{' 所在行及之后的内容

    result
  } else {
    // 如果没有 '{'，返回原字符串
    input.to_string()
  }
}

fn rmdir<P: AsRef<std::path::Path>>(path: P) -> io::Result<()> {
  match remove_dir_all(&path) {
    Ok(()) => Ok(()),
    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()), // 忽略目录不存在的错误
    Err(e) => Err(e),                                        // 其他错误返回
  }
}

const PROTO: &str = "proto";

pub fn run(workdir: impl AsRef<Path>) -> Result<()> {
  let workdir = workdir.as_ref();

  let dir_rust = workdir.join("rust");
  rmdir(&dir_rust)?;

  let dir_proto = workdir.join("proto");
  if !dir_proto.exists() {
    return OK;
  }
  let dir_tmp = tempdir()?;
  let dir_tmp = dir_tmp.path();

  let mut proto_cargo_toml = ifs::w(workdir.join("pb/Cargo.toml"))?;
  let mut proto_cargo_lib = ifs::w(workdir.join("pb/src/lib.rs"))?;

  let _ = proto_cargo_toml.write(
    br#"[package]
name = "pb"
version = "0.1.0"
edition = "2024"

[workspace]
members = []

[dependencies]
"#,
  )?;

  for fp in WalkDir::new(&dir_proto)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|file| file.path().extension().unwrap_or_default() == PROTO)
  {
    let fp = fp.path();
    let proto = fs::read_to_string(fp)?;
    if let Ok(rel_fp) = fp.strip_prefix(&dir_proto) {
      let rel_fp = rel_fp.to_str().unwrap();
      let crate_name = &rel_fp.as_bytes()[..rel_fp.len() - 1 - PROTO.len()];

      if crate_name == b"_" {
        let _ = proto_cargo_lib.write(b"pub use proto__::proto__::*;\n");
      } else {
        for i in [
          b"pub mod ",
          crate_name,
          b" {\n  #[allow(unused_imports)]\n  pub use proto_",
          crate_name,
          b"::proto_",
          crate_name,
          b"::*",
          b";\n}\n",
        ] {
          let _ = proto_cargo_lib.write(i)?;
        }
      }

      for i in [
        b"proto_",
        crate_name,
        b" = { path = \"../rust/proto_",
        crate_name,
        b"\" }",
        b"\n",
      ] {
        let _ = proto_cargo_toml.write(i)?;
      }

      let mut li = vec![];
      let mut use_ext = false;
      for i in proto.split('\n') {
        let i = i.trim();
        li.push(i);
        if i.starts_with("enum ") {
          use_ext = true;
          li.push("option (rust.closed_enum) = true;");
        }
      }
      let mut proto = li.join("\n");
      if use_ext {
        proto = insert_before_first_brace(&proto, "import \"rust/extensions.proto\";\n");
      }
      ifs::w(dir_tmp.join(rel_fp))?.write_all(proto.as_bytes())?;
    }
  }

  unsafe { std::env::set_var("CARGO_MANIFEST_DIR", workdir) };

  if let Err(e) = GenProtos::builder()
    .out_path(&dir_rust)
    .src_path(dir_tmp)
    .cleanup_out_path(true)
    .gen_protos()
  {
    eprintln!("{}", e);
    std::process::exit(1);
  }

  let fp = dir_rust.join("proto__/src/proto__.rs");
  let code = ifs::rstr(&fp)?;
  ifs::w(fp)?.write_all(code.replace("i32", "u32").as_bytes())?;
  rmdir(dir_tmp)?;

  OK
}
