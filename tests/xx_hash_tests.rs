use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};
use xx_hash::{self, xx_hash32, xx_hash64, Xx64};

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

    let input = b"Nobody inspects the spammish repetition";
    let expected = 0xfbcea83c8a378bf1;
    let mut xx64 = Xx64::default();
    xx64.read_chunk(input);
    let actual = xx64.finalize();
    assert_eq!(expected, actual);

    let mut rng = StdRng::seed_from_u64(1234);

    {
        for num_elements in [1, 10, 100, 500, 1000, 5000, 10000, 50000] {
            let strings_input = (1..num_elements)
                .map(|_i| {
                    let string_size = rng.gen_range(5..20);
                    (&mut rng)
                        .sample_iter(&Alphanumeric)
                        .take(string_size)
                        .map(char::from)
                        .collect::<String>()
                })
                .collect::<Vec<String>>();

            for string in &strings_input {
                dbg!(string);
                let input = string.as_bytes();

                let mut xx64 = Xx64::default();
                xx64.read_chunk(input);

                assert_eq!(dbg!(xx_hash64(input)), dbg!(xx64.finalize()));
            }
        }
    }
}
