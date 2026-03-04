pub fn str_to_bytes(string: &str) -> Vec<u8> {
    string.chars().map(|c| c as u8).collect()
}