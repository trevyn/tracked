# tracked

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
