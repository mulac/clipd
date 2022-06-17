#[inline]
pub fn truncate_utf8(s: &str, index: usize) -> &str {
    if index >= s.len() {
        return s
    }
    let lower_bound = index.saturating_sub(3);
    let new_index = s.as_bytes()[lower_bound..=index]
        .iter()
        .rposition(|b| (*b as i8) >= -0x40);

    // SAFETY: we know that the character boundary will be within four bytes
    unsafe { &s[..lower_bound + new_index.unwrap_unchecked()] }
}