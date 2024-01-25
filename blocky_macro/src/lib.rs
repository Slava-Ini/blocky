extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, LitStr, Token};

// TODO:
// [ ] - Check accordance with Hygiene rules (Notion)
// [ ] - Make syntax error messages more informative
//    ( ) - No empty wrappers
//    ( ) - No empty content
#[derive(Debug)]
enum VContent {
    Text(String),
    Children(::std::vec::Vec<VBlock>),
}

impl ToTokens for VContent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            VContent::Text(text) => tokens.extend(quote! {
                Content::Text(#text.to_string())
            }),
            VContent::Children(children) => tokens.extend(quote! {
                Content::Children(::std::vec![#(#children),*])
            }),
        }
    }
}

#[derive(Debug)]
struct VBlock {
    name: String,
    content: VContent,
}

impl VBlock {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let name = input.parse::<LitStr>()?.value();
        input.parse::<Token![>]>()?;

        if input.peek(Token![<]) {
            let mut children = ::std::vec::Vec::new();

            while !input.cursor().eof() {
                let child = VBlock::parse(input)?;
                children.push(child);
            }

            return Ok(VBlock {
                name,
                content: VContent::Children(children),
            });
        }

        let content;
        parenthesized!(content in input);
        let text = content.parse::<LitStr>()?.value();

        return Ok(VBlock {
            name,
            content: VContent::Text(text),
        });
    }
}

impl Parse for VBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(VBlock::parse(input)?)
    }
}

impl ToTokens for VBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let content = &self.content.to_token_stream();

        tokens.extend(quote! {
            Block {
                name: #name.to_string(),
                content: #content,
            }
        });
    }
}

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as VBlock);
    TokenStream::from(root.to_token_stream())
}
