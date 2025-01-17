#!/usr/bin/env coffee

export default (func_li)=>
  """use aok::{anyhow, Result};
use hpc::Call;
use pb_jelly::Message;

pub struct Hpc;

impl hpc::Hpc for Hpc {
  async fn run(call: &Call) -> Result<Vec<u8>> {
    let func = call.func;
    let args = &call.args;
    Ok(match func {
      1 => {
        let r = captcha::captcha().await?;
        pb::captcha::Captcha {
          id: r.id.into(),
          img: r.img.into(),
          tip: r.tip.into(),
        }
        .serialize_to_vec()
      }
      2 => {
        let args = pb::auth::SignupMailArgs::deserialize_from_slice(args)?;
        auth::signup_mail(&args.address, &args.password).await?;
        vec![]
      }
      _ => return Err(anyhow!("call.func {func} no match")),
    })
  }
}"""
