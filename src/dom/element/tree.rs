use super::content::collect_items;
use super::content::parse_fragment;
use super::main::HTMLElement;
use crate::dom::node::{CowStr, Node, NodeOrStr};

impl HTMLElement {
	// ---- Public multi-node insertion (parity with JS variadic before/after/prepend/append) ----
	/// Insert nodes (elements/text/comments) before this element in the parent's children list.
	pub fn before_nodes(&mut self, mut nodes: Vec<Node>) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let idx = match self.index_in_parent() {
				Some(i) => i,
				None => return,
			};
			for n in nodes.iter_mut() {
				Self::detach_node(n);
			}
			for (i, mut n) in nodes.into_iter().enumerate() {
				if let Node::Element(ref mut e) = n {
					e.parent = Some(parent_ptr);
				}
				parent.children.insert(idx + i, n);
			}
		}
	}

	/// 下一个元素兄弟。
	pub fn next_element_sibling(&self) -> Option<&HTMLElement> {
		let parent = self.parent()?;
		let mut seen = false;
		let self_ptr = self as *const HTMLElement;
		for child in &parent.children {
			if let Node::Element(e) = child {
				let ptr: *const HTMLElement = &**e;
				if seen {
					return Some(e);
				}
				if ptr == self_ptr {
					seen = true;
				}
			}
		}
		None
	}
	/// 上一个元素兄弟。
	pub fn previous_element_sibling(&self) -> Option<&HTMLElement> {
		let parent = self.parent()?;
		let self_ptr = self as *const HTMLElement;
		let mut prev: Option<&HTMLElement> = None;
		for child in &parent.children {
			if let Node::Element(e) = child {
				let ptr: *const HTMLElement = &**e;
				if ptr == self_ptr {
					return prev;
				}
				prev = Some(e);
			}
		}
		None
	}

	pub(super) fn index_in_parent(&self) -> Option<usize> {
		let parent = self.parent()?;
		let self_ptr = self as *const HTMLElement;
		for (i, child) in parent.children.iter().enumerate() {
			if let Node::Element(e) = child {
				let ptr: *const HTMLElement = &**e;
				if ptr == self_ptr {
					return Some(i);
				}
			}
		}
		None
	}

	/// 下一个兄弟节点（包含文本/注释）。
	pub fn next_sibling(&self) -> Option<&Node> {
		let parent = self.parent()?;
		let idx = self.index_in_parent()?;
		parent.children.get(idx + 1)
	}
	/// 上一个兄弟节点（包含文本/注释）。
	pub fn previous_sibling(&self) -> Option<&Node> {
		let parent = self.parent()?;
		let idx = self.index_in_parent()?;
		if idx == 0 {
			return None;
		}
		parent.children.get(idx - 1)
	}
	/// Insert nodes after this element.
	pub fn after_nodes(&mut self, mut nodes: Vec<Node>) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let idx = match self.index_in_parent() {
				Some(i) => i,
				None => return,
			} + 1;
			for n in nodes.iter_mut() {
				Self::detach_node(n);
			}
			for (i, mut n) in nodes.into_iter().enumerate() {
				if let Node::Element(ref mut e) = n {
					e.parent = Some(parent_ptr);
				}
				parent.children.insert(idx + i, n);
			}
		}
	}

	/// Prepend nodes to this element's children.
	pub fn prepend_nodes(&mut self, mut nodes: Vec<Node>) {
		for n in nodes.iter_mut() {
			Self::detach_node(n);
		}
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		for (i, n) in nodes.into_iter().enumerate() {
			self.children.insert(i, n);
		}
	}

	/// Append nodes to this element's children.
	pub fn append_nodes(&mut self, mut nodes: Vec<Node>) {
		for n in nodes.iter_mut() {
			Self::detach_node(n);
		}
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for mut n in nodes.into_iter() {
			if let Node::Element(ref mut e) = n {
				e.parent = Some(self_ptr);
			}
			self.children.push(n);
		}
	}

	fn detach_node(node: &mut Node) {
		match node {
			Node::Element(e) => {
				if let Some(parent_ptr) = e.parent {
					unsafe {
						let parent = &mut *parent_ptr;
						let elem_ptr: *const HTMLElement = &**e;
						parent.children.retain(|c| {
							if let Node::Element(pe) = c {
								(&**pe) as *const HTMLElement != elem_ptr
							} else {
								true
							}
						});
					}
					e.parent = None;
				}
			}
			Node::Text(_) | Node::Comment(_) => { /* no parent pointer tracking */ }
		}
	}

	/// 内部：在当前元素 children 指定 index 插入片段（仅用于自身 afterbegin/beforeend）。
	pub(super) fn insert_children_at(&mut self, index: usize, html: &str) {
		let mut nodes = parse_fragment(html);
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		for (i, n) in nodes.into_iter().enumerate() {
			self.children.insert(index + i, n);
		}
	}

	/// 内部：作为兄弟插入；after=true 表示 afterend
	pub(super) fn insert_as_sibling(&mut self, html: &str, after: bool) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let idx = match self.index_in_parent() {
				Some(i) => i,
				None => return,
			};
			let mut nodes = parse_fragment(html);
			for n in nodes.iter_mut() {
				if let Node::Element(e) = n {
					e.parent = Some(parent_ptr);
				}
			}
			let insert_pos = if after { idx + 1 } else { idx };
			for (i, n) in nodes.into_iter().enumerate() {
				parent.children.insert(insert_pos + i, n);
			}
		}
	}

	/// remove() 移除自身
	pub fn remove(&mut self) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let self_ptr = self as *const HTMLElement;
			parent.children.retain(|n| {
				if let Node::Element(e) = n {
					let ptr: *const HTMLElement = &**e;
					ptr != self_ptr
				} else {
					true
				}
			});
		}
	}
	/// before(html_fragment) 在自身前插入兄弟
	pub fn before(&mut self, html_fragment: &str) {
		self.before_items(&[NodeOrStr::Str(CowStr(html_fragment))]);
	}
	pub fn before_items(&mut self, items: &[NodeOrStr]) {
		self.insert_sibling_items(items, false);
	}
	/// after(html_fragment) 在自身后插入兄弟
	pub fn after(&mut self, html_fragment: &str) {
		self.after_items(&[NodeOrStr::Str(CowStr(html_fragment))]);
	}
	pub fn after_items(&mut self, items: &[NodeOrStr]) {
		self.insert_sibling_items(items, true);
	}
	/// 由于当前结构不存 parent 指针，需要传入 root，用 DFS 寻找祖先链。
	pub fn closest_in<'a>(
		&'a self,
		root: &'a HTMLElement,
		selector: &str,
	) -> Option<&'a HTMLElement> {
		// 1. 若自身匹配，直接返回
		if self.matches_selector(root, selector) {
			return Some(self);
		}
		// 2. 构造从 root 到 self 的路径，再从倒数第二个往上测试
		let target_ptr = self as *const HTMLElement;
		let mut stack: Vec<&HTMLElement> = Vec::new();
		fn dfs<'b>(
			cur: &'b HTMLElement,
			target: *const HTMLElement,
			path: &mut Vec<&'b HTMLElement>,
		) -> bool {
			path.push(cur);
			let cur_ptr = cur as *const HTMLElement;
			if cur_ptr == target {
				return true;
			}
			for child in cur.children.iter() {
				if let Node::Element(e) = child {
					if dfs(e, target, path) {
						return true;
					}
				}
			}
			path.pop();
			false
		}
		if !dfs(root, target_ptr, &mut stack) {
			return None;
		}
		// stack 包含 root..self，去掉最后一个 self，从上往下倒序（接近 self 的祖先优先）
		for ancestor in stack[..stack.len() - 1].iter().rev() {
			if ancestor.matches_selector(root, selector) {
				return Some(ancestor);
			}
		}
		None
	}
	pub fn prepend_child(&mut self, node: Node) {
		let mut n = node;
		if let Node::Element(ref mut e) = n {
			let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
			e.parent = Some(self_ptr);
		}
		self.children.insert(0, n);
	}
	/// JS 风格 appendChild(Element) -> 返回子元素可变引用以便链式操作
	pub fn append_child_element(&mut self, child: HTMLElement) -> &mut HTMLElement {
		let mut boxed = Box::new(child);
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		boxed.parent = Some(self_ptr);
		self.children.push(Node::Element(boxed));
		match self.children.last_mut().unwrap() {
			Node::Element(ref mut e) => e,
			_ => unreachable!(),
		}
	}
	/// JS appendChild(TextNode) 等价：直接加入文本节点
	pub fn append_child_text(&mut self, text: &str) {
		use crate::dom::text::TextNode;
		self.children
			.push(Node::Text(TextNode::new(text.to_string())));
	}
	pub fn remove_children_where<F: FnMut(&Node) -> bool>(&mut self, mut f: F) {
		self.children.retain(|n| !f(n));
	}

	/// JS 兼容别名：attributes() => 已解码的小写属性映射。

	pub fn set_content_node(&mut self, node: Node) {
		self.set_content_nodes(vec![node]);
	}
	/// --- Variadic mutation helpers (支持字符串与已存在 Node) ---
	pub fn append(&mut self, html_fragment: &str) {
		self.append_items(&[NodeOrStr::Str(CowStr(html_fragment))]);
	}
	pub fn prepend(&mut self, html_fragment: &str) {
		self.prepend_items(&[NodeOrStr::Str(CowStr(html_fragment))]);
	}
	pub fn append_items(&mut self, items: &[NodeOrStr]) {
		let nodes = collect_items(items);
		self.adopt_vec(nodes, None);
	}
	pub fn prepend_items(&mut self, items: &[NodeOrStr]) {
		let nodes = collect_items(items);
		self.adopt_vec(nodes, Some(0));
	}
	pub(super) fn insert_sibling_items(&mut self, items: &[NodeOrStr], after: bool) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let idx = match self.index_in_parent() {
				Some(i) => i,
				None => return,
			};
			let mut nodes = collect_items(items);
			for n in nodes.iter_mut() {
				if let Node::Element(e) = n {
					e.parent = Some(parent_ptr);
				}
			}
			let insert_pos = if after { idx + 1 } else { idx };
			for (i, n) in nodes.into_iter().enumerate() {
				parent.children.insert(insert_pos + i, n);
			}
		}
	}

	fn adopt_vec(&mut self, mut nodes: Vec<Node>, at: Option<usize>) {
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		match at {
			Some(pos) => {
				for (i, n) in nodes.into_iter().enumerate() {
					self.children.insert(pos + i, n);
				}
			}
			None => self.children.extend(nodes),
		}
	}
	/// 获取下一个元素兄弟（模拟 JS nextElementSibling），需要 root (当前无 parent 指针)。
	pub fn next_element_sibling_in<'a>(&'a self, root: &'a HTMLElement) -> Option<&'a HTMLElement> {
		let target = self as *const HTMLElement;
		// DFS 找到父节点
		fn find_parent<'b>(
			cur: &'b HTMLElement,
			target: *const HTMLElement,
		) -> Option<&'b HTMLElement> {
			for child in &cur.children {
				if let Node::Element(e) = child {
					let ptr: *const HTMLElement = &**e;
					if ptr == target {
						return Some(cur);
					}
					if let Some(p) = find_parent(&**e, target) {
						return Some(p);
					}
				}
			}
			None
		}
		let parent = find_parent(root, target)?;
		let mut seen = false;
		for child in &parent.children {
			if let Node::Element(e) = child {
				let ptr: *const HTMLElement = &**e;
				if seen {
					return Some(&**e);
				}
				if ptr == target {
					seen = true;
				}
			}
		}
		None
	}

	/// 获取上一个元素兄弟（模拟 JS previousElementSibling）。
	pub fn previous_element_sibling_in<'a>(
		&'a self,
		root: &'a HTMLElement,
	) -> Option<&'a HTMLElement> {
		let target = self as *const HTMLElement;
		fn find_parent<'b>(
			cur: &'b HTMLElement,
			target: *const HTMLElement,
		) -> Option<&'b HTMLElement> {
			for child in &cur.children {
				if let Node::Element(e) = child {
					let ptr: *const HTMLElement = &**e;
					if ptr == target {
						return Some(cur);
					}
					if let Some(p) = find_parent(&**e, target) {
						return Some(p);
					}
				}
			}
			None
		}
		let parent = find_parent(root, target)?;
		let mut prev: Option<&HTMLElement> = None;
		for child in &parent.children {
			if let Node::Element(e) = child {
				let ptr: *const HTMLElement = &**e;
				if ptr == target {
					return prev;
				}
				prev = Some(&**e);
			}
		}
		None
	}

	pub fn append_child(&mut self, node: Node) {
		let mut n = node;
		// 若节点已有父，先从旧父移除（与 JS appendChild 行为一致）
		match &mut n {
			Node::Element(e) => {
				if let Some(parent_ptr) = e.parent {
					unsafe {
						let parent = &mut *parent_ptr;
						let elem_ptr: *const HTMLElement = &**e;
						parent.children.retain(|c| {
							if let Node::Element(pe) = c {
								(&**pe) as *const HTMLElement != elem_ptr
							} else { true }
						});
					}
					e.parent = None; // 临时清空防止错误引用
				}
				let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
				e.parent = Some(self_ptr);
			},
			Node::Text(_) | Node::Comment(_) => { /* Text/Comment 按当前实现不跟踪 parent 指针 */ }
		}
		self.children.push(n);
	}
	pub fn first_child(&self) -> Option<&Node> {
		self.children.first()
	}

	pub fn last_child(&self) -> Option<&Node> {
		self.children.last()
	}

	pub fn first_element_child(&self) -> Option<&HTMLElement> {
		for c in &self.children {
			if let Node::Element(e) = c {
				return Some(e);
			}
		}
		None
	}
	pub fn last_element_child(&self) -> Option<&HTMLElement> {
		for c in self.children.iter().rev() {
			if let Node::Element(e) = c {
				return Some(e);
			}
		}
		None
	}
	pub fn first_element_child_mut(&mut self) -> Option<&mut HTMLElement> {
		for child in self.children.iter_mut() {
			if let Node::Element(e) = child {
				return Some(e);
			}
		}
		None
	}

	/// 返回父元素引用（只读）。
	pub fn parent(&self) -> Option<&HTMLElement> {
		self.parent.map(|p| unsafe { &*p })
	}
	pub fn children_elements(&self) -> Vec<&HTMLElement> {
		self.children
			.iter()
			.filter_map(|c| {
				if let Node::Element(e) = c {
					Some(&**e)
				} else {
					None
				}
			})
			.collect()
	}
	pub fn child_element_count(&self) -> usize {
		self.children
			.iter()
			.filter(|c| matches!(c, Node::Element(_)))
			.count()
	}
}
