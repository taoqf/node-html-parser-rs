//! Attribute normalization utilities for HTML elements.

/// 规范化属性串中的引号，统一转换为双引号格式并正确转义内部双引号
pub fn normalize_attr_quotes(attrs_with_leading_space: &str, force_normalize: bool) -> String {
	let s = attrs_with_leading_space;
	let bytes = s.as_bytes();
	let mut i = 0;
	let mut out = String::with_capacity(s.len() + 16);
	while i < bytes.len() {
		// 拷贝前导空白和分隔符
		if bytes[i].is_ascii_whitespace() {
			out.push(bytes[i] as char);
			i += 1;
			continue;
		}
		// 读取属性名
		let key_start = i;
		while i < bytes.len() {
			let c = bytes[i] as char;
			if c.is_ascii_alphanumeric() || matches!(c, '_' | ':' | '.' | '-') {
				i += 1;
			} else {
				break;
			}
		}
		if key_start == i {
			// 非属性字符，直接复制
			out.push(bytes[i] as char);
			i += 1;
			continue;
		}
		let key = &s[key_start..i];
		// 跳过属性名后的空白
		let mut j = i;
		while j < bytes.len() && bytes[j].is_ascii_whitespace() {
			j += 1;
		}
		// 若无 '=' 则视作布尔属性
		if j >= bytes.len() || bytes[j] != b'=' {
			// 记录 key 后的空白长度
			let ws_len = j - i;
			out.push_str(key);
			// 若后面还有其它属性（j < len 且不是 '>' 或 '/'），且存在原空白，则保留一个空格分隔
			if ws_len > 0 && j < bytes.len() {
				let next = bytes[j] as char;
				if next != '>' && next != '/' {
					out.push(' ');
				}
			}
			i = j;
			continue;
		}
		j += 1; // 跳过 '='
		while j < bytes.len() && bytes[j].is_ascii_whitespace() {
			j += 1;
		}
		out.push_str(key);
		out.push('=');
		if j >= bytes.len() {
			break;
		}
		let c = bytes[j] as char;
		if c == '"' || c == '\'' {
			// 提取属性值
			j += 1;
			let val_start = j;
			while j < bytes.len() && bytes[j] as char != c {
				j += 1;
			}
			let val = &s[val_start..j];

			if force_normalize {
				// 统一转换为双引号格式，并转义内部双引号
				out.push('"');
				if c == '\'' && val.contains('"') {
					// 单引号值内包含双引号需要转义
					out.push_str(&val.replace('"', "&quot;"));
				} else {
					out.push_str(val);
				}
				out.push('"');
			} else {
				// 保留原始引号风格
				out.push(c);
				out.push_str(val);
				out.push(c);
			}

			if j < bytes.len() {
				j += 1; // 跳过结束引号
			}
			i = j;
			continue;
		} else {
			// 无引号
			let val_start = j;
			while j < bytes.len() {
				let cc = bytes[j] as char;
				if cc.is_ascii_whitespace() || matches!(cc, '>' | '/' | '"' | '\'' | '=') {
					break;
				}
				j += 1;
			}
			out.push('"');
			out.push_str(&s[val_start..j]);
			out.push('"');
			i = j;
			continue;
		}
	}
	out
}
