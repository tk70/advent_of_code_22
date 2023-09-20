pub const fn get_char_index(c: char) -> isize {
    if c.is_ascii_lowercase() {
        (c as isize) - ('a' as isize) + 1
    } else if c.is_ascii_uppercase() {
        (c as isize) - ('A' as isize) + 27
    } else {
        0
    }
}
