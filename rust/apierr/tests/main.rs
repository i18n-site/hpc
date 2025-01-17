use aok::Result;
use num_derive::ToPrimitive;

#[derive(ToPrimitive)]
pub enum TestErr {
  Xxx = 1,
}

#[test]
fn main() -> Result<()> {
  // let mut err_code_li = apierr::code_li();
  //
  // err_code_li += TestErr::Xxx;
  //
  // err_code_li += 33;
  // dbg!(&err_code_li);
  // err_code_li?;
  // let mut code_msg_li = apierr::code_msg_li();
  //
  // code_msg_li += (1, "msg1");
  // dbg!(&code_msg_li);
  //
  // code_msg_li?;

  // let err: Result<()> = Err(ApiErr::User.into());
  // if let Err(err) = err {
  //   if let Ok(err) = err.downcast::<ApiErr>() {
  //     dbg!(err);
  //   }
  // }

  Ok(())
}
