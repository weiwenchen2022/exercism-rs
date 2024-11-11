pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")

    let message = message.trim();
    let is_question = message.ends_with("?");
    let letters = message
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .collect::<String>();
    let is_yelling = !letters.is_empty() && letters.chars().all(char::is_uppercase);
    let is_silence = message.is_empty() || message.chars().all(char::is_whitespace);

    if is_question && is_yelling {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "Whoa, chill out!"
    } else if is_silence {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
