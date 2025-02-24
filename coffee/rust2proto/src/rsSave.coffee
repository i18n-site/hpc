#!/usr/bin/env coffee

> @3-/write
  path > join

< (dir_gen, rs_run)=>
  write(
    join dir_gen, '_hpc.rs'
    """
//! AUTO GEN BY rust2proto , DO NOT EDIT

use aok::{anyhow, Result};
use dstr::dvec;
use hpc::CallErr;
use hpc_captcha::{Captcha, GenCaptcha, call_err};
use pb::Func;
use pb_jelly::Message;
use r#mod::*;

pub struct Hpc;

impl hpc::Hpc for Hpc {

  type Func = Func;

  async fn run<G: GenCaptcha>(
    req: &req_::Req, func: Func, args: &[u8], captcha: &Captcha<G>
  ) -> Result<Vec<u8>> {
    Ok(match func {

Func::None => return Ok(vec![]),
#{rs_run}

    })
  }

}
"""
  )
  return
