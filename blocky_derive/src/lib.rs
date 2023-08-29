extern crate proc_macro;

use proc_macro::TokenStream;

enum Content {
    Text(String),
    Number(u32),
    Children(Vec<Block>),
}

struct Block {
    width: u32,
    height: u32,
    name: String,
    content: Content,
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
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    println!("{:#?}", ast);
    "println!(\"Hello, world!\");".parse().unwrap()
}


