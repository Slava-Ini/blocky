// TODO: 
// [ ] - Check all structs visibility
// [ ] - Display trait for Block may be a feature
#[derive(Debug)]
pub enum Content {
    Children(Vec<Block>),
    Text(String),
}

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub content: Content,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = String::from("");

        fn traverse(
            block: &Block,
            f: &mut std::fmt::Formatter,
            mut indent: String,
        ) -> std::fmt::Result {
            match &block.content {
                Content::Children(children) => {
                    write!(f, "{}<{}>\n", indent, block.name)?;

                    indent.push_str("  ");

                    for child in children.iter() {
                        traverse(child, f, indent.clone())?;
                    }
                }
                Content::Text(text) => {
                    write!(f, "{}<{}>({})\n", indent, block.name, text)?;
                }
            }

            Ok(())
        }

        traverse(self, f, indent)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use blocky_macro::block;

    #[test]
    fn test_hello() {
        let t: Block = block! {
          <"wrapper">
            <"text_1">("hello")
            <"text_2">("world")
            <"inner-wrapper">
              <"text_3">("inner")
        };

        println!("{}", t);
    }
}
