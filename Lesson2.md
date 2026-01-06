
in rust
$[]  are generally referred to as Attributes
when above main functions specifically, they are called: 
`Proc macros`

Think of attributes as metadata or instructions for the Rust compiler. They tell the compiler to do something extra with the code that follows them.
1. lazy_static:
  Dependency that allows you to import from a .env file
```rust
lazy_static!{
    pub static ref ADDRESS: String = set_address();
    pub static PORT: u16 = set_port();
}
fn set_address()-> String{
    dotenv:dotenv().ok();
    env:: var
}
```
