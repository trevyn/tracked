Another way of spelling `anyhow`, with some extra features.

```rust,no_run
use tracked::{tracked, Result};

fn f() -> Option<()> {
    None
}

#[tracked]
fn main() -> Result<()> {
    let _ = f()?;
    Ok(())
}
```

```text
Error: NoneError at src/main.rs:9:16
```
