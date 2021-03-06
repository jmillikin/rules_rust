#[test]
fn test() {
    // our source file should be readable
    let source_file = std::fs::read_to_string(env!("SOURCE_FILE")).unwrap();
    assert_eq!(source_file, "source\n");
    // our generated data file should be readable
    let generated_data = std::fs::read_to_string(env!("GENERATED_DATA")).unwrap();
    assert_eq!(generated_data, "hello\n");
    // and we should be able to read (and thus execute) our tool
    assert_eq!(std::fs::read(env!("SOME_TOOL")).unwrap().is_empty(), false);
}
