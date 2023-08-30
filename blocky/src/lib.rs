#[cfg(test)]
mod test {
    use blocky_derive::block;

    #[test]
    fn test_hello() {
        block! { "text_1";"hello_world" };
    }
}
