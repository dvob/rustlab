# Project structure
## Modules
The file `main.rs` or `lib.rs` is the entrypoint for the compiler.
To split your logic into multiple units/files you can introduce new modules.
Lets assume we want to put the utility function `run()` into a seperate module and then  call it from `main()`.

### Seperate file
Put the `run()` function into one of these files:
* `src/util.rs`
* `src/util/mod.rs`

```rust
mod util; // resolves to src/util.rs or src/util/mod.rs

fn main() {
    util::run()
}
```

### Module block
Add a module block to `src/main.rs`:
```rust
mod util {
    pub fn run() {}
}

fn main() {
    util::run()
}
```
# Links
* Modules
  * https://doc.rust-lang.org/reference/items/modules.html
  * https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html
