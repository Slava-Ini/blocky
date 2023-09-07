extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, LitStr, Token};

#[allow(dead_code)]
#[derive(Debug)]
enum Content {
    Text(String),
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

        while !input.cursor().eof() {
            input.parse::<Token![<]>()?;
            let name: LitStr = input.parse()?;
            input.parse::<Token![>]>()?;
        }
        // let content;
        // let open_angle_bracket = input.parse::<Token![<]>()?;
        // let inner_text: LitStr = input.parse()?;
        // let close_angle_bracket = input.parse::<Token![>]>()?;
        // parenthesized!(content in input);
        // let paren_content: LitStr = content.parse()?;

        // println!(
        //     "{:#?}{:#?}{:#?}",
        //     open_angle_bracket, inner_text, close_angle_bracket
        // );
        // println!("{:#?}", paren_content);

        Ok(Block {
            width: 0,
            height: 0,
            name: String::new(),
            content: Content::Text(String::new()),
        })
    }
}

// block! {
//  <"wrapper_1">
//      <"text_1">("Hello, World!")
//      <"text_2">("123")
//  <"wrapper_2">
//      <"text_3">("Hello, World!")
//      <"text_4">("123")
// }

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
