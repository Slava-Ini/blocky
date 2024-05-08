# Blocky Macro

## `block!` macro

A macro for creating nested blocks

### General

- `<>` value is class (for layout and functionality) - can only be string literal
- `()` value can be ident(string) or string literal is the text content of the block or a number
- `<>` without `()` is an empty wrapper

Example:

```rust
let text = "second";

let b: Block = block! {
   <"wrapper">
     <"text_1">("first")
     <"text_2">(second)
       <"inner-wrapper">
        <"text_3">("inner")
};
```

### Looping

- Looping is done using `{}`, where inside of `{}` is expression that should return a list of blocks

Example 1:

```rust
let items = vec!["one", "two", "three"];

let b: Block = block! {
    <"wrapper">
      { items.iter().collect::<Block>() }
}
```

Example 2:

```rust
let items = vec!["one", "two", "three"];

let b: Block = block! {
    <"wrapper">
    { items.iter().map(|text| {
            block! {
               <"line">(text)
          }
    })}

}
```



<!-- ## Looping -->
<!-- ## Conditionals     -->
<!-- ## Variables -->
<!-- ## Nested Blocks -->
<!-- ## Closures -->

<!-- # Blocky Design -->

<!-- # Styling -->
