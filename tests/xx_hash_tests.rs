use xx_hash::{self, xx_hash32, xx_hash64};

#[test]
fn test_xx_hash32_hello_world() {
    let input = b"Hello World!";
    let expected = 0x0bd69788;
    let actual = xx_hash32(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_xx_hash32() {
    let input = b"The quick brown fox jumps over the lazy dog";
    let expected = 0xe85ea4de;
    let actual = xx_hash32(input);
    assert_eq!(expected, actual);

    let input = b"Nobody inspects the spammish repetition";
    let expected = 0xe2293b2f;
    let actual = xx_hash32(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_xx_hash64() {
    let input = b"The quick brown fox jumps over the lazy dog";
    let expected = 0x0b242d361fda71bc;
    let actual = xx_hash64(input);
    assert_eq!(expected, actual);

    let input = b"Nobody inspects the spammish repetition";
    let expected = 0xfbcea83c8a378bf1;
    let actual = xx_hash64(input);
    assert_eq!(expected, actual);
}
