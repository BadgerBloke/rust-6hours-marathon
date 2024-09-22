pub fn find_first_a(s: String) -> Option<usize> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index);
        }
    }
    return None;
}
