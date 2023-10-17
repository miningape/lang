use std::rc::Rc;

use crate::{
    environment,
    expression::Expression,
    expression::{
        assign::Assign,
        binary::Binary,
        body::Body,
        call::Call,
        declare::Declare,
        function::{Function, FunctionArgument},
        if_expression::If,
        literal::Literal,
        unary::Unary,
        variable::Variable,
    },
    tokeniser::{self, Keyword, Operator, Symbol, Token, TypeLiteral},
    types::{BaseType, FunctionType, Type},
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

    fn advance_symbol(&mut self) -> Symbol {
        self.advance().symbol
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

    fn type_base(&mut self) -> Result<Type, String> {
        if let Some(Symbol::Literal(tokeniser::Literal::Null)) = self.safe_peek_symbol() {
            self.advance();
            return Ok(Type::BaseType(BaseType::Null));
        }

        if self.check(Symbol::LeftParen) {
            self.advance();

            if self.check(Symbol::RightParen) {
                self.advance();

                if self.check(Symbol::Arrow) {
                    self.advance();

                    let return_type = self.type_annotation()?;
                    return Ok(Type::Function(Box::from(FunctionType::Literal(
                        Vec::new(),
                        return_type,
                    ))));
                } else {
                    return Err(String::from("Expected => after ()"));
                }
            }

            let first = &self.type_annotation()?;

            let mut argument_types = Vec::new();
            argument_types.push(first.clone());

            if self.check(Symbol::RightParen) {
                self.advance();

                if !self.check(Symbol::Arrow) {
                    self.advance();
                    return Ok(first.clone());
                }

                let return_type = self.type_annotation()?;

                return Ok(Type::Function(Box::from(FunctionType::Literal(
                    argument_types,
                    return_type,
                ))));
            }

            while self.check(Symbol::Comma) {
                self.advance();
                let type_expr = self.type_annotation()?;
                argument_types.push(type_expr);
            }

            self.expect(&[Symbol::RightParen])?;
            self.expect(&[Symbol::Arrow])?;

            let return_type = self.type_annotation()?;

            return Ok(Type::Function(Box::from(FunctionType::Literal(
                argument_types,
                return_type,
            ))));
        }

        let Symbol::TypeLiteral(type_literal) = self.advance_symbol() else {
            return Err("Expected type annotation".to_owned());
        };

        Ok(match type_literal {
            TypeLiteral::Any => Type::BaseType(BaseType::Any),
            TypeLiteral::Number => Type::BaseType(BaseType::Number),
            TypeLiteral::String => Type::BaseType(BaseType::String),
            TypeLiteral::Boolean => Type::BaseType(BaseType::Boolean),
        })
    }

    fn type_or(&mut self) -> Result<Type, String> {
        let mut type_expr = self.type_base()?;

        while let Some(_) = self.match_operators(&[Operator::Or]) {
            let right = self.type_base()?;
            type_expr = Type::Or(Box::from(type_expr), Box::from(right));
        }

        return Ok(type_expr);
    }

    fn type_annotation(&mut self) -> Result<Type, String> {
        self.type_or()
    }

    fn function_arguments(&mut self) -> Result<Vec<FunctionArgument>, String> {
        let mut arguments: Vec<FunctionArgument> = Vec::new();

        if !self.check(Symbol::RightParen) {
            loop {
                let Symbol::Identifier(name) = self.advance_symbol() else {
                    return Err("Expected identifier in arg list".to_owned());
                };
                self.expect(&[Symbol::Colon])?;

                let type_annotation = match self.type_annotation() {
                    Err(err) => Err(format!("In function argument definition | {}", err)),
                    t => t,
                }?;
                arguments.push(FunctionArgument {
                    name,
                    type_annotation,
                });

                if let Some(Symbol::RightParen) = self.safe_peek_symbol() {
                    break;
                }

                self.expect(&[Symbol::Comma])?;
            }
        }

        self.expect(&[Symbol::RightParen])?;

        return Ok(arguments);
    }

    fn function_definition(&mut self) -> Result<Box<dyn Expression>, String> {
        let arguments = self.function_arguments()?;

        let mut return_type = Type::BaseType(BaseType::Infer);
        if let Some(_) = self.match_symbols(&[Symbol::Colon]) {
            return_type = match self.type_annotation() {
                Err(err) => Err(format!(
                    "After function argument definition, expected return type | {}",
                    err
                )),
                t => t,
            }?;
        }

        self.expect(&[Symbol::Arrow])?;

        let body = self.expression()?;

        Ok(Box::new(Function {
            arguments,
            return_type,
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
            _ => Err(format!("Cannot match token: {:#?}", self.previous())),
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

                    self.advance();
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
        if let Some(Symbol::Identifier(identifier)) = self.safe_peek_symbol() {
            self.advance();
            if self.check(Symbol::Assign) {
                self.advance();
                let value = self.expression()?;

                return Ok(Box::from(Assign {
                    key: identifier,
                    value,
                }));
            } else {
                self.index -= 1;
            }
        }

        self.logic_or()
    }

    fn optional_type_annotation(&mut self) -> Type {
        let index_before = self.index;

        match self.type_annotation() {
            Ok(annotation) => annotation,
            Err(_) => {
                self.index = index_before;
                Type::BaseType(BaseType::Infer)
            }
        }
    }

    fn declare(&mut self) -> Result<Box<dyn Expression>, String> {
        if let Some(_) = self.match_keywords(&[Keyword::Let]) {
            return match self.peek().symbol {
                Symbol::Identifier(key) => {
                    self.advance();

                    let mut assigned_type: Option<environment::Variable<Type>> = None;
                    if self.check(Symbol::Colon) {
                        self.advance();

                        let mut mutable = false;
                        if self.check(Symbol::Keyword(Keyword::Mutable)) {
                            self.advance();
                            mutable = true;
                        }

                        let value = match mutable {
                            true => self.optional_type_annotation(),
                            false => self.type_annotation()?,
                        };

                        assigned_type = Some(environment::Variable { mutable, value })
                    }

                    self.expect(&[Symbol::Assign])?;
                    let value = self.expression()?;
                    Ok(Box::from(Declare {
                        key,
                        assigned_type,
                        value,
                    }))
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

        self.assign()
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        return self.declare();
    }

    pub fn next(&mut self) -> Result<Box<dyn Expression>, String> {
        let expr = self.expression()?;
        self.expect(&[Symbol::Semi])?;
        return Ok(expr);
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
