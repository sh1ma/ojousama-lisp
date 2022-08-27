use std::{char, str::Chars};

use crate::token::{Token, TokenKind};

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub(crate) struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { chars: src.chars() }
    }

    fn read(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn skip_whitspace(&mut self) {
        while self.peek().is_whitespace() && !self.is_eof() {
            self.read();
        }
    }
}

impl Lexer<'_> {
    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitspace();

        if let Some(ch) = self.read() {
            let ch_string = ch.to_string();
            match ch {
                '(' => Token::new(TokenKind::OpenParen, ch_string),
                ')' => Token::new(TokenKind::CloseParen, ch_string),
                value => {
                    if is_id_chracter(&value) {
                        self.letter(value)
                    } else if value.is_numeric() {
                        self.number(value)
                    } else {
                        Token::new(TokenKind::Illegal, ch.to_string())
                    }
                }
            }
        } else {
            Token::new(TokenKind::EOF, '\0'.to_string())
        }
    }

    fn letter(&mut self, value: char) -> Token {
        let mut identifier = vec![value];
        let mut cloned_chars = self.chars.clone();
        while let Some(ch) = cloned_chars.next() {
            if is_id_chracter(&ch) {
                identifier.push(ch);
                self.read();
            } else {
                break;
            }
        }
        let identifier = String::from_iter(identifier);
        Token::new(TokenKind::Identfier, identifier)
    }

    fn number(&mut self, value: char) -> Token {
        let mut literal = vec![value];
        let mut cloned_chars = self.chars.clone();
        while let Some(ch) = cloned_chars.next() {
            if ch.is_numeric() {
                literal.push(ch);
                self.read();
            } else {
                break;
            }
        }
        let identifier = String::from_iter(literal);
        Token::new(TokenKind::Int, identifier)
    }
}

fn is_id_chracter(ch: &char) -> bool {
    ch.is_alphabetic() || ch == &'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_parens() {
        let input = "()";
        let expected = vec![(TokenKind::OpenParen, "("), (TokenKind::CloseParen, ")")];

        let mut lexer = Lexer::new(input);
        for ex in expected {
            let token = lexer.next_token();
            assert_eq!(token.kind, ex.0);
            assert_eq!(token.literal, ex.1);
        }
    }

    #[test]
    fn test_next_token_identdfier() {
        let input = "(hello world)";
        let expected = vec![
            (TokenKind::OpenParen, "("),
            (TokenKind::Identfier, "hello"),
            (TokenKind::Identfier, "world"),
            (TokenKind::CloseParen, ")"),
        ];

        let mut lexer = Lexer::new(input);
        for ex in expected {
            let token = lexer.next_token();
            assert_eq!(token.kind, ex.0);
            assert_eq!(token.literal, ex.1);
        }
    }

    #[test]
    fn test_next_token_number() {
        let input = "(hello 123 number)";
        let expected = vec![
            (TokenKind::OpenParen, "("),
            (TokenKind::Identfier, "hello"),
            (TokenKind::Int, "123"),
            (TokenKind::Identfier, "number"),
            (TokenKind::CloseParen, ")"),
        ];

        let mut lexer = Lexer::new(input);
        for ex in expected {
            let token = lexer.next_token();
            assert_eq!(token.kind, ex.0);
            assert_eq!(token.literal, ex.1);
        }
    }
}
