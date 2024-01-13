pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_not_digit(c: char) -> bool {
    !c.is_ascii_digit()
}