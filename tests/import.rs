use std::path::PathBuf;
use std::process::Command;

fn node_available() -> bool {
	Command::new("node").arg("--version").output().is_ok()
}

fn run_node_cmd(cwd: PathBuf, args: &[&str]) -> std::io::Result<std::process::Output> {
	Command::new("node").args(args).current_dir(cwd).output()
}

#[test]
fn import_esm_and_cjs() {
	let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let esm_dir = repo_root
		.join("test")
		.join("tests")
		.join("assets")
		.join("packages")
		.join("esm");
	let cjs_dir = repo_root
		.join("test")
		.join("tests")
		.join("assets")
		.join("packages")
		.join("cjs");

	if !node_available() || !esm_dir.exists() || !cjs_dir.exists() {
		eprintln!("skipping import tests: node or example packages not available");
		return;
	}

	// ESM
	let out = run_node_cmd(esm_dir, &["--loader", "ts-node/esm", "index.ts"])
		.expect("failed to run node for esm");
	assert!(
		out.status.success(),
		"ESM command failed: {}",
		String::from_utf8_lossy(&out.stderr)
	);
	assert_eq!(String::from_utf8_lossy(&out.stdout), "parse succeeded\n");

	// CommonJS
	let out2 = run_node_cmd(cjs_dir, &["-r", "ts-node/register", "index.ts"])
		.expect("failed to run node for cjs");
	assert!(
		out2.status.success(),
		"CJS command failed: {}",
		String::from_utf8_lossy(&out2.stderr)
	);
	assert_eq!(String::from_utf8_lossy(&out2.stdout), "parse succeeded\n");
}
