//! CSS selector public API functions.

use super::legacy;
use super::types::{Adapter, CompiledQuery, Options};
use crate::{css_select::compile::compile_token as compile_token_impl, dom::element::HTMLElement};

/// Select all elements matching the CSS selector.
pub fn select_all<'a>(selector: &str, root: &'a HTMLElement) -> Vec<&'a HTMLElement> {
	legacy::query_selector_all(root, selector)
}

/// Select the first element matching the CSS selector.
pub fn select_one<'a>(selector: &str, root: &'a HTMLElement) -> Option<&'a HTMLElement> {
	select_all(selector, root).into_iter().next()
}

/// Check if the element matches the CSS selector.
pub fn is<'a>(elem: &'a HTMLElement, selector: &str, root: &'a HTMLElement) -> bool {
	// naive: run full selection and pointer compare
	let ptr = elem as *const HTMLElement;
	select_all(selector, root)
		.into_iter()
		.any(|e| std::ptr::eq(e as *const HTMLElement, ptr))
}

/// Prepare selection context.
pub fn prepare_context<'a>(root: &'a HTMLElement) -> Vec<&'a HTMLElement> {
	vec![root]
}

/// Compile CSS selector with options and root element.
pub fn compile<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

/// Compile a CSS selector token.
pub fn compile_token<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

/// Compile CSS selector without safety checks.
pub fn compile_unsafe<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

/// Experimental internal compilation (convert + general).
pub fn compile_experimental<'a, A: Adapter + 'a>(
	selector: &str,
	_options: &Options<A>,
	adapter: &'a A,
) -> CompiledQuery<'a, A> {
	// 直接调用新管线；调用方需自行提供 adapter 实例 (例如 HtmlAdapter)。
	crate::css_select::compile::compile_internal_new::<A>(selector, adapter)
}
