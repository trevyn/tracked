use tracked::{tracked, Result};

fn f() -> Option<()> {
 None
}

#[tracked]
fn main() -> Result<()> {
 let _ = f()?;
 Ok(())
}

#[test]
fn run_main() {
 assert_eq!(format!("{:?}", main()), "Err(NoneError at tracked/tests/readme.rs:9:13)");
}
