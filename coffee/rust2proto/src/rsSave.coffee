#!/usr/bin/env coffee

> @3-/write
  path > join

< (dir_gen, rs_run)=>
  write(
    join dir_gen, '_hpc.rs'
    """
//! AUTO GEN BY rust2proto , DO NOT EDIT

use aok::{anyhow, Result};
use hpc::CallErr;
use pb::Func;
use dstr::dvec;
use pb_jelly::Message;
use r#mod::*;

pub struct Hpc;

impl hpc::Hpc for Hpc {

  type Func = Func;

  async fn run(req: &req_::Req, func: Func, args: &[u8]) -> Result<Vec<u8>> {
    Ok(match func {

Func::None => return Ok(vec![]),
#{rs_run}

    })
  }

}
"""
  )
  return
