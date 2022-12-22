# tracked

[<img alt="github" src="https://img.shields.io/badge/github-trevyn/tracked-663399?style=for-the-badge&labelColor=555555&logo=github" height="27">](https://github.com/trevyn/tracked)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tracked.svg?style=for-the-badge&color=ffc833&logo=rust" height="27">](https://crates.io/crates/tracked)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tracked-353535?style=for-the-badge&labelColor=555555&logo=docs.rs" height="27">](https://docs.rs/tracked)

A stringly-typed Error that includes `#[track_caller]` information.

Points you to the _location_ in your code that errored, without the `panic!`.

Also lets you try an `Option` or a `bool` into a `Result`.

```rust,no_run
use tracked::tracked;

fn f() -> Option<()> {
    None
}

#[tracked]
fn main() -> Result<(), tracked::StringError> {
    true?;
    f()?;
    Ok(())
}
```

```text
Error: NoneError in main at src/main.rs:10:8
```
