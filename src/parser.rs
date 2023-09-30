use std::rc::Rc;

use crate::{
    expression::Expression,
    expression::{
        assign::Assign, binary::Binary, body::Body, call::Call, function::Function,
        if_expression::If, literal::Literal, unary::Unary, variable::Variable,
    },
    tokeniser::{Keyword, Operator, Symbol, Token},
};

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        return self.previous();
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.index].clone();
    }

    fn safe_peek_symbol(&mut self) -> Option<Symbol> {
        if self.is_at_end() {
            return None;
        }

        Some(self.peek().symbol)
    }

    pub fn previous(&mut self) -> Token {
        return self.tokens[self.index - 1].clone();
    }

    fn is_at_end(&mut self) -> bool {
        match self.tokens.get(self.index).cloned() {
            Some(Token {
                symbol: Symbol::Fin,
                line: _,
                index: _,
            }) => true,
            _ => false,
        }
    }

    fn check(&mut self, symbol: Symbol) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().symbol == symbol;
    }

    fn match_symbols(&mut self, symbols: &[Symbol]) -> Option<Token> {
        for symbol in symbols.iter().cloned() {
            if self.check(symbol) {
                return Some(self.advance());
            }
        }

        return None;
    }

    fn match_operators(&mut self, operators: &[Operator]) -> Option<Operator> {
        for operator in operators.iter().cloned() {
            if self.check(Symbol::Operator(operator)) {
                self.advance();
                return Some(operator);
            }
        }

        return None;
    }

    fn match_keywords(&mut self, operators: &[Keyword]) -> Option<Keyword> {
        for operator in operators.iter().cloned() {
            if self.check(Symbol::Keyword(operator)) {
                self.advance();
                return Some(operator);
            }
        }

        return None;
    }

    fn expect(&mut self, symbols: &[Symbol]) -> Result<Token, String> {
        if let Some(symbol) = self.match_symbols(symbols) {
            return Ok(symbol);
        }

        return Err(format!(
            "Expected {:#?} but got ${:#?}",
            symbols,
            self.peek()
        ));
    }

    fn function_definition(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut argument_names = Vec::new();
        while let Some(Symbol::Identifier(identifier)) = self.safe_peek_symbol() {
            argument_names.push(identifier);
            self.advance();
        }

        self.expect(&[Symbol::RightParen])?;
        match self.expect(&[Symbol::Arrow]) {
            Ok(_) => (),
            Err(_) if argument_names.len() == 1 => {
                return Ok(Box::new(Variable {
                    name: argument_names[0].clone(),
                }))
            }
            Err(err) => return Err(err),
        }

        let body = self.expression()?;

        Ok(Box::new(Function {
            argument_names,
            body: Rc::new(body),
        }))
    }

    fn bottom(&mut self) -> Result<Box<dyn Expression>, String> {
        self.advance();
        match self.previous().symbol {
            Symbol::Identifier(identifier) => Ok(Box::new(Variable { name: identifier })),
            Symbol::Literal(value) => Ok(Box::new(Literal { value })),
            Symbol::LeftBrace => {
                let mut body = Vec::new();

                while !self.check(Symbol::RightBrace) {
                    let expression = self.next()?;
                    body.push(expression);
                }

                self.expect(&[Symbol::RightBrace])?;

                Ok(Box::new(Body { body }))
            }
            Symbol::LeftParen => {
                match self.safe_peek_symbol() {
                    Some(Symbol::Identifier(_)) => return self.function_definition(),
                    Some(Symbol::RightParen) => return self.function_definition(),
                    _ => (),
                }

                let expr = self.expression()?;
                self.expect(&[Symbol::RightParen])?;

                Ok(expr)
            }
            _ => Err(format!("Unrecognized token")),
        }
    }

    fn call(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.bottom()?;

        if let Some(_) = self.match_symbols(&[Symbol::LeftParen]) {
            let mut args = Vec::new();
            if !self.check(Symbol::RightParen) {
                loop {
                    let arg = self.expression()?;
                    args.push(arg);

                    if !self.check(Symbol::Comma) {
                        break;
                    }
                }
            }

            self.expect(&[Symbol::RightParen])?;

            expr = Box::from(Call {
                target: expr,
                arguments: args,
            })
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, String> {
        if let Some(operator) = self.match_operators(&[Operator::Minus, Operator::Not]) {
            let expression = self.unary()?;
            return Ok(Box::new(Unary {
                operator,
                value: expression,
            }));
        }

        return self.call();
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_operators(&[Operator::Star, Operator::Slash]) {
            let right = self.unary()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_operators(&[Operator::Plus, Operator::Minus]) {
            let right = self.factor()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.term()?;

        while let Some(operator) = self.match_operators(&[
            Operator::GreaterThan,
            Operator::GreaterThanOrEqual,
            Operator::LesserThan,
            Operator::LesserThanOrEqual,
        ]) {
            let right = self.term()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_operators(&[Operator::Equal, Operator::NotEqual]) {
            let right = self.comparison()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn logic_and(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.equality()?;

        while let Some(operator) = self.match_operators(&[Operator::And]) {
            let right = self.equality()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn logic_or(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.logic_and()?;

        while let Some(operator) = self.match_operators(&[Operator::Or]) {
            let right = self.logic_and()?;

            expr = Box::from(Binary {
                left: expr,
                operator: operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn assign(&mut self) -> Result<Box<dyn Expression>, String> {
        if let Some(_) = self.match_keywords(&[Keyword::Let]) {
            return match self.peek().symbol {
                Symbol::Identifier(key) => {
                    self.advance();
                    self.expect(&[Symbol::Assign])?;
                    let value = self.expression()?;
                    Ok(Box::from(Assign { key, value }))
                }
                symbol => Err(format!(
                    "Expected Identifier after `let` keyword, got: {:#?}",
                    symbol
                )),
            };
        }

        if let Some(Symbol::Keyword(Keyword::If)) = self.safe_peek_symbol() {
            self.advance();
            let condition = self.expression()?;
            let body = self.expression()?;
            let mut else_body: Option<Box<dyn Expression>> = None;

            if let Some(Symbol::Keyword(Keyword::Else)) = self.safe_peek_symbol() {
                self.advance();
                else_body = Some(self.expression()?);
            }

            return Ok(Box::from(If {
                condition,
                body,
                else_body,
            }));
        }

        self.logic_or()
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        return self.assign();
    }

    pub fn next(&mut self) -> Result<Box<dyn Expression>, String> {
        let expr = self.expression();
        self.expect(&[Symbol::Semi])?;
        return expr;
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Box<dyn Expression>>, String> {
    let mut expressions: Vec<Box<dyn Expression>> = Vec::new();
    let mut parser = Parser { index: 0, tokens };

    while !parser.is_at_end() {
        let expr = parser.next()?;
        expressions.push(expr);
    }

    return Ok(expressions);
}
