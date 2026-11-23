use csb_demo::process_batch;

#[test]
fn errors_include_original_line_numbers() {
    let message = process_batch(&b"west,1,4\n\nbad\n"[..]).unwrap_err();
    assert!(message.starts_with("line 3:"));
}

#[test]
fn conflicting_duplicate_aborts_before_output() {
    assert!(process_batch(&b"a,1,8\nb,2,8\na,1,9\n"[..]).is_err());
}

#[test]
fn duplicate_replays_do_not_change_summary() {
    let unique = process_batch(&b"a,10,12\nb,22,14\na,9,-1\n"[..]).unwrap();
    let replayed = process_batch(&b"b,22,14\na,9,-1\na,10,12\na,9,-1\n"[..]).unwrap();
    assert_eq!(unique, replayed);
    assert_eq!(unique.lines().count(), 3);
}
