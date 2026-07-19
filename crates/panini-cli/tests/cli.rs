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
