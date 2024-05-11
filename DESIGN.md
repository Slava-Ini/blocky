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

### Resources

- `tui-rs` example of handling higher-level structures (`crossterm`/`termion`)
- `zi` smaller library for interface manipulations (`crossterm`)
- `yew` dom manipulations - good macro examples
- `crossterm`/`termion` - select the one to be building around
- `inquirer` - terminal prompt (`crossterm`/`termion`/`console`)
- `ratatui` - terminal UI library (`crossterm`/`termion`/`wezterm`)
- `comfy-table` - terminal tables library (`crossterm`/`console`)
- `dialoguer` - terminal prompts etc. (`console`)

### Steps Done

### Next steps

- Select `crossterm`/`termion`/`console`/`ncurses`/`wezterm`
- Learn more about how terminal rendering works
- Decide on `blocky` workflow after researching
  - Partial rendering
  - Callbacks (maybe more rusty way of structs)

<!-- ## Conditionals     -->
<!-- ## Variables -->
<!-- ## Nested Blocks -->
<!-- ## Closures -->

<!-- # Blocky Design -->

<!-- # Styling -->
