use std::collections::HashSet;

use crate::dom::element::HTMLElement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
	Descendant,
	Child,
	Adjacent,
	Sibling,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrOp {
	Exists,
	Eq,
	Prefix,
	Suffix,
	Substr,
	Includes,
	Dash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMode {
	Sensitive,
	Insensitive,
}

#[derive(Debug, Clone)]
pub struct AttrMatcher {
	pub name: String,
	pub op: AttrOp,
	pub value: String,
	pub case: CaseMode,
}

#[derive(Debug, Clone)]
pub enum NthExpr {
	Number(i32),
	Odd,
	Even,
	Pattern { a: i32, b: i32 },
}

#[derive(Debug, Clone)]
pub enum Pseudo {
	FirstChild,
	LastChild,
	OnlyChild,
	FirstOfType,
	LastOfType,
	OnlyOfType,
	NthChild(NthExpr),
	NthLastChild(NthExpr),
	NthOfType(NthExpr),
	NthLastOfType(NthExpr),
	Not(Vec<Selector>),
	Scope,
	Is(Vec<Selector>),
	Where(Vec<Selector>),
	Has(Vec<Selector>),
	Empty,
	Root,
}

#[derive(Debug, Clone, Default)]
pub struct CompoundSelector {
	pub tag: Option<String>,
	pub id: Option<String>,
	pub classes: Vec<String>,
	pub attrs: Vec<AttrMatcher>,
	pub pseudos: Vec<Pseudo>,
}

#[derive(Debug, Clone)]
pub struct Selector(pub Vec<(Option<Combinator>, CompoundSelector)>);

pub fn query_selector_all<'a>(root: &'a HTMLElement, selector_str: &str) -> Vec<&'a HTMLElement> {
	let selectors = parse_selector_list(selector_str);
	if selectors.is_empty() {
		return vec![];
	}

	// 注意：由于HTMLElement包含原始指针，不能直接并行处理
	// 但我们可以并行处理解析后的选择器列表本身
	// TODO: 未来可以通过重构HTMLElement的内存布局来支持完全并行处理

	let mut ptr_set: HashSet<*const HTMLElement> = HashSet::new();
	for sel in selectors {
		for el in apply_selector(root, &sel) {
			ptr_set.insert(el as *const HTMLElement);
		}
	}
	let mut ordered = Vec::new();
	collect_in_order_smart(root, &ptr_set, &mut ordered);
	ordered
}

fn collect_in_order<'a>(
	root: &'a HTMLElement,
	set: &HashSet<*const HTMLElement>,
	out: &mut Vec<&'a HTMLElement>,
) {
	// 优化：使用迭代式遍历替代递归，避免栈溢出并提升性能
	let mut stack = vec![root];

	while let Some(el) = stack.pop() {
		if !el.is_root() {
			let p = el as *const HTMLElement;
			if set.contains(&p) {
				out.push(el);
			}
		}

		// 收集子元素并逆序入栈以保持文档顺序
		let children: Vec<_> = el.iter_elements().collect();
		for child in children.into_iter().rev() {
			stack.push(child);
		}
	}
}

#[cfg(feature = "parallel")]
fn collect_in_order_parallel<'a>(
	root: &'a HTMLElement,
	set: &HashSet<*const HTMLElement>,
) -> Vec<&'a HTMLElement> {
	// 注意：由于HTMLElement包含原始指针，暂时禁用完全并行遍历
	// 使用改进的迭代版本替代
	let mut results = Vec::new();
	collect_in_order(root, set, &mut results);
	results
}

// 智能选择遍历策略
fn collect_in_order_smart<'a>(
	root: &'a HTMLElement,
	set: &HashSet<*const HTMLElement>,
	out: &mut Vec<&'a HTMLElement>,
) {
	#[cfg(feature = "parallel")]
	{
		// 估算DOM树的宽度和深度来决定使用哪种策略
		let (width, depth) = estimate_dom_dimensions(root);
		const PARALLEL_WIDTH_THRESHOLD: usize = 20;
		const PARALLEL_DEPTH_THRESHOLD: usize = 5;

		// 宽而浅的DOM树适合并行层级遍历
		if width >= PARALLEL_WIDTH_THRESHOLD && depth <= PARALLEL_DEPTH_THRESHOLD {
			let parallel_results = collect_in_order_parallel(root, set);
			out.extend(parallel_results);
			return;
		}
	}

	// 默认使用迭代式遍历
	collect_in_order(root, set, out);
}

#[cfg(feature = "parallel")]
fn estimate_dom_dimensions(root: &HTMLElement) -> (usize, usize) {
	// 快速估算DOM树的宽度和深度
	let mut max_width = 0;
	let mut depth = 0;
	let mut current_level = vec![root];

	while !current_level.is_empty() && depth < 10 {
		// 限制深度评估避免过度计算
		max_width = max_width.max(current_level.len());
		let next_level: Vec<_> = current_level
			.iter()
			.flat_map(|el| el.iter_elements())
			.collect();
		current_level = next_level;
		depth += 1;
	}

	(max_width, depth)
}

fn apply_selector<'a>(root: &'a HTMLElement, selector: &Selector) -> Vec<&'a HTMLElement> {
	let mut current: Vec<&HTMLElement> = vec![root];
	for (idx, (comb, comp)) in selector.0.iter().enumerate() {
		let mut next_vec: Vec<&HTMLElement> = Vec::new();
		for base in &current {
			let effective = comb.unwrap_or(Combinator::Descendant);
			match effective {
				Combinator::Descendant => {
					if idx == 0 {
						if match_compound(root, base, comp) {
							next_vec.push(base);
						}
					}
					collect_descendants(root, base, comp, &mut next_vec);
				}
				Combinator::Child => {
					for child in base.iter_elements() {
						if match_compound(root, child, comp) {
							next_vec.push(child);
						}
					}
				}
				Combinator::Adjacent => {
					if let Some(parent) = find_parent(root, base) {
						let sibs: Vec<&HTMLElement> = parent.iter_elements().collect();
						if let Some(pos) = sibs.iter().position(|e| std::ptr::eq(*e, *base)) {
							if pos + 1 < sibs.len() {
								let n = sibs[pos + 1];
								if match_compound(root, n, comp) {
									next_vec.push(n);
								}
							}
						}
					}
				}
				Combinator::Sibling => {
					if let Some(parent) = find_parent(root, base) {
						let sibs: Vec<&HTMLElement> = parent.iter_elements().collect();
						if let Some(pos) = sibs.iter().position(|e| std::ptr::eq(*e, *base)) {
							for n in sibs.iter().skip(pos + 1) {
								if match_compound(root, *n, comp) {
									next_vec.push(*n);
								}
							}
						}
					}
				}
			}
		}
		let mut seen: HashSet<*const HTMLElement> = HashSet::new();
		let mut dedup = Vec::new();
		for e in next_vec {
			let p = e as *const HTMLElement;
			if seen.insert(p) {
				dedup.push(e);
			}
		}
		current = dedup;
	}
	current.into_iter().filter(|e| !e.is_root()).collect()
}

fn collect_descendants<'a>(
	root: &'a HTMLElement,
	el: &'a HTMLElement,
	comp: &CompoundSelector,
	out: &mut Vec<&'a HTMLElement>,
) {
	for child in el.iter_elements() {
		if match_compound(root, child, comp) {
			out.push(child);
		}
		collect_descendants(root, child, comp, out);
	}
}

fn match_compound<'a>(root: &'a HTMLElement, el: &'a HTMLElement, comp: &CompoundSelector) -> bool {
	if let Some(tag) = &comp.tag {
		if !el.name().eq_ignore_ascii_case(tag) {
			return false;
		}
	}
	if let Some(id) = &comp.id {
		match el.get_attr("id") {
			Some(v) if v == id => {}
			_ => return false,
		}
	}
	for class in &comp.classes {
		if !el.class_list_view().iter().any(|c| c == class) {
			return false;
		}
	}
	if !comp.attrs.is_empty() {
		for matcher in &comp.attrs {
			// Check if element has this attribute (case insensitive name matching)
			// First ensure all attributes are available for searching
			let mut_ptr = el as *const HTMLElement as *mut HTMLElement;
			unsafe {
				(*mut_ptr).ensure_all_attrs();
			}

			// Find attribute with case-insensitive name matching
			let raw_opt = unsafe {
				(*mut_ptr).attrs.iter().find_map(|(k, v)| {
					if k.eq_ignore_ascii_case(&matcher.name) {
						Some(v.as_str())
					} else {
						None
					}
				})
			};

			match matcher.op {
				AttrOp::Exists => {
					if raw_opt.is_none() {
						return false;
					}
				}
				_ => {
					if raw_opt.is_none() {
						return false;
					}
					let val = raw_opt.unwrap();
					let (left, right) = match matcher.case {
						CaseMode::Insensitive => {
							(val.to_ascii_lowercase(), matcher.value.to_ascii_lowercase())
						}
						CaseMode::Sensitive => (val.to_string(), matcher.value.clone()),
					};
					let ok = match matcher.op {
						AttrOp::Exists => true,
						AttrOp::Eq => left == right,
						AttrOp::Prefix => left.starts_with(&right),
						AttrOp::Suffix => left.ends_with(&right),
						AttrOp::Substr => left.contains(&right),
						AttrOp::Includes => left.split_whitespace().any(|t| t == right),
						AttrOp::Dash => left == right || left.starts_with(&(right + "-")),
					};
					if !ok {
						return false;
					}
				}
			}
		}
	}
	if !comp.pseudos.is_empty() {
		for p in &comp.pseudos {
			if !match_pseudo(root, el, p) {
				return false;
			}
		}
	}
	true
}

fn match_pseudo<'a>(root: &'a HTMLElement, el: &'a HTMLElement, pseudo: &Pseudo) -> bool {
	match pseudo {
		Pseudo::FirstChild => position_in_parent(root, el)
			.map(|(i, _, _, _)| i == 0)
			.unwrap_or(false),
		Pseudo::LastChild => position_in_parent(root, el)
			.map(|(i, len, _, _)| i + 1 == len)
			.unwrap_or(false),
		Pseudo::OnlyChild => position_in_parent(root, el)
			.map(|(_, len, _, _)| len == 1)
			.unwrap_or(false),
		Pseudo::FirstOfType => position_in_parent(root, el)
			.map(|(_, _, ti, _)| ti == 0)
			.unwrap_or(false),
		Pseudo::LastOfType => position_in_parent(root, el)
			.map(|(_, _, ti, tlen)| ti + 1 == tlen)
			.unwrap_or(false),
		Pseudo::OnlyOfType => position_in_parent(root, el)
			.map(|(_, _, _, tlen)| tlen == 1)
			.unwrap_or(false),
		Pseudo::NthChild(expr) => position_in_parent(root, el)
			.map(|(i, _, _, _)| match_nth(expr, i as i32 + 1))
			.unwrap_or(false),
		Pseudo::NthLastChild(expr) => position_in_parent(root, el)
			.map(|(i, len, _, _)| {
				let rev = (len - i - 1) as i32 + 1;
				match_nth(expr, rev)
			})
			.unwrap_or(false),
		Pseudo::NthOfType(expr) => position_in_parent(root, el)
			.map(|(_, _, ti, _)| match_nth(expr, ti as i32 + 1))
			.unwrap_or(false),
		Pseudo::NthLastOfType(expr) => position_in_parent(root, el)
			.map(|(_, _, ti, tlen)| {
				let rev = (tlen - ti - 1) as i32 + 1;
				match_nth(expr, rev)
			})
			.unwrap_or(false),
		Pseudo::Not(list) => !list.iter().any(|sel| apply_selector_from_el(root, el, sel)),
		Pseudo::Empty => {
			el.iter_elements().next().is_none()
				&& el
					.children
					.iter()
					.all(|n| n.as_element().is_some() || n.text().trim().is_empty())
		}
		Pseudo::Root => find_parent(root, el).map(|p| p.is_root()).unwrap_or(false),
		Pseudo::Is(list) => list.iter().any(|sel| apply_selector_from_el(root, el, sel)),
		Pseudo::Where(list) => list.iter().any(|sel| apply_selector_from_el(root, el, sel)),
		Pseudo::Has(list) => {
			// :has(A) 若当前元素存在后代匹配 A；使用 DFS
			fn any_desc<'a>(
				cur: &'a HTMLElement,
				root: &'a HTMLElement,
				list: &Vec<Selector>,
			) -> bool {
				for child in cur.iter_elements() {
					for sel in list {
						if apply_selector_from_el(root, child, sel) {
							return true;
						}
					}
					if any_desc(child, root, list) {
						return true;
					}
				}
				false
			}
			any_desc(el, root, list)
		}
		Pseudo::Scope => true,
	}
}

fn match_nth(expr: &NthExpr, index_one: i32) -> bool {
	match expr {
		NthExpr::Number(n) => index_one == *n,
		NthExpr::Odd => (index_one % 2) == 1,
		NthExpr::Even => (index_one % 2) == 0,
		NthExpr::Pattern { a, b } => {
			let a = *a;
			let b = *b;
			if a == 0 {
				return index_one == b;
			}
			if a > 0 {
				if index_one < b {
					return false;
				}
				(index_one - b) % a == 0
			} else {
				let mut k = 0;
				loop {
					let val = a * k + b;
					if val == index_one {
						return true;
					}
					if val < 1 {
						return false;
					}
					if val < index_one {
						k += 1;
						continue;
					}
					return false;
				}
			}
		}
	}
}

fn position_in_parent<'a>(
	root: &'a HTMLElement,
	el: &'a HTMLElement,
) -> Option<(usize, usize, usize, usize)> {
	let parent = find_parent(root, el)?;
	let list: Vec<&HTMLElement> = parent.iter_elements().collect();
	if list.is_empty() {
		return None;
	}
	let same: Vec<&HTMLElement> = list
		.iter()
		.copied()
		.filter(|c| c.name() == el.name())
		.collect();
	let idx = list.iter().position(|e| std::ptr::eq(*e, el))?;
	let tidx = same.iter().position(|e| std::ptr::eq(*e, el))?;
	Some((idx, list.len(), tidx, same.len()))
}

fn find_parent<'a>(root: &'a HTMLElement, target: &'a HTMLElement) -> Option<&'a HTMLElement> {
	if std::ptr::eq(root, target) {
		return None;
	}
	fn dfs<'a>(cur: &'a HTMLElement, target: &'a HTMLElement) -> Option<&'a HTMLElement> {
		for child in cur.iter_elements() {
			if std::ptr::eq(child, target) {
				return Some(cur);
			}
			if let Some(p) = dfs(child, target) {
				return Some(p);
			}
		}
		None
	}
	dfs(root, target)
}

fn parse_selector_list(input: &str) -> Vec<Selector> {
	let mut parts = Vec::new();
	let mut buf = String::new();
	let mut depth = 0; // parentheses depth
	for c in input.chars() {
		match c {
			'(' => {
				depth += 1;
				buf.push(c);
			}
			')' => {
				if depth > 0 {
					depth -= 1;
				}
				buf.push(c);
			}
			',' if depth == 0 => {
				let t = buf.trim();
				if !t.is_empty() {
					parts.push(t.to_string());
				}
				buf.clear();
			}
			_ => buf.push(c),
		}
	}
	let t = buf.trim();
	if !t.is_empty() {
		parts.push(t.to_string());
	}
	parts
		.into_iter()
		.filter_map(|p| {
			let tt = p.trim();
			if tt.is_empty() {
				None
			} else {
				Some(parse_single_selector(tt))
			}
		})
		.collect()
}

fn parse_single_selector(input: &str) -> Selector {
	let mut chars = input.chars().peekable();
	let mut parts: Vec<(Option<Combinator>, CompoundSelector)> = Vec::new();
	let mut current = CompoundSelector::default();
	let mut pending: Option<Combinator> = None;
	while let Some(&ch) = chars.peek() {
		match ch {
			' ' | '\t' | '\n' | '\r' => {
				chars.next();
				if !current_is_empty(&current) {
					parts.push((pending.take(), current));
					current = CompoundSelector::default();
				}
				pending.get_or_insert(Combinator::Descendant);
				while let Some(&c2) = chars.peek() {
					if c2.is_whitespace() {
						chars.next();
					} else {
						break;
					}
				}
			}
			'>' => {
				chars.next();
				if !current_is_empty(&current) {
					parts.push((pending.take(), current));
					current = CompoundSelector::default();
				}
				pending = Some(Combinator::Child);
				skip_ws(&mut chars);
			}
			'+' => {
				chars.next();
				if !current_is_empty(&current) {
					parts.push((pending.take(), current));
					current = CompoundSelector::default();
				}
				pending = Some(Combinator::Adjacent);
				skip_ws(&mut chars);
			}
			'~' => {
				chars.next();
				if !current_is_empty(&current) {
					parts.push((pending.take(), current));
					current = CompoundSelector::default();
				}
				pending = Some(Combinator::Sibling);
				skip_ws(&mut chars);
			}
			'#' => {
				chars.next();
				let ident = read_ident(&mut chars);
				current.id = Some(ident);
			}
			'.' => {
				chars.next();
				let ident = read_ident(&mut chars);
				current.classes.push(ident);
			}
			'[' => {
				chars.next();
				let name = read_ident(&mut chars);
				skip_ws(&mut chars);
				let mut op = AttrOp::Exists;
				let mut value = String::new();
				let mut case = CaseMode::Sensitive;
				if let Some(&c2) = chars.peek() {
					match c2 {
						'=' => {
							op = AttrOp::Eq;
							chars.next();
							skip_ws(&mut chars);
							value = read_attr_value(&mut chars);
						}
						'^' | '$' | '*' | '~' | '|' => {
							let first = c2;
							chars.next();
							if matches!(chars.peek(), Some('=')) {
								chars.next();
								op = match first {
									'^' => AttrOp::Prefix,
									'$' => AttrOp::Suffix,
									'*' => AttrOp::Substr,
									'~' => AttrOp::Includes,
									'|' => AttrOp::Dash,
									_ => AttrOp::Eq,
								};
								skip_ws(&mut chars);
								value = read_attr_value(&mut chars);
							}
						}
						_ => {}
					}
				}
				skip_ws(&mut chars);
				if let Some(&flag) = chars.peek() {
					if flag == 'i' {
						case = CaseMode::Insensitive;
						chars.next();
					} else if flag == 's' {
						case = CaseMode::Sensitive;
						chars.next();
					}
				}
				while let Some(c) = chars.next() {
					if c == ']' {
						break;
					}
				}
				current.attrs.push(AttrMatcher {
					name: name.to_lowercase(),
					op,
					value,
					case,
				});
			}
			'*' => {
				chars.next();
			}
			':' => {
				chars.next();
				let pseudo_name = read_ident(&mut chars).to_ascii_lowercase();
				let pseudo = if matches!(chars.peek(), Some('(')) {
					chars.next();
					let arg = read_until_paren(&mut chars);
					parse_pseudo_with_arg(&pseudo_name, &arg)
				} else {
					parse_pseudo_no_arg(&pseudo_name)
				};
				if let Some(p) = pseudo {
					current.pseudos.push(p);
				}
			}
			_ => {
				if current.tag.is_none() {
					let ident = read_ident_starting(ch, &mut chars);
					current.tag = Some(ident.to_lowercase());
				} else {
					chars.next();
				}
			}
		}
	}
	if !current_is_empty(&current) || parts.is_empty() {
		parts.push((pending.take(), current));
	}
	Selector(parts)
}

fn current_is_empty(c: &CompoundSelector) -> bool {
	c.tag.is_none()
		&& c.id.is_none()
		&& c.classes.is_empty()
		&& c.attrs.is_empty()
		&& c.pseudos.is_empty()
}
fn skip_ws<I: Iterator<Item = char>>(it: &mut std::iter::Peekable<I>) {
	while matches!(it.peek(), Some(c) if c.is_whitespace()) {
		it.next();
	}
}
fn read_ident<I: Iterator<Item = char>>(it: &mut std::iter::Peekable<I>) -> String {
	let mut s = String::new();
	while let Some(&c) = it.peek() {
		if c.is_alphanumeric() || c == '-' || c == '_' {
			s.push(c);
			it.next();
		} else {
			break;
		}
	}
	s
}
fn read_ident_starting(first: char, it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
	let mut s = String::new();
	s.push(first);
	it.next();
	while let Some(&c) = it.peek() {
		if c.is_alphanumeric() || c == '-' || c == '_' {
			s.push(c);
			it.next();
		} else {
			break;
		}
	}
	s
}
fn read_attr_value<I: Iterator<Item = char>>(it: &mut std::iter::Peekable<I>) -> String {
	if let Some(&q) = it.peek() {
		if q == '"' || q == '\'' {
			it.next();
			let mut val = String::new();
			while let Some(&c) = it.peek() {
				it.next();
				if c == q {
					break;
				}
				val.push(c);
			}
			return val;
		}
	}
	read_ident(it)
}
fn read_until_paren<I: Iterator<Item = char>>(it: &mut std::iter::Peekable<I>) -> String {
	let mut depth = 1;
	let mut s = String::new();
	while let Some(&c) = it.peek() {
		it.next();
		if c == '(' {
			depth += 1;
		} else if c == ')' {
			depth -= 1;
			if depth == 0 {
				break;
			}
		}
		s.push(c);
	}
	s.trim().to_string()
}
fn parse_pseudo_no_arg(name: &str) -> Option<Pseudo> {
	Some(match name {
		"first-child" => Pseudo::FirstChild,
		"last-child" => Pseudo::LastChild,
		"only-child" => Pseudo::OnlyChild,
		"first-of-type" => Pseudo::FirstOfType,
		"last-of-type" => Pseudo::LastOfType,
		"only-of-type" => Pseudo::OnlyOfType,
		"empty" => Pseudo::Empty,
		"root" => Pseudo::Root,
		"scope" => Pseudo::Scope,
		_ => return None,
	})
}
fn parse_pseudo_with_arg(name: &str, arg: &str) -> Option<Pseudo> {
	match name {
		"nth-child" => Some(Pseudo::NthChild(parse_nth_expr(arg)?)),
		"nth-last-child" => Some(Pseudo::NthLastChild(parse_nth_expr(arg)?)),
		"nth-of-type" => Some(Pseudo::NthOfType(parse_nth_expr(arg)?)),
		"nth-last-of-type" => Some(Pseudo::NthLastOfType(parse_nth_expr(arg)?)),
		"not" => {
			let sels = parse_selector_list(arg);
			if sels.is_empty() {
				None
			} else {
				Some(Pseudo::Not(sels))
			}
		}
		"is" => {
			let sels = parse_selector_list(arg);
			if sels.is_empty() {
				None
			} else {
				Some(Pseudo::Is(sels))
			}
		}
		"where" => {
			let sels = parse_selector_list(arg);
			if sels.is_empty() {
				None
			} else {
				Some(Pseudo::Where(sels))
			}
		}
		"has" => {
			let sels = parse_selector_list(arg);
			if sels.is_empty() {
				None
			} else {
				Some(Pseudo::Has(sels))
			}
		}
		_ => None,
	}
}
fn parse_nth_expr(s: &str) -> Option<NthExpr> {
	let t = s.trim().to_ascii_lowercase();
	if t == "odd" {
		return Some(NthExpr::Odd);
	}
	if t == "even" {
		return Some(NthExpr::Even);
	}
	if let Ok(n) = t.parse::<i32>() {
		if n > 0 {
			return Some(NthExpr::Number(n));
		}
	}
	if let Some(pos) = t.find('n') {
		let (a_part, rest) = t.split_at(pos);
		let rest = &rest[1..];
		let a = if a_part.is_empty() || a_part == "+" {
			1
		} else if a_part == "-" {
			-1
		} else {
			a_part.parse().ok()?
		};
		let b = if rest.is_empty() {
			0
		} else {
			let r = rest.trim();
			if r.starts_with('+') {
				r[1..].parse().ok()?
			} else {
				r.parse().ok()?
			}
		};
		return Some(NthExpr::Pattern { a, b });
	}
	None
}

pub fn apply_selector_from_el<'a>(
	root: &'a HTMLElement,
	el: &'a HTMLElement,
	selector: &Selector,
) -> bool {
	let matches = apply_selector(root, selector);
	matches.into_iter().any(|e| std::ptr::eq(e, el))
}
pub fn parse_selector_list_public(input: &str) -> Vec<Selector> {
	parse_selector_list(input)
}

// Expose helper for conversion: return reference parts
pub fn selector_parts(sel: &Selector) -> &Vec<(Option<Combinator>, CompoundSelector)> {
	&sel.0
}
