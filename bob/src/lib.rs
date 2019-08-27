pub fn reply(message: &str) -> &str {
    fn is_all_capital(message: &str) -> bool {
        message.contains(char::is_alphabetic) && !message.contains(char::is_lowercase)
    }

    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    match (message.ends_with("?"), is_all_capital(message)) {
        (true, true) => "Calm down, I know what I\'m doing!",
        (false, true) => "Whoa, chill out!",
        (true, false) => "Sure.",
        (_, _) => "Whatever.",
    }
}
