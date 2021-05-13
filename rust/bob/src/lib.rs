pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if all_uppercase(m) && is_message(m) => "Calm down, I know what I'm doing!",
        m if is_message(m) => "Sure.",
        m if m.is_empty() => "Fine. Be that way!",
        m if all_uppercase(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

fn all_uppercase(message: &str) -> bool {
    message.chars().any(char::is_alphabetic)
        &&
        message
            .chars()
            .filter(|c| c.is_lowercase())
            .count() == 0
}

fn is_message(message: &str) -> bool {
    message.ends_with('?')
}
