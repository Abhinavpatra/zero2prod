1. Setup & Tooling
    cargo-watch: Auto-trigger commands on file changes.
    cargo-tarpaulin: Code coverage (Linux x86_64 restriction).
2. Web Architecture (Actix)
    Core Dependencies:
        serde / serde_json: Serialization.
        env_logger / dotenv: Environment configuration.
3. The Rust Module System
    The mod.rs Pattern: The "index.js" equivalent (Legacy).
    The Folder-Name Pattern: folder.rs at the same level (Modern/Preferred).
    Visibility: Mapping src/utils/ to the compiler.
  


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
oming from Python or Javascript?

Roughly, mod.rs is kind of like index.js in Javascript. But only kind of. This is a bit more complicated in Rust.
Rust is different

Folders are not immediately ready to use as modules in Rust.

You have to add a file named mod.rs in a folder to expose a new module named like that folder. The code in mod.rs is the content of that module. All other files in the folder may in turn be exposed as submodules (more on that below).
Wait, there is another way

Instead of adding a mod.rs file in the folder, `you may add a file named <folder_name>.rs at the same level as the folder.`

As noted by MarkusToman in the comments, this is the preferred way since rustc 1.30.
// Source - https://stackoverflow.com/a
// Posted by Romain Vincent, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-05, License - CC BY-SA 4.0

src
    utils
        bar.rs
        foo.rs
    main.rs
    utils.rs
  

Untill now compiler does not know about src/utils/foo.rs or bar.rs
you have 2 options:

    add the file: src/utils/mod.rs
    add the file src/utils.rs (named exactly like the folder, without the extension)
Then inside the <folder_name>.rs files, insert: `pub mod constant;` as the files if constant.rs exists inside that folder.

