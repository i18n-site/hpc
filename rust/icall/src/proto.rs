// @generated, do not edit
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
  pub func_li: ::std::vec::Vec<i32>,
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
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(
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
pub struct CodeBin {
  pub code: u32,
  pub bin: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for CodeBin {
  fn default() -> Self {
    CodeBin {
      code: ::std::default::Default::default(),
      bin: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref CodeBin_default: CodeBin = CodeBin::default();
}
impl ::pb_jelly::Message for CodeBin {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeBin",
      full_name: "CodeBin",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "code",
          full_name: "CodeBin.code",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "bin",
          full_name: "CodeBin.bin",
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
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.bin,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.code,
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
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "CodeBin",
            1,
          )?;
          self.code = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "CodeBin", 2,
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
impl ::pb_jelly::Reflection for CodeBin {
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
      "bin" => ::pb_jelly::reflection::FieldMut::Value(&mut self.bin),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Code {
  pub code: u32,
}
impl ::std::default::Default for Code {
  fn default() -> Self {
    Code {
      code: ::std::default::Default::default(),
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
        name: "code",
        full_name: "Code.code",
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
      &self.code,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.code,
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
          self.code = val;
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
      "code" => ::pb_jelly::reflection::FieldMut::Value(&mut self.code),
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
pub struct CodeMsgLi {
  pub code_li: ::std::vec::Vec<u32>,
  pub msg_li: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for CodeMsgLi {
  fn default() -> Self {
    CodeMsgLi {
      code_li: ::std::default::Default::default(),
      msg_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref CodeMsgLi_default: CodeMsgLi = CodeMsgLi::default();
}
impl ::pb_jelly::Message for CodeMsgLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeMsgLi",
      full_name: "CodeMsgLi",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "code_li",
          full_name: "CodeMsgLi.code_li",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "msg_li",
          full_name: "CodeMsgLi.msg_li",
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
    if !self.code_li.is_empty() {
      let mut code_li_size = 0usize;
      for val in &self.code_li {
        code_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(code_li_size as u64);
      size += code_li_size;
    }
    for val in &self.msg_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.code_li.is_empty() {
      let mut size = 0usize;
      for val in &self.code_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.code_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    for val in &self.msg_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(
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
            "CodeMsgLi",
            1,
            &mut self.code_li,
          )?;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf,
            typ,
            "CodeMsgLi",
            2,
          )?;
          self.msg_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for CodeMsgLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "code_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "msg_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
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
