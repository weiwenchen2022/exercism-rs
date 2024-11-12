use std::iter::Peekable;

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn answer(command: &str) -> Option<i32> {
    // todo!("Return the result of the command '{command}' or None, if the command is invalid.");

    let mut tokens = command.chars().peekable();

    let mut left = parse_number(&mut tokens)?;

    let mut first_round = true;
    skip_whitespace(&mut tokens);

    while tokens.peek().is_some() {
        skip_whitespace(&mut tokens);
        let op = match parse_operator(&mut tokens) {
            Some(op) => op,
            None => {
                return if !first_round && parse_number(&mut tokens).is_none() {
                    Some(left)
                } else {
                    None
                };
            }
        };

        skip_whitespace(&mut tokens);
        let right = parse_number(&mut tokens)?;

        left = match op {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        };

        first_round = false;
    }

    Some(left)
}

fn skip_whitespace<I>(tokens: &mut Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while let Some(&ch) = tokens.peek() {
        if ch.is_ascii_whitespace() || ch.is_ascii_punctuation() && ch != '-' {
            tokens.next();
        } else {
            break;
        }
    }
}

fn parse_operator<I>(tokens: &mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = char>,
{
    let mut op = "".to_string();
    loop {
        match tokens.peek() {
            Some(&ch)
                if ch.is_ascii_alphabetic() || (ch.is_ascii_whitespace() && !op.is_empty()) =>
            {
                op.push(ch);
            }
            _ => break,
        }
        tokens.next();
    }

    match op.trim() {
        "plus" => Some(Operator::Add),
        "minus" => Some(Operator::Sub),
        "multiplied by" => Some(Operator::Mul),
        "divided by" => Some(Operator::Div),
        _ => None,
    }
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> Option<i32>
where
    I: Iterator<Item = char>,
{
    let mut num = "".to_string();
    loop {
        match tokens.peek() {
            Some(&'-') if num.is_empty() => num.push('-'),
            Some(&ch) if ch.is_ascii_digit() => num.push(ch),
            Some(_) if num.is_empty() => {}
            _ => break,
        }

        tokens.next();
    }

    num.parse().ok()
}
