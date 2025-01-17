// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ErrCode {
  User = 0,
  Captcha = 1,
  Form = 2,
}
impl ErrCode {
  pub const KNOWN_VARIANTS: [ErrCode; 3] = [ErrCode::User, ErrCode::Captcha, ErrCode::Form];
}
impl ::std::default::Default for ErrCode {
  fn default() -> Self {
    ErrCode::User
  }
}
impl From<ErrCode> for i32 {
  fn from(v: ErrCode) -> i32 {
    match v {
      ErrCode::User => 0,
      ErrCode::Captcha => 1,
      ErrCode::Form => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ErrCode {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ErrCode::User),
      1 => Ok(ErrCode::Captcha),
      2 => Ok(ErrCode::Form),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ErrCode {}
impl ::pb_jelly::ClosedProtoEnum for ErrCode {
  fn name(self) -> &'static str {
    match self {
      ErrCode::User => "User",
      ErrCode::Captcha => "Captcha",
      ErrCode::Form => "Form",
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
  pub func: i32,
  pub args: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for Call {
  fn default() -> Self {
    Call {
      func: ::std::default::Default::default(),
      args: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Call_default: Call = Call::default();
}
impl ::pb_jelly::Message for Call {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Call",
      full_name: "Call",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "func",
          full_name: "Call.func",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "args",
          full_name: "Call.args",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<i32>(
      &self.func,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.args,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, i32>(
      w,
      &self.func,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.args,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "Call",
            1,
          )?;
          self.func = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "Call", 2,
          )?;
          self.args = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Call {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "func" => ::pb_jelly::reflection::FieldMut::Value(&mut self.func),
      "args" => ::pb_jelly::reflection::FieldMut::Value(&mut self.args),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CallLi {
  pub call_li: ::std::vec::Vec<Call>,
}
impl ::std::default::Default for CallLi {
  fn default() -> Self {
    CallLi {
      call_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref CallLi_default: CallLi = CallLi::default();
}
impl ::pb_jelly::Message for CallLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CallLi",
      full_name: "CallLi",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "call_li",
        full_name: "CallLi.call_li",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::LengthDelimited,
        label: ::pb_jelly::Label::Repeated,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    for val in &self.call_li {
      size += ::pb_jelly::helpers::compute_size_field::<Call>(
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.call_li {
      ::pb_jelly::helpers::serialize_field::<W, Call>(
        w,
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, Call>(buf, typ, "CallLi", 1)?;
          self.call_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for CallLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "call_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinLi {
  pub bin_li: ::std::vec::Vec<::std::vec::Vec<u8>>,
}
impl ::std::default::Default for BinLi {
  fn default() -> Self {
    BinLi {
      bin_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref BinLi_default: BinLi = BinLi::default();
}
impl ::pb_jelly::Message for BinLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "BinLi",
      full_name: "BinLi",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "bin_li",
        full_name: "BinLi.bin_li",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::LengthDelimited,
        label: ::pb_jelly::Label::Repeated,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    for val in &self.bin_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::vec::Vec<u8>>(
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.bin_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::vec::Vec<u8>>(
        w,
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "BinLi", 1,
          )?;
          self.bin_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for BinLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "bin_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormErr {
  pub code: u32,
  pub bin: ::std::option::Option<::std::vec::Vec<u8>>,
}
impl ::std::default::Default for FormErr {
  fn default() -> Self {
    FormErr {
      code: ::std::default::Default::default(),
      bin: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref FormErr_default: FormErr = FormErr::default();
}
impl ::pb_jelly::Message for FormErr {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FormErr",
      full_name: "FormErr",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "code",
          full_name: "FormErr.code",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "bin",
          full_name: "FormErr.bin",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.code,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    if let Some(ref val) = self.bin {
      size += ::pb_jelly::helpers::compute_size_field::<::std::vec::Vec<u8>>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.code,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    if let Some(ref val) = self.bin {
      ::pb_jelly::helpers::serialize_field::<W, ::std::vec::Vec<u8>>(
        w,
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "FormErr",
            1,
          )?;
          self.code = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "FormErr", 2,
          )?;
          self.bin = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FormErr {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "code" => ::pb_jelly::reflection::FieldMut::Value(&mut self.code),
      "bin" => ::pb_jelly::reflection::FieldMut::Value(
        self
          .bin
          .get_or_insert_with(::std::default::Default::default),
      ),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormErrLi {
  pub form_err_li: ::std::vec::Vec<FormErr>,
}
impl ::std::default::Default for FormErrLi {
  fn default() -> Self {
    FormErrLi {
      form_err_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref FormErrLi_default: FormErrLi = FormErrLi::default();
}
impl ::pb_jelly::Message for FormErrLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FormErrLi",
      full_name: "FormErrLi",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "form_err_li",
        full_name: "FormErrLi.form_err_li",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::LengthDelimited,
        label: ::pb_jelly::Label::Repeated,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    for val in &self.form_err_li {
      size += ::pb_jelly::helpers::compute_size_field::<FormErr>(
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.form_err_li {
      ::pb_jelly::helpers::serialize_field::<W, FormErr>(
        w,
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FormErr>(
            buf,
            typ,
            "FormErrLi",
            1,
          )?;
          self.form_err_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FormErrLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "form_err_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrMsg {
  pub err_code: ErrCode,
  pub bin: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for ErrMsg {
  fn default() -> Self {
    ErrMsg {
      err_code: ::std::default::Default::default(),
      bin: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref ErrMsg_default: ErrMsg = ErrMsg::default();
}
impl ::pb_jelly::Message for ErrMsg {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ErrMsg",
      full_name: "ErrMsg",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "err_code",
          full_name: "ErrMsg.err_code",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "bin",
          full_name: "ErrMsg.bin",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<ErrCode>(
      &self.err_code,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.bin,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ErrCode>(
      w,
      &self.err_code,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.bin,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, ErrCode>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "ErrMsg",
            1,
          )?;
          self.err_code = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "ErrMsg", 2,
          )?;
          self.bin = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ErrMsg {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "err_code" => ::pb_jelly::reflection::FieldMut::Value(&mut self.err_code),
      "bin" => ::pb_jelly::reflection::FieldMut::Value(&mut self.bin),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
