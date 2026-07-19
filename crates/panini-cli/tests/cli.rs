use std::process::Command;

fn run(args: &[&str]) -> (String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_panini"))
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).to_string(),
        out.status.code().unwrap_or(-1),
    )
}

#[test]
fn check_valid_word_exits_zero() {
    let (stdout, code) = run(&["check", "bhavati"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("VALID"));
}

#[test]
fn check_invalid_word_exits_one() {
    let (_stdout, code) = run(&["check", "xyzq"]);
    assert_eq!(code, 1);
}

#[test]
fn trace_flag_lists_sutras() {
    let (stdout, _code) = run(&["check", "bhavati", "--trace"]);
    assert!(stdout.contains("3.1.68"));
}

#[test]
fn in_iast_scheme_is_honored() {
    let (stdout, code) = run(&["check", "bhavati", "--in", "iast"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("VALID"));
}

#[test]
fn in_slp1_scheme_is_honored() {
    // Already-SLP1 input, explicit scheme: the declared scheme is
    // authoritative and must not be second-guessed by auto-detection.
    let (stdout, code) = run(&["check", "Bavati", "--in", "slp1"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("VALID"));
}
