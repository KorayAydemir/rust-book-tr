// ANCHOR: here
fn last_char_of_first_line(metin: &str) -> Option<char> {
    metin.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        last_char_of_first_line("Merhaba, dünya\nBugün nasılsın?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nselam"), None);
}
