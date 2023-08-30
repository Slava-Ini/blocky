extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, LitStr, parse_macro_input};

#[allow(dead_code)]
#[derive(Debug)]
enum Content {
    Text(String),
    Number(u32),
    Children(Vec<Block>),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Block {
    width: u32,
    height: u32,
    name: String,
    content: Content,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let block_name: LitStr = input.parse()?;
        input.parse::<Token![;]>()?;
        let inner_text: LitStr = input.parse()?;

        println!("{:#?}{:#?}", block_name, inner_text);

        Ok(Block {
            width: 0,
            height: 0,
            name: String::new(),
            content: Content::Text(String::new()),
        })
    }
}

// block! {
//  ["wrapper_1"]
//      ["text_1"]("Hello, World!")
//      ["text_2"](123)
//  ["wrapper_2"]
//      ["text_3"]("Hello, World!")
//      ["text_4"](123)
// }

// block!(
//     ["text_1"]("hello_world")
// );

// block_style! {
//     wrapper_1 {
//         width: 100,
//         height: 100,
//     }
// }

// block_function! {
//     wrapper_1 {
//         on_focus: handle_focus,
//     }
// }

// fn handle_focus() {
//     println!("handle_focus");
// }

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    // let ast: syn::DeriveInput = syn::parse(input).unwrap();
    // println!("{:#?}", ast);
    let my_block = parse_macro_input!(input as Block);
    println!("{:#?}", my_block);
    "println!(\"Hello, world!\");".parse().unwrap()
}
