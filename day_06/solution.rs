// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1 = s1.trim();
    let s2 = s2.trim();
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    if s1_len > s2_len {
        Some(s1)
    } else if s1_len < s2_len {
        Some(s2)
    } else {
        None
    }
}

