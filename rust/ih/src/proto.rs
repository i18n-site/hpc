// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(u32)]
pub enum State {
  OK = 0,
  JSON = 1,
  CODE = 2,
  CODE_LI = 3,
  BIN = 4,
  CAPTCHA = 10,
  NEED_SIGNIN = 11,
  NO_PERMISSION = 12,
  MISS_FUNC = 100,
  ARGS_INVALID = 101,
  BATCH_LIMIT = 102,
  CALL_ERROR = 103,
  MIDDLEWARE_ERROR = 104,
}
impl State {
  pub const KNOWN_VARIANTS: [State; 13] = [
    State::OK,
    State::JSON,
    State::CODE,
    State::CODE_LI,
    State::BIN,
    State::CAPTCHA,
    State::NEED_SIGNIN,
    State::NO_PERMISSION,
    State::MISS_FUNC,
    State::ARGS_INVALID,
    State::BATCH_LIMIT,
    State::CALL_ERROR,
    State::MIDDLEWARE_ERROR,
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
      State::JSON => 1,
      State::CODE => 2,
      State::CODE_LI => 3,
      State::BIN => 4,
      State::CAPTCHA => 10,
      State::NEED_SIGNIN => 11,
      State::NO_PERMISSION => 12,
      State::MISS_FUNC => 100,
      State::ARGS_INVALID => 101,
      State::BATCH_LIMIT => 102,
      State::CALL_ERROR => 103,
      State::MIDDLEWARE_ERROR => 104,
    }
  }
}
impl ::std::convert::TryFrom<u32> for State {
  type Error = u32;
  fn try_from(v: u32) -> ::std::result::Result<Self, u32> {
    match v {
      0 => Ok(State::OK),
      1 => Ok(State::JSON),
      2 => Ok(State::CODE),
      3 => Ok(State::CODE_LI),
      4 => Ok(State::BIN),
      10 => Ok(State::CAPTCHA),
      11 => Ok(State::NEED_SIGNIN),
      12 => Ok(State::NO_PERMISSION),
      100 => Ok(State::MISS_FUNC),
      101 => Ok(State::ARGS_INVALID),
      102 => Ok(State::BATCH_LIMIT),
      103 => Ok(State::CALL_ERROR),
      104 => Ok(State::MIDDLEWARE_ERROR),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for State {}
impl ::pb_jelly::ClosedProtoEnum for State {
  fn name(self) -> &'static str {
    match self {
      State::OK => "OK",
      State::JSON => "JSON",
      State::CODE => "CODE",
      State::CODE_LI => "CODE_LI",
      State::BIN => "BIN",
      State::CAPTCHA => "CAPTCHA",
      State::NEED_SIGNIN => "NEED_SIGNIN",
      State::NO_PERMISSION => "NO_PERMISSION",
      State::MISS_FUNC => "MISS_FUNC",
      State::ARGS_INVALID => "ARGS_INVALID",
      State::BATCH_LIMIT => "BATCH_LIMIT",
      State::CALL_ERROR => "CALL_ERROR",
      State::MIDDLEWARE_ERROR => "MIDDLEWARE_ERROR",
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
pub struct CodeLi {
  pub li: ::std::vec::Vec<u32>,
}
impl ::std::default::Default for CodeLi {
  fn default() -> Self {
    CodeLi {
      li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref CodeLi_default: CodeLi = CodeLi::default();
}
impl ::pb_jelly::Message for CodeLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeLi",
      full_name: "CodeLi",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "li",
        full_name: "CodeLi.li",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::Varint,
        label: ::pb_jelly::Label::Repeated,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if !self.li.is_empty() {
      let mut li_size = 0usize;
      for val in &self.li {
        li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(li_size as u64);
      size += li_size;
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.li.is_empty() {
      let mut size = 0usize;
      for val in &self.li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
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
            "CodeLi",
            1,
            &mut self.li,
          )?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for CodeLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bin {
  pub inner: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for Bin {
  fn default() -> Self {
    Bin {
      inner: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Bin_default: Bin = Bin::default();
}
impl ::pb_jelly::Message for Bin {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Bin",
      full_name: "Bin",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "inner",
        full_name: "Bin.inner",
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
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.inner,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "Bin", 1,
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
impl ::pb_jelly::Reflection for Bin {
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
pub struct Captcha {
  pub id: ::std::vec::Vec<u8>,
  pub img: ::std::vec::Vec<u8>,
  pub tip: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for Captcha {
  fn default() -> Self {
    Captcha {
      id: ::std::default::Default::default(),
      img: ::std::default::Default::default(),
      tip: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Captcha_default: Captcha = Captcha::default();
}
impl ::pb_jelly::Message for Captcha {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Captcha",
      full_name: "Captcha",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "id",
          full_name: "Captcha.id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "img",
          full_name: "Captcha.img",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "tip",
          full_name: "Captcha.tip",
          index: 2,
          number: 3,
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
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.id,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.img,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.tip,
      3,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.id,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.img,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.tip,
      3,
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "Captcha", 1,
          )?;
          self.id = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "Captcha", 2,
          )?;
          self.img = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "Captcha", 3,
          )?;
          self.tip = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Captcha {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "id" => ::pb_jelly::reflection::FieldMut::Value(&mut self.id),
      "img" => ::pb_jelly::reflection::FieldMut::Value(&mut self.img),
      "tip" => ::pb_jelly::reflection::FieldMut::Value(&mut self.tip),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
