//! This crate provides `tracked`'s procedural macro.
//!
//! Please refer to the `tracked` crate for details.

#![forbid(unsafe_code)]

use quote::quote;
use syn::parse_quote_spanned;
use syn::spanned::Spanned;
use syn::visit_mut::{self, VisitMut};

struct TrackReplace(String);

impl VisitMut for TrackReplace {
	fn visit_expr_try_mut(&mut self, node: &mut syn::ExprTry) {
		let fn_name = &self.0;
		let expr = &node.expr;
		let span = node.question_token.span();
		*node.expr = parse_quote_spanned!(span => tracked::Track::t_named( #expr, #fn_name ));
		visit_mut::visit_expr_try_mut(self, node);
	}
	fn visit_macro_mut(&mut self, node: &mut syn::Macro) {
		// Macros that successfully parse as a block of valid Rust code or a
		// comma-separated list of expressions are recursed into.
		if let Ok(mut body) = node.parse_body_with(syn::Block::parse_within) {
			for stmt in &mut body {
				TrackReplace(self.0.clone()).visit_stmt_mut(stmt);
			}
			node.tokens = quote!( #(#body)* );
		} else if let Ok(mut punctuated) =
			node.parse_body_with(<syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>>::parse_terminated)
		{
			for expr in punctuated.iter_mut() {
				TrackReplace(self.0.clone()).visit_expr_mut(expr);
			}
			node.tokens = quote!( #punctuated );
		}
		visit_mut::visit_macro_mut(self, node);
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
	TrackReplace(new_fn.sig.ident.to_string()).visit_block_mut(&mut new_fn.block);
	#[cfg(test)]
	// #[allow(clippy::needless_question_mark)] should be on expression instead of whole fn
	todo_or_die::issue_closed!("rust-lang", "rust", 15701);
	let output = quote! {
		#[allow(clippy::needless_question_mark)]
		#new_fn
	};
	output.into()
}

// #[proc_macro]
// pub fn track(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//  let mut block = syn::Block {
//   brace_token: syn::token::Brace::default(),
//   stmts: syn::parse_macro_input!(input with syn::Block::parse_within),
//  };
//  // let mut block = syn::parse_macro_input!(input as syn::Block);
//  TrackReplace.visit_block_mut(&mut block);
//  let stmts = block.stmts;
//  let output = quote! {
//   #(#stmts)*
//  };
//  output.into()
// }
