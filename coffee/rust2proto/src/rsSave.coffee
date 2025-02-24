#!/usr/bin/env coffee

> @3-/write
  path > join

< (dir_gen, rs_run)=>
  write(
    join dir_gen, '_hpc.rs'
    """
//! AUTO GEN BY rust2proto , DO NOT EDIT

use aok::anyhow;
use dstr::dvec;
use hpc::{call_err, args_invalid};
use icall::{CodeBody,State};
use hpc_captcha::{Captcha, GenCaptcha};
use pb::Func;
use pb_jelly::Message;
use r#mod::*;

pub struct Hpc;

impl hpc::Hpc for Hpc {

  type Func = Func;

  async fn run<G: GenCaptcha>(
    req: &req_::Req, func: Func, args: &[u8], captcha: &Captcha<G>
  ) -> CodeBody {
    match func {

Func::None => return (State::OK, vec![]),
#{rs_run}

    }
  }

}
"""
  )
  return
