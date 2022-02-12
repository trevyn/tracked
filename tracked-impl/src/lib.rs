//! This crate provides `tracked`'s procedural macro.
//!
//! Please refer to the `tracked` crate for details.

#![forbid(unsafe_code)]

use quote::quote;
use syn::parse_quote_spanned;
use syn::spanned::Spanned;
use syn::visit_mut::{self, VisitMut};

struct TrackReplace;

impl VisitMut for TrackReplace {
 fn visit_expr_try_mut(&mut self, node: &mut syn::ExprTry) {
  let expr = &node.expr;
  let span = node.question_token.span();
  *node.expr = parse_quote_spanned!(span => tracked::Track::t( #expr ));
  visit_mut::visit_expr_try_mut(self, node);
 }
}

/// Apply this to a `fn` to track line numbers for `?` errors.
#[proc_macro_attribute]
// #[proc_macro_error]
pub fn tracked(
 _args: proc_macro::TokenStream,
 input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
 let mut new_fn = syn::parse_macro_input!(input as syn::ItemFn);
 TrackReplace.visit_block_mut(&mut new_fn.block);
 #[cfg(feature = "todo-or-die")]
 // #[allow(clippy::needless_question_mark)] should be on expression instead of whole fn
 todo_or_die::issue_closed!("rust-lang", "rust", 15701);
 let output = quote! {
  #[allow(clippy::needless_question_mark)]
  #new_fn
 };
 output.into()
}
