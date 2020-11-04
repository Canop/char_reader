use {
    super::*,
    std::io,
};

static TEXT: &str = "Comunicações\n概\n要éléphants blancs\nالعاشر ليونيكود\n\nhi?\n";
static INVALID: &[u8] = &[b'a', 0xff, 0x4a]; // invalid as UTF8

#[test]
fn test_chars(){
    let bytes = TEXT.as_bytes(); // a Read, mocking a file or stream
    let mut reader = CharReader::new(bytes);
    for str_char in TEXT.chars() {
        let read_char = reader.next_char().unwrap();
        assert_eq!(read_char, Some(str_char));
    }
    assert_eq!(reader.next_char().unwrap(), None);
}
#[test]
fn test_lines(){
    let bytes = TEXT.as_bytes();
    let mut reader = CharReader::new(bytes);
    for str_line in TEXT.lines() {
        let cr_line = reader.next_line(50, 500).unwrap();
        assert_eq!(&cr_line.unwrap(), str_line);
    }
    assert_eq!(reader.next_line(50, 500).unwrap(), None);
}
#[test]
fn test_thresholds(){
    let bytes = TEXT.as_bytes();
    let mut reader = CharReader::new(bytes);
    assert_eq!(&reader.next_line(5, 15).unwrap().unwrap(), "Comun");
    assert_eq!(&reader.next_line(5, 15).unwrap().unwrap(), "概");
    assert_eq!(
        reader.next_line(5, 15).map_err(|e| e.kind()),
        Err(io::ErrorKind::Other), // too long
    );
}
#[test]
fn check_utf8_error(){
    let mut reader = CharReader::new(INVALID);
    assert_eq!(reader.next_char().unwrap(), Some('a'));
    assert_eq!(
        reader.next_char().map_err(|e| e.kind()),
        Err(io::ErrorKind::InvalidData),
    );
}

