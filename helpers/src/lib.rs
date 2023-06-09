pub fn to_opcode(input: &str) -> Vec<usize> {
    input.split(",").filter_map(|s| s.parse().ok()).collect()
}
