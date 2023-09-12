#[cfg(test)]
mod test {
    use blocky_derive::block;

    #[test]
    fn test_hello() {
        block! {
          <"wrapper">
            <"text_1">("hello")
            <"text_2">("world")
        };
    }
}

