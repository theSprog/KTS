TypeScript implement by Rust

Place the `.ts` source file under `resource` and run it to get the corresponding AST image. See the corresponding file example for details



### Stage

This project is still in the development stage, we will polish the readme documentation and the way to use



## Usage

To Run test cases:
    cargo test rs_name -- --nocapture --test-threads=4



Or  you can use it manually by this way

```rust
#[allow(dead_code, unused_imports)]
use kts::compiler::Compiler;
use kts::error::err_exit;

// Passing the filename as a parameter
fn test_parser(filename: &str) {
    let compiler = Compiler::new(filename).set_show_ast();	// show ast
    let res = compiler.run();
    if let Err(err) = res {
        err_exit(err);
    }
}
```

