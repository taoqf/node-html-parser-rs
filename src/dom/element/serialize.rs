use super::main::HTMLElement; // temporarily rely on full struct defined in all.rs
use super::normalize_attr_quotes; // make helper public in all.rs first

impl HTMLElement {
	pub fn outer_html(&self) -> String {
		if self.is_root() {
			return self.inner_html();
		} // root container unwrap
		let tag = self.name();
		let attrs = if self.raw_attrs.is_empty() {
			String::new()
		} else {
			format!(" {}", self.raw_attrs.trim())
		};
		if self.is_void {
			if self.void_add_slash {
				// JS VoidTag.formatNode: 若 addClosingSlash 且存在属性且 attrs 不以空格结尾，补一个空格再加 '/'
				// 这里 attrs 变量自身已经带前导空格（当非空）。需要确保 '/' 前存在恰当空格：
				// 1. 无属性: <br/>
				// 2. 有属性且最后不是空格: <img src="x.png" />
				// 3. 若 attrs 末尾已经是空格（极少数构造），直接拼接 '/'
				let mut norm_attrs = if attrs.is_empty() {
					String::new()
				} else {
					normalize_attr_quotes(&attrs)
				};
				// 局部策略：为 void 且需要加 '/' 的情况下，尽量将单引号属性值改成双引号，以匹配 JS 测试（img src="x.png" />）。
				if !norm_attrs.is_empty() {
					let mut converted = String::with_capacity(norm_attrs.len());
					let nb = norm_attrs.as_bytes();
					let mut i = 0;
					while i < nb.len() {
						let c = nb[i] as char;
						if c == '\'' {
							// attribute value shouldn't start with quote directly; safe fallback
							converted.push(c);
							i += 1;
							continue;
						}
						// Detect pattern ='<value>' and convert
						if c == '=' && i + 1 < nb.len() && nb[i + 1] as char == '\'' {
							converted.push('=');
							converted.push('"');
							i += 2; // skip ='
							let start = i;
							while i < nb.len() && nb[i] as char != '\'' {
								i += 1;
							}
							let val = &norm_attrs[start..i];
							// if value contains double quotes, fallback to single quotes original
							if val.contains('"') {
								converted.push('\'');
								converted.push_str(val);
								converted.push('\'');
							} else {
								converted.push_str(val);
								converted.push('"');
							}
							if i < nb.len() && nb[i] as char == '\'' {
								i += 1;
							}
							continue;
						}
						converted.push(c);
						i += 1;
					}
					norm_attrs = converted;
				}
				if norm_attrs.is_empty() {
					format!("<{}{}{}>", tag, norm_attrs, "/")
				} else if norm_attrs.ends_with(' ') {
					format!("<{}{}{}>", tag, norm_attrs, "/")
				} else {
					format!("<{}{} {}>", tag, norm_attrs, "/")
				}
			} else {
				let norm_attrs = if attrs.is_empty() {
					String::new()
				} else {
					normalize_attr_quotes(&attrs)
				};
				format!("<{}{}>", tag, norm_attrs)
			}
		} else {
			// 对于非 void 元素，也需要规范化属性引号并确保属性间有空格
			let norm_attrs = if attrs.is_empty() {
				String::new()
			} else {
				normalize_attr_quotes(&attrs)
			};
			// 兼容 JS 行为：如果该元素是被后续标签自动闭合，且本身没有子节点（空），原输入中不存在显式关闭标签，
			// 则保留原样仅输出起始标签（如 <ul><li><li></ul> 中的两个 <li>）。
			let auto_closed_empty = match self.range() {
				Some((s, e)) if s == e && self.children.is_empty() => true,
				_ => false,
			};
			if auto_closed_empty {
				format!("<{}{}>", tag, norm_attrs)
			} else {
				format!("<{}{}>{}</{}>", tag, norm_attrs, self.inner_html(), tag)
			}
		}
	}
	/// JS API 等价：toString() => outerHTML (保留显式方法以便测试调用)
	pub fn to_string(&self) -> String {
		self.outer_html()
	}
}
