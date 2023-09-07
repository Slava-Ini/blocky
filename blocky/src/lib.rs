#[cfg(test)]
mod test {
    use blocky_derive::block;

    #[test]
    fn test_hello() {
        // block! {
        //  <"wrapper_1">
        //      <"text_1">("Hello, World!")
        //      <"text_2">(123)
        //  <"wrapper_2">
        //      <"text_3">("Hello, World!")
        //      <"text_4">(123)
        // }
        block! { <"text_1">("hello") };
    }
}
