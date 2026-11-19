
use crate::tokenizer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if !matches!(self.current(), Token::Eof) {
            self.pos += 1;
        }
    }

    fn precedence(token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            _ => 0,
        }
    }

    pub fn parse(&mut self) -> Result<f64, &'static str> {
        let result = self.parse_expression(0)?;

        if !matches!(self.current(), Token::Eof) {
            return Err("Expressao invalida");
        }

        Ok(result)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<f64, &'static str> {
        let mut left = self.parse_prefix()?;

        loop {
            let prec = Self::precedence(self.current());

            if prec < min_prec || prec == 0 {
                break;
            }

            let op = self.current().clone();
            self.advance();

            let right = self.parse_expression(prec + 1)?;

            left = match op {
                Token::Plus => left + right,
                Token::Minus => left - right,
                Token::Star => left * right,
                Token::Slash => {
                    if right == 0.0 {
                        return Err("Divisao por zero");
                    }
                    left / right
                }
                _ => return Err("Operador invalido"),
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<f64, &'static str> {
        let token = self.current().clone();
        self.advance();

        match token {
            Token::Number(n) => Ok(n),

            Token::Minus => {
                Ok(-self.parse_expression(3)?)
            }

            Token::Plus => {
                self.parse_expression(3)
            }

            Token::LeftParen => {
                let result = self.parse_expression(0)?;

                if !matches!(self.current(), Token::RightParen) {
                    return Err("Parenteses nao fechado");
                }

                self.advance();
                Ok(result)
            }

            _ => Err("Expressao invalida"),
        }
    }
}