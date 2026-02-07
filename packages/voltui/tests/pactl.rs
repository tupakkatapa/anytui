use voltui::extract_sink_name;

#[test]
fn test_extract_sink_name_simple() {
    assert_eq!(extract_sink_name("sink_name=my_sink"), "my_sink");
}

#[test]
fn test_extract_sink_name_quoted() {
    assert_eq!(extract_sink_name("sink_name=\"my sink\""), "my sink");
    assert_eq!(extract_sink_name("sink_name='my sink'"), "my sink");
}

#[test]
fn test_extract_sink_name_with_other_args() {
    assert_eq!(
        extract_sink_name("slaves=a,b sink_name=combined_out adjust_time=3"),
        "combined_out"
    );
}

#[test]
fn test_extract_sink_name_missing() {
    assert_eq!(extract_sink_name("slaves=a,b"), "combined");
    assert_eq!(extract_sink_name(""), "combined");
}

#[test]
fn test_extract_sink_name_with_tabs() {
    assert_eq!(
        extract_sink_name("slaves=a\tsink_name=test\tadjust=1"),
        "test"
    );
}
