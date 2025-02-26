use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn captcha(_attr: TokenStream, item: TokenStream) -> TokenStream {
  // 解析输入的 TokenStream 为一个函数项
  let mut item_fn = parse_macro_input!(item as ItemFn);

  // 使用 parse_quote! 直接解析 quote! 宏生成的 TokenStream
  let doc_attr: syn::Attribute = parse_quote! {
      #[doc = "@captcha"]
  };

  // 将新的属性添加到函数的所有属性的开头
  item_fn.attrs.insert(0, doc_attr);

  // 将修改后的函数项转换回 TokenStream
  let output = quote! {
      #item_fn
  };

  output.into()
}
