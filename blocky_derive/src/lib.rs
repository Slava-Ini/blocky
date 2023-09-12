extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, LitStr, Token};

#[derive(Debug)]
enum Content {
    Text(String),
    Children(Vec<Block>),
}

#[derive(Debug)]
struct Block {
    name: String,
    content: Content,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut text: String = String::new();
        let mut name: String = String::new();
        let mut children = Vec::new();

        println!("{:#?}", input);

        while !input.cursor().eof() {
            input.parse::<Token![<]>()?;
            name = input.parse::<LitStr>()?.value();
            input.parse::<Token![>]>()?;

            if input.peek(Token![<]) {
                // while input.peek(Token![<]) {
                let child: Block = input.parse()?;
                children.push(child);
                // }

                continue;
            }

            let content;
            parenthesized!(content in input);
            text = content.parse::<LitStr>()?.value();
        }

        if !children.is_empty() {
            return Ok(Block {
                name,
                content: Content::Children(children),
            });
        }

        Ok(Block {
            name,
            content: Content::Text(text),
        })
    }
}

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    let my_block = parse_macro_input!(input as Block);
    println!("{:#?}", my_block);
    "println!(\"Hello, world!\");".parse().unwrap()
}
