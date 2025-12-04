use crate::run_godot_arch;

#[test]
fn run_e2e_tests() {
    let result = run_godot_arch(
        "./src/tests/e2e/e2e-test-projects/project_a/godot-arch.config.yaml",
        "./src/tests/e2e/e2e-test-projects/project_a/",
        None,
    );
    assert!(result.is_ok(), "run_godot_arch failed: {:?}", result.err());
    if let Ok(result) = result {
        assert_eq!(result.files_failed, 0);
        assert!(result.files_tested > 0);
    }
}
