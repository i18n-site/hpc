use icall::{CodeBody, State};

mod batch;
pub(crate) use batch::batch;

mod one;
pub(crate) use one::one;

mod call_err;
pub use call_err::CallErr;
pub(crate) use call_err::{call_err, miss_func};

mod run;
pub use run::run;

fn res(code: State, body: impl AsRef<[u8]>) -> CodeBody {
  (code, body.as_ref().into())
}
