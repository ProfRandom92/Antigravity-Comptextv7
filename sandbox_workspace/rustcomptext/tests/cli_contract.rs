use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_target_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target");
    p.push(name);
    p
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "rustcomptext", "--", "--help"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Check all subcommands are visible
    assert!(stdout.contains("compress"));
    assert!(stdout.contains("inspect"));
    assert!(stdout.contains("verify"));
    assert!(stdout.contains("replay"));
    assert!(stdout.contains("adversarial-test"));
}

#[test]
fn test_cli_compress_success() {
    let input_path = get_target_path("cli_test_input.txt");
    let output_path = get_target_path("cli_test_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output_path.exists());

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_compress_non_utf8_fails() {
    let input_path = get_target_path("cli_test_input_invalid.txt");
    let output_path = get_target_path("cli_test_output_invalid.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    // Invalid UTF-8 bytes
    fs::write(&input_path, vec![0, 159, 146, 150]).unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(!output_path.exists());

    let _ = fs::remove_file(input_path);
}

#[test]
fn test_cli_inspect_success() {
    let input_path = get_target_path("cli_inspect_input.txt");
    let output_path = get_target_path("cli_inspect_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress first
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Inspect
    let inspect_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "inspect",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(inspect_output.status.success());
    let stdout = String::from_utf8(inspect_output.stdout).unwrap();

    assert!(stdout.contains("version: 1\n"));
    assert!(stdout.contains("entries: 1\n"));
    assert!(stdout.contains("operations:\n- 0: compress.plain.v1\n"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_inspect_invalid_json_fails() {
    let invalid_path = get_target_path("cli_inspect_invalid.ctx");
    let _ = fs::remove_file(&invalid_path);

    fs::write(&invalid_path, b"{invalid json}").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "inspect",
            invalid_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let _ = fs::remove_file(invalid_path);
}

#[test]
fn test_cli_verify_success() {
    let input_path = get_target_path("cli_verify_input.txt");
    let output_path = get_target_path("cli_verify_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress first
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Verify
    let verify_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "verify",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(verify_output.status.success());
    let stdout = String::from_utf8(verify_output.stdout).unwrap();
    assert!(stdout.contains("verify: ok\n") || stdout.contains("verify: ok\r\n"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_verify_tampered_fails() {
    let input_path = get_target_path("cli_verify_tamper_input.txt");
    let output_path = get_target_path("cli_verify_tamper_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Tamper the file contents (e.g. change the payload)
    let json_content = fs::read_to_string(&output_path).unwrap();
    let mut package: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    package["payload"] = serde_json::Value::String("Tampered payload".to_string());
    let tampered_json = serde_json::to_string(&package).unwrap();
    fs::write(&output_path, tampered_json.as_bytes()).unwrap();

    // Verify
    let verify_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "verify",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!verify_output.status.success());
    let stderr = String::from_utf8(verify_output.stderr).unwrap();
    assert!(stderr.contains("verify: failed"));
    assert!(stderr.contains("reason:"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_replay_success() {
    let input_path = get_target_path("cli_replay_input.txt");
    let output_path = get_target_path("cli_replay_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Replay
    let replay_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "replay",
            output_path.to_str().unwrap(),
            "--steps",
            "1000",
        ])
        .output()
        .unwrap();

    assert!(replay_output.status.success());
    let stdout = String::from_utf8(replay_output.stdout).unwrap();
    assert!(stdout.contains("replay: ok\n") || stdout.contains("replay: ok\r\n"));
    assert!(stdout.contains("steps_requested: 1000"));
    assert!(stdout.contains("steps_replayed: 1"));
    assert!(stdout.contains("ledger_root:"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_replay_tampered_fails() {
    let input_path = get_target_path("cli_replay_tamper_input.txt");
    let output_path = get_target_path("cli_replay_tamper_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Tamper payload
    let json_content = fs::read_to_string(&output_path).unwrap();
    let mut package: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    package["payload"] = serde_json::Value::String("Tampered payload".to_string());
    let tampered_json = serde_json::to_string(&package).unwrap();
    fs::write(&output_path, tampered_json.as_bytes()).unwrap();

    // Replay
    let replay_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "replay",
            output_path.to_str().unwrap(),
            "--steps",
            "1000",
        ])
        .output()
        .unwrap();

    assert!(!replay_output.status.success());
    let stderr = String::from_utf8(replay_output.stderr).unwrap();
    assert!(stderr.contains("replay: failed"));
    assert!(stderr.contains("reason:"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_adversarial_success() {
    let input_path = get_target_path("cli_adv_input.txt");
    let output_path = get_target_path("cli_adv_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Adversarial-test
    let adv_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "adversarial-test",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(adv_output.status.success());
    let stdout = String::from_utf8(adv_output.stdout).unwrap();
    assert!(
        stdout.contains("adversarial-test: ok\n") || stdout.contains("adversarial-test: ok\r\n")
    );
    assert!(stdout.contains("tamper cases detected: 15/15"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_cli_adversarial_invalid_fails() {
    let input_path = get_target_path("cli_adv_invalid_input.txt");
    let output_path = get_target_path("cli_adv_invalid_output.ctx");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    fs::write(&input_path, b"Hallo Alex\n").unwrap();

    // Compress
    let compress_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "compress",
            input_path.to_str().unwrap(),
            "--out",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(compress_output.status.success());

    // Tamper payload
    let json_content = fs::read_to_string(&output_path).unwrap();
    let mut package: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    package["payload"] = serde_json::Value::String("Tampered payload".to_string());
    let tampered_json = serde_json::to_string(&package).unwrap();
    fs::write(&output_path, tampered_json.as_bytes()).unwrap();

    // Adversarial-test
    let adv_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "rustcomptext",
            "--",
            "adversarial-test",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!adv_output.status.success());
    let stderr = String::from_utf8(adv_output.stderr).unwrap();
    assert!(stderr.contains("adversarial-test: failed"));
    assert!(stderr.contains("reason:"));

    // Clean up
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
}
