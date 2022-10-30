# Changelog

## [0.5.3] - 2022-10-30

- Allow trying bools: `true?` is Ok, `false?` is Err.

## [0.5.2] - 2022-06-03

- Include function name in error message.

## [0.5.1] - 2022-06-03

- `impl From<&str> for StringError`.

## [0.5.0] - 2022-04-20

- Remove `anyhow` and make `serde` optional.

## [0.4.0] - 2022-04-17

- `#[tracked]` will now recurse into macros that successfully parse as a list of comma-separated expressions.

## [0.3.0] - 2022-04-16

- `#[tracked]` will now recurse into macros that successfully parse as statement blocks.

## [0.2.1] - 2022-04-14

- `StringError` is now `Serialize`/`Deserialize`/`Clone`.

## [0.2.0] - 2022-03-29

- `Track` returns `StringError` now.

## [0.1.1] - 2022-02-13

- Added `tracked::set_build_id`.
- Fixed `clippy::needless_question_mark` warnings.

## [0.1.0] - 2022-01-29

- Initial release! 🎉
