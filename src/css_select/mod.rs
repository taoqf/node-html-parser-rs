pub mod attributes;
pub mod compile;
pub mod convert;
pub mod general;
pub mod helpers;
pub mod legacy;
pub mod types;

use crate::{css_select::compile::compile_token as compile_token_impl, dom::element::HTMLElement};
use types::{Adapter, CompiledQuery, Options};

pub fn select_all<'a>(selector: &str, root: &'a HTMLElement) -> Vec<&'a HTMLElement> {
	legacy::query_selector_all(root, selector)
}

pub fn select_one<'a>(selector: &str, root: &'a HTMLElement) -> Option<&'a HTMLElement> {
	select_all(selector, root).into_iter().next()
}

pub fn is<'a>(elem: &'a HTMLElement, selector: &str, root: &'a HTMLElement) -> bool {
	// naive: run full selection and pointer compare
	let ptr = elem as *const HTMLElement;
	select_all(selector, root)
		.into_iter()
		.any(|e| std::ptr::eq(e as *const HTMLElement, ptr))
}

pub fn prepare_context<'a>(root: &'a HTMLElement) -> Vec<&'a HTMLElement> {
	vec![root]
}

// Placeholder compile that currently always returns false predicate; will be replaced
// by real compileToken + general selector pipeline.
pub fn compile<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

pub fn compile_token<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

pub fn compile_unsafe<'a, A: Adapter>(
	selector: &str,
	options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	compile_token_impl::<A>(selector, options, root)
}

// 新实验性内部编译（convert + general），暂不替换对外 API：
pub fn compile_experimental<'a, A: Adapter + 'a>(
	selector: &str,
	_options: &Options<A>,
	adapter: &'a A,
) -> CompiledQuery<'a, A> {
	// 直接调用新管线；调用方需自行提供 adapter 实例 (例如 HtmlAdapter)。
	crate::css_select::compile::compile_internal_new::<A>(selector, adapter)
}
