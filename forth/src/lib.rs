pub type Value = i32;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

use std::collections::HashMap;

pub struct Forth {
    pub stack: Vec<Value>,

    pub user_defined_words: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

use std::iter::Peekable;

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            user_defined_words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        // eprintln!("result of evaluating '{input}'");

        let mut input = input.chars().peekable();

        while input.peek().is_some() {
            Self::skip_whitespace(&mut input);
            if let Some(number) = Self::parse_number(&mut input) {
                self.stack.push(number);
                continue;
            }

            Self::skip_whitespace(&mut input);
            let word = self.parse_word(&mut input)?;
            let word = self
                .user_defined_words
                .get(&word)
                .map(String::as_str)
                .unwrap_or(word.as_str());

            word.split_whitespace().try_for_each(|word| {
                let word = self
                    .user_defined_words
                    .get(word)
                    .map(String::as_str)
                    .unwrap_or(word);

                match word {
                    // integer arithmetic
                    op @ ("+" | "-" | "*" | "/") => {
                        let number1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                        let number2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                        self.stack
                            .push(Self::perform_arithmetic(number2, number1, op)?);
                    }

                    // stack manipulation
                    "dup" => {
                        let top = self.stack.last().copied().ok_or(Error::StackUnderflow)?;
                        self.stack.push(top);
                    }
                    "drop" => {
                        let _ = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    }
                    "swap" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }

                        let top = self.stack.len() - 1;
                        self.stack.swap(top - 1, top);
                    }
                    "over" => {
                        if self.stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }

                        let last_second = self
                            .stack
                            .get(self.stack.len() - 2)
                            .copied()
                            .ok_or(Error::StackUnderflow)?;
                        self.stack.push(last_second);
                    }

                    // custom word definition
                    ";" => (),

                    word if word.parse::<Value>().is_ok() => {
                        self.stack.push(word.parse().unwrap());
                    }

                    _ => {
                        eprintln!("unknown word {word:?}");
                        return Err(Error::UnknownWord);
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    fn parse_number<I>(input: &mut Peekable<I>) -> Option<Value>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut number = String::new();

        loop {
            match input.peek().copied() {
                Some('-')
                    if number.is_empty()
                        && matches!(
                            input.clone().skip(1).peekable().peek(),
                            Some(t) if t.is_ascii_digit()
                        ) =>
                {
                    number.push('-')
                }
                Some(t) if t.is_ascii_digit() => number.push(t),

                _ => break,
            }
            input.next();
        }

        if number.is_empty() {
            None
        } else {
            Some(number.parse().unwrap())
        }
    }

    fn parse_word<I>(&mut self, input: &mut Peekable<I>) -> Result<String>
    where
        I: Iterator<Item = char>,
    {
        let mut word = String::new();

        loop {
            match input.peek() {
                Some(&':') if word.is_empty() => {
                    input.next();

                    Self::skip_whitespace(input);

                    let word_name = {
                        let mut name = String::new();
                        while matches!(input.peek(), Some(t) if !t.is_whitespace() && !t.is_ascii_digit() && (t != &'-' || !name.is_empty()))
                        {
                            name.push(input.next().unwrap());
                        }

                        if name.is_empty() {
                            Err(Error::InvalidWord)
                        } else {
                            Ok(name.to_lowercase())
                        }
                    }?;

                    let word_definition = {
                        let mut definition = String::new();
                        while matches!(input.peek(), Some(t) if t != &';') {
                            definition.push(input.next().unwrap());
                        }

                        let definition = definition
                            .split_whitespace()
                            .map(|s| s.to_lowercase())
                            .collect::<Vec<_>>()
                            .join(" ");
                        if definition.is_empty() {
                            Err(Error::InvalidWord)
                        } else {
                            Ok(definition)
                        }
                    }?;

                    if !matches!(input.peek(), Some(&';')) {
                        return Err(Error::InvalidWord);
                    }

                    let mut word_definition = self
                        .user_defined_words
                        .get(&word_definition)
                        .cloned()
                        .unwrap_or(word_definition);

                    if let Some(old_definition) = self.user_defined_words.get(&word_name).cloned() {
                        #[allow(clippy::search_is_some)]
                        if word_definition
                            .split_whitespace()
                            .find(|&w| word_name == w)
                            .is_some()
                        {
                            let mut f = Self {
                                stack: Vec::new(),
                                user_defined_words: std::mem::take(&mut self.user_defined_words),
                            };

                            f.eval(&word_definition.replace(&word_name, &old_definition))?;
                            word_definition = f
                                .stack()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(" ");

                            self.user_defined_words = f.user_defined_words;
                        }
                    }

                    self.user_defined_words.insert(word_name, word_definition);
                    word = input.next().unwrap().to_string();
                    break;
                }

                Some(t) if t.is_whitespace() => break,
                Some(t) if !t.is_ascii_digit() => {
                    word.push(*t);
                }

                _ => break,
            }
            input.next();
        }

        if word.is_empty() {
            Err(Error::InvalidWord)
        } else {
            Ok(word.to_lowercase())
        }
    }

    fn perform_arithmetic(left_num: Value, right_num: Value, op: &str) -> Result<Value> {
        match op {
            "+" => Ok(left_num + right_num),
            "-" => Ok(left_num - right_num),
            "*" => Ok(left_num * right_num),
            "/" => {
                if right_num == 0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(left_num / right_num)
                }
            }
            _ => Err(Error::UnknownWord),
        }
    }

    fn skip_whitespace<I>(input: &mut Peekable<I>)
    where
        I: Iterator<Item = char>,
    {
        while matches!(input.peek(), Some(t) if t.is_whitespace()) {
            input.next();
        }
    }
}
