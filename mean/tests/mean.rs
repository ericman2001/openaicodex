use std::io::Cursor;
use mean::{mean_from_file, mean_from_reader};

#[test]
fn computes_mean_correctly() {
    let data = b"1\n2\n3\n4\n";
    let cursor = Cursor::new(&data[..]);
    let mean = mean_from_reader(cursor).expect("should compute mean");
    assert!((mean - 2.5).abs() < f64::EPSILON);
}

#[test]
fn invalid_file_returns_error() {
    let result = mean_from_file("does_not_exist.txt");
    assert!(result.is_err());
}

#[test]
fn empty_input_returns_error() {
    let cursor = Cursor::new(b"   \n   \n");
    let result = mean_from_reader(cursor);
    assert!(result.is_err());
}
