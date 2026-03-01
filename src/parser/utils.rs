//! 解析器辅助工具函数

/// 高效的大小写不敏感字符串搜索，避免创建整个文档的小写副本
/// 返回匹配位置的索引，如果未找到则返回 None
pub fn find_closing_tag_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
	// 对于短的 needle，逐字符比较比创建副本更高效
	if needle.is_empty() {
		return Some(0);
	}

	let needle_bytes = needle.as_bytes();
	let haystack_bytes = haystack.as_bytes();

	if needle_bytes.len() > haystack_bytes.len() {
		return None;
	}

	// 使用字节级别的大小写不敏感比较
	for i in 0..=haystack_bytes.len() - needle_bytes.len() {
		let mut matches = true;
		for j in 0..needle_bytes.len() {
			let h_byte = haystack_bytes[i + j];
			let n_byte = needle_bytes[j];

			// ASCII 大小写不敏感比较
			let h_lower = if h_byte >= b'A' && h_byte <= b'Z' {
				h_byte + 32 // 转小写
			} else {
				h_byte
			};
			let n_lower = if n_byte >= b'A' && n_byte <= b'Z' {
				n_byte + 32 // 转小写
			} else {
				n_byte
			};

			if h_lower != n_lower {
				matches = false;
				break;
			}
		}
		if matches {
			return Some(i);
		}
	}
	None
}

/// 🚀 优化的自闭合标记检测，避免不必要的字符数组分配
/// 修正策略：从右向左扫描，若发现未在引号内的末尾 '/'，则把它视作自闭合标记并剥离。
pub fn strip_trailing_self_close_optimized(s: &str) -> (String, bool) {
	if s.is_empty() {
		return (s.to_string(), false);
	}

	let bytes = s.as_bytes();
	let mut idx = bytes.len();

	// 跳过尾部空白
	while idx > 0 && bytes[idx - 1].is_ascii_whitespace() {
		idx -= 1;
	}

	// 检查是否以 '/' 结尾
	if idx > 0 && bytes[idx - 1] == b'/' {
		// 简单情况：如果没有引号，直接剥离
		if !s.contains('"') && !s.contains('\'') {
			let cleaned = s[..idx - 1].to_string();
			return (cleaned, true);
		}

		// 复杂情况：需要检查引号状态
		let mut in_single = false;
		let mut in_double = false;

		// 正向扫描到 '/' 位置
		for i in 0..(idx - 1) {
			match bytes[i] {
				b'"' if !in_single => in_double = !in_double,
				b'\'' if !in_double => in_single = !in_single,
				_ => {}
			}
		}

		// 如果 '/' 不在引号内，则剥离
		if !in_single && !in_double {
			let cleaned = s[..idx - 1].to_string();
			return (cleaned, true);
		}
	}

	(s.to_string(), false)
}
