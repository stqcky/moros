pub fn xor(source_key: (&[u8], &[u8])) -> String {
    let (source, key) = source_key;

    String::from_utf8(
        source
            .iter()
            .zip(key.iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect(),
    )
    .unwrap()
}
