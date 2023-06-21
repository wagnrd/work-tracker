pub fn filter_integers_from_text(text: String) -> Option<i32> {
    let mut result = "".to_string();
    let mut minus_possible = true;

    for character in text.chars() {
        if character.is_numeric() || (minus_possible && character == '-') {
            result += character.to_string().as_str();
            minus_possible = false;
        }
    }

    result.parse::<i32>().ok()
}
