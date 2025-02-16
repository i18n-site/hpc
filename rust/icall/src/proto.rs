// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(u32)]
pub enum State {
  OK = 0,
  MISS_FUNC = 1,
  ARGS_DECODE_ERROR = 2,
  BATCH_LIMIT = 3,
  CALL_ERROR = 4,
  NO_PERMISSION = 5,
  NEED_SIGNIN = 6,
  CAPTCHA = 7,
  JSON = 8,
  CODE = 9,
}
impl State {
  pub const KNOWN_VARIANTS: [State; 10] = [
    State::OK,
    State::MISS_FUNC,
    State::ARGS_DECODE_ERROR,
    State::BATCH_LIMIT,
    State::CALL_ERROR,
    State::NO_PERMISSION,
    State::NEED_SIGNIN,
    State::CAPTCHA,
    State::JSON,
    State::CODE,
  ];
}
impl ::std::default::Default for State {
  fn default() -> Self {
    State::OK
  }
}
impl From<State> for u32 {
  fn from(v: State) -> u32 {
    match v {
      State::OK => 0,
      State::MISS_FUNC => 1,
      State::ARGS_DECODE_ERROR => 2,
      State::BATCH_LIMIT => 3,
      State::CALL_ERROR => 4,
      State::NO_PERMISSION => 5,
      State::NEED_SIGNIN => 6,
      State::CAPTCHA => 7,
      State::JSON => 8,
      State::CODE => 9,
    }
  }
}
impl ::std::convert::TryFrom<u32> for State {
  type Error = u32;
  fn try_from(v: u32) -> ::std::result::Result<Self, u32> {
    match v {
      0 => Ok(State::OK),
      1 => Ok(State::MISS_FUNC),
      2 => Ok(State::ARGS_DECODE_ERROR),
      3 => Ok(State::BATCH_LIMIT),
      4 => Ok(State::CALL_ERROR),
      5 => Ok(State::NO_PERMISSION),
      6 => Ok(State::NEED_SIGNIN),
      7 => Ok(State::CAPTCHA),
      8 => Ok(State::JSON),
      9 => Ok(State::CODE),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for State {}
impl ::pb_jelly::ClosedProtoEnum for State {
  fn name(self) -> &'static str {
    match self {
      State::OK => "OK",
      State::MISS_FUNC => "MISS_FUNC",
      State::ARGS_DECODE_ERROR => "ARGS_DECODE_ERROR",
      State::BATCH_LIMIT => "BATCH_LIMIT",
      State::CALL_ERROR => "CALL_ERROR",
      State::NO_PERMISSION => "NO_PERMISSION",
      State::NEED_SIGNIN => "NEED_SIGNIN",
      State::CAPTCHA => "CAPTCHA",
      State::JSON => "JSON",
      State::CODE => "CODE",
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CallLi {
  pub func_li: ::std::vec::Vec<u32>,
  pub args_li: ::std::vec::Vec<::std::vec::Vec<u8>>,
}
impl ::std::default::Default for CallLi {
  fn default() -> Self {
    CallLi {
      func_li: ::std::default::Default::default(),
      args_li: ::std::default::Default::default(),
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
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "func_li",
          full_name: "CallLi.func_li",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "args_li",
          full_name: "CallLi.args_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if !self.func_li.is_empty() {
      let mut func_li_size = 0usize;
      for val in &self.func_li {
        func_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(func_li_size as u64);
      size += func_li_size;
    }
    for val in &self.args_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::vec::Vec<u8>>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.func_li.is_empty() {
      let mut size = 0usize;
      for val in &self.func_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.func_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    for val in &self.args_li {
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
          ::pb_jelly::helpers::deserialize_packed::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "CallLi",
            1,
            &mut self.func_li,
          )?;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "CallLi", 2,
          )?;
          self.args_li.push(val);
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
      "func_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "args_li" => {
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
  pub state_li: ::std::vec::Vec<u32>,
  pub bin_li: ::std::vec::Vec<::std::vec::Vec<u8>>,
}
impl ::std::default::Default for BinLi {
  fn default() -> Self {
    BinLi {
      state_li: ::std::default::Default::default(),
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
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "state_li",
          full_name: "BinLi.state_li",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "bin_li",
          full_name: "BinLi.bin_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if !self.state_li.is_empty() {
      let mut state_li_size = 0usize;
      for val in &self.state_li {
        state_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(state_li_size as u64);
      size += state_li_size;
    }
    for val in &self.bin_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::vec::Vec<u8>>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.state_li.is_empty() {
      let mut size = 0usize;
      for val in &self.state_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.state_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    for val in &self.bin_li {
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
          ::pb_jelly::helpers::deserialize_packed::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "BinLi",
            1,
            &mut self.state_li,
          )?;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "BinLi", 2,
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
      "state_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "bin_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// for hpc/rust/apierr/src/err.rs
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Code {
  pub inner: u32,
}
impl ::std::default::Default for Code {
  fn default() -> Self {
    Code {
      inner: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Code_default: Code = Code::default();
}
impl ::pb_jelly::Message for Code {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Code",
      full_name: "Code",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "inner",
        full_name: "Code.inner",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::Varint,
        label: ::pb_jelly::Label::Optional,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.inner,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.inner,
      1,
      ::pb_jelly::wire_format::Type::Varint,
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
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "Code",
            1,
          )?;
          self.inner = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Code {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "inner" => ::pb_jelly::reflection::FieldMut::Value(&mut self.inner),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Json {
  pub inner: ::std::string::String,
}
impl ::std::default::Default for Json {
  fn default() -> Self {
    Json {
      inner: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Json_default: Json = Json::default();
}
impl ::pb_jelly::Message for Json {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Json",
      full_name: "Json",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "inner",
        full_name: "Json.inner",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::LengthDelimited,
        label: ::pb_jelly::Label::Optional,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.inner,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.inner,
      1,
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "Json", 1,
          )?;
          self.inner = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Json {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "inner" => ::pb_jelly::reflection::FieldMut::Value(&mut self.inner),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
