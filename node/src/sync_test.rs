use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::{
	io::{BufRead, BufReader},
	process::{Child, Command, Stdio},
	thread,
	time::{Duration, Instant},
};

const RPC_PORT: u32 = 9933;
const TIMEOUT: Duration = Duration::from_secs(180);

fn is_synced() -> bool {
	let request_body = json!({
		"jsonrpc": "2.0",
		"method": "system_health",
		"params": [],
		"id": 1
	});

	let client = Client::new();
	let response = match client
		.post(format!("http://127.0.0.1:{}", RPC_PORT))
		.header("Content-Type", "application/json")
		.json(&request_body)
		.send()
	{
		Ok(resp) => resp,
		Err(err) => {
			eprintln!("[rpc] Node is not ready or connection failed: {err}");
			// By returning `false` here, we treat "no response" as "still syncing"
			return false;
		},
	};

	// Parse JSON or assume "still syncing" on failure
	let response_json: Value = match response.json() {
		Ok(json) => json,
		Err(err) => {
			eprintln!("[rpc] Failed to parse JSON: {err}");
			return false;
		},
	};

	!response_json["result"]["isSyncing"].as_bool().unwrap_or(true)
}

fn spawn_laos_warp() -> Child {
	let laos_bin = std::env::current_dir()
        .expect("the directory node exists and it doesn't have restricted permissions, this shouldn't fail;qed;")
        .parent()
        .expect("the root directory of the repo exists and it doesn't have restricted permissions, this shouldn't fail;qed;")
        .join("target")
        .join("release")
        .join("laos");

	if !laos_bin.exists() {
		panic!("Executable not found: {:?}", laos_bin);
	}
	Command::new(laos_bin)
		.args([
			"--sync=warp",
			"--chain=laos",
			"--tmp",
			"--relay-chain-light-client",
			&format!("--rpc-port={}", RPC_PORT),
		])
		.stderr(Stdio::piped())
		.spawn()
		.expect("Failed to spawn laos process.")
}

#[test]
fn warp_sync() {
	let mut child = spawn_laos_warp();

	let stderr = child.stderr.take().expect("No stderr available");
	// This might be useful when activating logs within the test by adding "nocapture"
	// cargo test -- --nocapture
	let handle_stderr = thread::spawn(move || {
		let reader = BufReader::new(stderr);
		for line in reader.lines() {
			if let Ok(line) = line {
				println!("[laos-node] {}", line);
			}
		}
	});

	let start = Instant::now();

	while !is_synced() && start.elapsed() < TIMEOUT {
		thread::sleep(Duration::from_secs(5));
	}
	let synced = is_synced();

	let _ = child.kill();
	let _ = handle_stderr.join();

	assert!(synced, "Node was not fully synced within the timeout");
}
