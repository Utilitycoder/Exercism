pub fn reply(message: &str) -> &str {
    let message = message.trim_end();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');

    let is_yelling = message.to_uppercase() == message && message.to_lowercase() != message;

    if is_yelling {
        return if is_questioning {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    } else if is_questioning {
        return "Sure.";
    } else {
        return "Whatever.";
    }
}
