use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const RPC_PORT: u32 = 9933;
const TIMEOUT: Duration = Duration::from_secs(80); // It usually takes around 60 seconds to sync

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
			// By returning `true` here, we treat "no response" as "still syncing"
			return false;
		},
	};

	Parse JSON or assume "still syncing" on failure
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
	let laos_bin = std::path::Path::new("..").join("target").join("release").join("laos");

	// Ensure the path is correct
	if !laos_bin.exists() {
		panic!("Executable not found: {:?}", laos_bin);
	}
	Command::new(laos_bin)
		.args(&[
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

	child.kill();
	handle_stderr.join();

	assert!(synced, "Node was not fully synced within the timeout.");
}
