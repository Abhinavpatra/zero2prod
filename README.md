```bash
cargo install cargo-watch
```
- cargo-watch monitors your source code to trigger commands every time a file changes.
now to run we can use the command: `cargo watch -x check`

```bash
cargo install cargo-tarpaulin
```

- At the time of writing tarpaulin only supports
- x86_64 CPU architectures running Linux.


Web architectures
I will be going with Actix web
in cargo.toml file add :

```toml
[dependencies]
actix-web = "4"
```

pg-26
# Actix web
To use for Backend Web Development
First add an actix-web
Add 
- serde, serde_json: for serialization de-serialization
- env_logger, dotenv: for env handling
