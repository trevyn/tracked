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

#[test]
fn run_main() {
 assert_eq!(format!("{:?}", main()), "Err(NoneError in main at tracked/tests/readme.rs:10:5)");
}
