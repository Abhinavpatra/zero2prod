
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
