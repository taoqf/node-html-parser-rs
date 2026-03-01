//! HTML属性解析相关功能

/// 🚀 高效的手写属性解析器，替代正则表达式
/// 解析属性字符串，只提取id和class属性，其他标记为存在
pub fn parse_id_class_attrs_fast(attr_part: &str) -> (Vec<(String, String)>, bool) {
	let mut attrs = Vec::with_capacity(4);
	let mut saw_other_attr = false;

	if attr_part.is_empty() {
		return (attrs, saw_other_attr);
	}

	let bytes = attr_part.as_bytes();
	let mut i = 0;

	while i < bytes.len() {
		// 跳过空白
		while i < bytes.len() && bytes[i].is_ascii_whitespace() {
			i += 1;
		}
		if i >= bytes.len() {
			break;
		}

		// 读取属性名
		let name_start = i;
		while i < bytes.len() {
			let b = bytes[i];
			if b.is_ascii_alphanumeric()
				|| b == b'_' || b == b':'
				|| b == b'.' || b == b'-'
				|| b == b'(' || b == b')'
				|| b == b'[' || b == b']'
				|| b == b'#' || b == b'@'
				|| b == b'$' || b == b'?'
			{
				i += 1;
			} else {
				break;
			}
		}

		if name_start == i {
			// 不是有效的属性名，跳过这个字符
			i += 1;
			continue;
		}

		let attr_name = &attr_part[name_start..i];
		let lower_name = attr_name.to_lowercase();

		// 跳过空白
		while i < bytes.len() && bytes[i].is_ascii_whitespace() {
			i += 1;
		}

		// 检查是否有等号
		if i < bytes.len() && bytes[i] == b'=' {
			i += 1; // 跳过等号

			// 跳过空白
			while i < bytes.len() && bytes[i].is_ascii_whitespace() {
				i += 1;
			}

			// 读取属性值
			let value = if i < bytes.len() {
				if bytes[i] == b'"' {
					// 双引号值
					i += 1;
					let value_start = i;
					while i < bytes.len() && bytes[i] != b'"' {
						i += 1;
					}
					let value = &attr_part[value_start..i];
					if i < bytes.len() {
						i += 1;
					} // 跳过结束引号
					value
				} else if bytes[i] == b'\'' {
					// 单引号值
					i += 1;
					let value_start = i;
					while i < bytes.len() && bytes[i] != b'\'' {
						i += 1;
					}
					let value = &attr_part[value_start..i];
					if i < bytes.len() {
						i += 1;
					} // 跳过结束引号
					value
				} else {
					// 无引号值
					let value_start = i;
					while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
						i += 1;
					}
					&attr_part[value_start..i]
				}
			} else {
				""
			};

			// 只处理id和class属性
			if lower_name == "id" || lower_name == "class" {
				let decoded = html_escape::decode_html_entities(value);
				attrs.push((lower_name, decoded.into_owned()));
			} else {
				saw_other_attr = true;
			}
		} else {
			// 布尔属性（无值）
			if lower_name == "id" || lower_name == "class" {
				attrs.push((lower_name, String::new()));
			} else {
				saw_other_attr = true;
			}
		}
	}

	(attrs, saw_other_attr)
}
