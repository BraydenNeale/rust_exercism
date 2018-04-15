const QUESTION_RESPONSE: &str = "Sure.";
const YELL_RESPONSE: &str = "Whoa, chill out!";
const YELL_QUESTION_RESPONSE: &str = "Calm down, I know what I'm doing!";
const BLANK_RESPONSE: &str = "Fine. Be that way!";
const DEFAULT_RESPONSE: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
	let trim = message.trim();
    if trim.is_empty() { return BLANK_RESPONSE };

    let mut response = DEFAULT_RESPONSE;

    // fails for only number and punctation messages (3/25 cases)...
    // interested to see how this is done without regex
    let is_question = trim.to_string().pop().unwrap() == '?';
    let is_yelling = trim.to_string() == trim.to_uppercase();

    if is_question && is_yelling { response = YELL_QUESTION_RESPONSE }
    else if is_question { response = QUESTION_RESPONSE }
    else if is_yelling { response = YELL_RESPONSE }

    response
}
