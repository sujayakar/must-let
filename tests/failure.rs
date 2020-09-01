use must_let::must_let;

#[test]
#[should_panic(expected = "Expected Ok(_), received Err(4)")]
fn test_enum() {
    must_let!(let Ok(_) = Err::<usize, _>(4));
}
