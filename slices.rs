fn main() {
    let a_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&a_string[0..6]);
    let _word = first_word(&a_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&a_string);

    let a_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&a_string_literal[0..6]);
    let _word = first_word(&a_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(a_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
