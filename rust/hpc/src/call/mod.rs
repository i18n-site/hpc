use ih::{BinLi, State};

mod batch;
pub(crate) use batch::batch;

mod one;
pub(crate) use one::one;

pub mod call_err;
pub(crate) use call_err::miss_func;

mod run;
pub use run::run;

pub fn bin_li(code: State, body: impl AsRef<[u8]>) -> BinLi {
  BinLi {
    state_li: vec![code.into()],
    bin_li: vec![body.as_ref().into()],
  }
}
