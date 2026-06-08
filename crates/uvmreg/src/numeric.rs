use std::collections::HashMap;

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperatorToken {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EqEq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    AndAnd,
    OrOr,
    Bang,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(u64),
    Ident(String),
    Operator(OperatorToken),
}

pub(crate) fn parse_u64_expr(field: &'static str, value: &str) -> Result<u64> {
    parse_u64_expr_with_symbols(field, value, &HashMap::new())
}

pub(crate) fn parse_u64_expr_with_symbols(
    field: &'static str,
    value: &str,
    symbols: &HashMap<String, u64>,
) -> Result<u64> {
    let tokens = tokenize(field, value)?;
    if tokens.is_empty() {
        return invalid_number(field, value);
    }
    let mut parser = Parser {
        tokens: &tokens,
        index: 0,
        field,
        value,
        symbols,
    };
    let result = parser.parse_expr()?;
    if parser.index != tokens.len() {
        return invalid_number(field, value);
    }
    Ok(result)
}

pub(crate) fn parse_bool_expr_with_symbols(
    field: &'static str,
    value: &str,
    symbols: &HashMap<String, u64>,
) -> Result<bool> {
    let tokens = tokenize(field, value)?;
    if tokens.is_empty() {
        return invalid_number(field, value);
    }
    let mut parser = Parser {
        tokens: &tokens,
        index: 0,
        field,
        value,
        symbols,
    };
    let result = parser.parse_bool_or()?;
    if parser.index != tokens.len() {
        return invalid_number(field, value);
    }
    Ok(result)
}

fn tokenize(field: &'static str, value: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut index = 0;
    while index < chars.len() {
        let (byte_index, ch) = chars[index];
        match ch {
            ch if ch.is_ascii_whitespace() => index += 1,
            '+' => {
                tokens.push(Token::Operator(OperatorToken::Plus));
                index += 1;
            }
            '-' => {
                tokens.push(Token::Operator(OperatorToken::Minus));
                index += 1;
            }
            '*' => {
                tokens.push(Token::Operator(OperatorToken::Star));
                index += 1;
            }
            '/' => {
                tokens.push(Token::Operator(OperatorToken::Slash));
                index += 1;
            }
            '=' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '=') => {
                tokens.push(Token::Operator(OperatorToken::EqEq));
                index += 2;
            }
            '!' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '=') => {
                tokens.push(Token::Operator(OperatorToken::NotEq));
                index += 2;
            }
            '<' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '=') => {
                tokens.push(Token::Operator(OperatorToken::LessEq));
                index += 2;
            }
            '<' => {
                tokens.push(Token::Operator(OperatorToken::Less));
                index += 1;
            }
            '>' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '=') => {
                tokens.push(Token::Operator(OperatorToken::GreaterEq));
                index += 2;
            }
            '>' => {
                tokens.push(Token::Operator(OperatorToken::Greater));
                index += 1;
            }
            '&' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '&') => {
                tokens.push(Token::Operator(OperatorToken::AndAnd));
                index += 2;
            }
            '|' if chars.get(index + 1).is_some_and(|(_, ch)| *ch == '|') => {
                tokens.push(Token::Operator(OperatorToken::OrOr));
                index += 2;
            }
            '!' => {
                tokens.push(Token::Operator(OperatorToken::Bang));
                index += 1;
            }
            '(' => {
                tokens.push(Token::Operator(OperatorToken::LParen));
                index += 1;
            }
            ')' => {
                tokens.push(Token::Operator(OperatorToken::RParen));
                index += 1;
            }
            ch if is_number_start(ch) => {
                let start = byte_index;
                index += 1;
                while index < chars.len() && is_number_char(chars[index].1) {
                    index += 1;
                }
                let end = chars
                    .get(index)
                    .map(|(byte_index, _)| *byte_index)
                    .unwrap_or(value.len());
                tokens.push(Token::Number(parse_literal(
                    field,
                    value,
                    &value[start..end],
                )?));
            }
            ch if is_ident_start(ch) => {
                let start = byte_index;
                index += 1;
                while index < chars.len() && is_ident_char(chars[index].1) {
                    index += 1;
                }
                let end = chars
                    .get(index)
                    .map(|(byte_index, _)| *byte_index)
                    .unwrap_or(value.len());
                tokens.push(Token::Ident(value[start..end].to_string()));
            }
            _ => return invalid_number(field, value),
        }
    }
    Ok(tokens)
}

fn is_number_start(ch: char) -> bool {
    ch.is_ascii_digit() || ch == '\''
}

fn is_number_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '\'' || ch == '_'
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '.'
}

fn parse_literal(field: &'static str, value: &str, literal: &str) -> Result<u64> {
    let literal = literal.replace('_', "");
    let lower = literal.to_ascii_lowercase();
    if let Some(hex) = lower.strip_prefix("0x") {
        return u64::from_str_radix(hex, 16).map_err(|_| invalid_number_error(field, value));
    }
    if let Some(binary) = lower.strip_prefix("0b") {
        return u64::from_str_radix(binary, 2).map_err(|_| invalid_number_error(field, value));
    }
    if let Some((_, based)) = lower.split_once('\'') {
        let based = based.strip_prefix('s').unwrap_or(based);
        let (base, digits) = based.split_at(1);
        let radix = match base {
            "h" => 16,
            "d" => 10,
            "b" => 2,
            "o" => 8,
            _ => return invalid_number(field, value),
        };
        return u64::from_str_radix(digits, radix).map_err(|_| invalid_number_error(field, value));
    }
    lower
        .parse::<u64>()
        .map_err(|_| invalid_number_error(field, value))
}

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    field: &'static str,
    value: &'a str,
    symbols: &'a HashMap<String, u64>,
}

impl Parser<'_> {
    fn parse_bool_or(&mut self) -> Result<bool> {
        let mut result = self.parse_bool_and()?;
        loop {
            match self.peek() {
                Some(Token::Operator(OperatorToken::OrOr)) => {
                    self.index += 1;
                    result = self.parse_bool_and()? || result;
                }
                _ => return Ok(result),
            }
        }
    }

    fn parse_bool_and(&mut self) -> Result<bool> {
        let mut result = self.parse_bool_not()?;
        loop {
            match self.peek() {
                Some(Token::Operator(OperatorToken::AndAnd)) => {
                    self.index += 1;
                    result = self.parse_bool_not()? && result;
                }
                _ => return Ok(result),
            }
        }
    }

    fn parse_bool_not(&mut self) -> Result<bool> {
        match self.peek() {
            Some(Token::Operator(OperatorToken::Bang)) => {
                self.index += 1;
                Ok(!self.parse_bool_not()?)
            }
            Some(Token::Operator(OperatorToken::LParen)) if self.parenthesized_bool_expr() => {
                self.index += 1;
                let result = self.parse_bool_or()?;
                match self.peek() {
                    Some(Token::Operator(OperatorToken::RParen)) => {
                        self.index += 1;
                        Ok(result)
                    }
                    _ => invalid_number(self.field, self.value),
                }
            }
            _ => self.parse_bool_comparison(),
        }
    }

    fn parse_bool_comparison(&mut self) -> Result<bool> {
        let left = self.parse_expr()?;
        match self.peek() {
            Some(Token::Operator(OperatorToken::EqEq)) => {
                self.index += 1;
                Ok(left == self.parse_expr()?)
            }
            Some(Token::Operator(OperatorToken::NotEq)) => {
                self.index += 1;
                Ok(left != self.parse_expr()?)
            }
            Some(Token::Operator(OperatorToken::Less)) => {
                self.index += 1;
                Ok(left < self.parse_expr()?)
            }
            Some(Token::Operator(OperatorToken::LessEq)) => {
                self.index += 1;
                Ok(left <= self.parse_expr()?)
            }
            Some(Token::Operator(OperatorToken::Greater)) => {
                self.index += 1;
                Ok(left > self.parse_expr()?)
            }
            Some(Token::Operator(OperatorToken::GreaterEq)) => {
                self.index += 1;
                Ok(left >= self.parse_expr()?)
            }
            _ => Ok(left != 0),
        }
    }

    fn parenthesized_bool_expr(&self) -> bool {
        let mut depth = 0usize;
        for token in &self.tokens[self.index..] {
            match token {
                Token::Operator(OperatorToken::LParen) => depth += 1,
                Token::Operator(OperatorToken::RParen) => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return false;
                    }
                }
                Token::Operator(
                    OperatorToken::EqEq
                    | OperatorToken::NotEq
                    | OperatorToken::Less
                    | OperatorToken::LessEq
                    | OperatorToken::Greater
                    | OperatorToken::GreaterEq
                    | OperatorToken::AndAnd
                    | OperatorToken::OrOr,
                ) if depth == 1 => return true,
                _ => {}
            }
        }
        false
    }

    fn parse_expr(&mut self) -> Result<u64> {
        let mut result = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Operator(OperatorToken::Plus)) => {
                    self.index += 1;
                    result = result
                        .checked_add(self.parse_term()?)
                        .ok_or_else(|| invalid_number_error(self.field, self.value))?;
                }
                Some(Token::Operator(OperatorToken::Minus)) => {
                    self.index += 1;
                    result = result
                        .checked_sub(self.parse_term()?)
                        .ok_or_else(|| invalid_number_error(self.field, self.value))?;
                }
                _ => return Ok(result),
            }
        }
    }

    fn parse_term(&mut self) -> Result<u64> {
        let mut result = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Token::Operator(OperatorToken::Star)) => {
                    self.index += 1;
                    result = result
                        .checked_mul(self.parse_factor()?)
                        .ok_or_else(|| invalid_number_error(self.field, self.value))?;
                }
                Some(Token::Operator(OperatorToken::Slash)) => {
                    self.index += 1;
                    let divisor = self.parse_factor()?;
                    if divisor == 0 {
                        return invalid_number(self.field, self.value);
                    }
                    result /= divisor;
                }
                _ => return Ok(result),
            }
        }
    }

    fn parse_factor(&mut self) -> Result<u64> {
        match self.peek().cloned() {
            Some(Token::Number(value)) => {
                self.index += 1;
                Ok(value)
            }
            Some(Token::Ident(name)) => {
                self.index += 1;
                self.symbols
                    .get(&name)
                    .copied()
                    .ok_or_else(|| invalid_number_error(self.field, self.value))
            }
            Some(Token::Operator(OperatorToken::Plus)) => {
                self.index += 1;
                self.parse_factor()
            }
            Some(Token::Operator(OperatorToken::LParen)) => {
                self.index += 1;
                let result = self.parse_expr()?;
                match self.peek() {
                    Some(Token::Operator(OperatorToken::RParen)) => {
                        self.index += 1;
                        Ok(result)
                    }
                    _ => invalid_number(self.field, self.value),
                }
            }
            _ => invalid_number(self.field, self.value),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}

fn invalid_number<T>(field: &'static str, value: &str) -> Result<T> {
    Err(invalid_number_error(field, value))
}

fn invalid_number_error(field: &'static str, value: &str) -> Error {
    Error::InvalidNumber {
        field,
        value: value.into(),
    }
}
