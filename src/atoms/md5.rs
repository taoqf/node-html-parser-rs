use md5::compute;

#[allow(dead_code)]
pub(crate) fn md5<T: AsRef<[u8]>>(input: T) -> String {
	let str = compute(input);
	let hash = format!("{:x}", str);
	return hash;
}
