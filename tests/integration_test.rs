use std::process::Command;

#[test]
fn test_build_example() {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg("simple_shape")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Build failed");
}