use std::string::ParseError;
use crate::tokenizer::{Tokenized, Tokenizer};
use crate::ast::*;
use crate::tokens::{Token, TokenType};



#[derive(Debug, Clone)]
pub enum AstParserError {
    EndOfFile,
    UnexpectedEof,

    TokenExpected(String, String),
    SomeTokenExpected(Vec<String>, String),
    StatementExpected(String),

    IncompatibleStatement
}

impl AstParserError {
    pub fn to_message(&self) -> String {
        match &self {
            AstParserError::EndOfFile => format!("EndOfFile"),
            AstParserError::UnexpectedEof => "UnexpectedEof".to_string(),
            AstParserError::TokenExpected(expected, given)
            => format!("Token '{}' expected but '{}' given.", given, expected),
            AstParserError::SomeTokenExpected(_, _) => "SomeTokenExpected".to_string(),
            AstParserError::StatementExpected(_) => "StatementExpected".to_string(),
            AstParserError::IncompatibleStatement => "IncompatibleStatement".to_string(),
        }
    }
}

pub struct AstParser {
    pub(crate) tokens : Tokenized,
    source : String
}


impl AstParser {
    fn expect_token_type(&mut self, token_type: TokenType) -> Result<Token, AstParserError> {
        let mut token = self.tokens.next().ok_or_else(|| AstParserError::UnexpectedEof)?.clone();
        if token.get_token_type() != token_type {
            self.tokens.prev();
            return Err(AstParserError::TokenExpected(token.value, format!("{}", "test")));
        }

        Ok(token)
    }

    fn expect_token_value(&mut self, token_value: String) -> Result<Token, AstParserError> {
        let mut token = self.tokens.next().ok_or_else(|| AstParserError::UnexpectedEof)?.clone();
        if token.value != token_value {
            self.tokens.prev();
            return Err(AstParserError::TokenExpected(token.value, format!("{}", &token_value)));
        }

        Ok(token)
    }

    /// If-Statement
    /// if <bracket_open_token> ... <expression> ... <bracket_close_token>
    // fn try_parse_if_statement(&mut self) -> Result<HuleStatement, AstParserError> {
    // }

    /// Bracket-Expression
    /// <bracket_open_token> <expression> <bracket_close_token>
    fn try_parse_bracket_expression(&mut self) -> Result<HuleExpression, AstParserError> {
        self.expect_token_type(TokenType::Semicolon)
            .or_else(|_| Err(AstParserError::IncompatibleStatement))?;

        let result = self.try_parse_expression()
            .or_else(|_| Err(AstParserError::IncompatibleStatement))?;

        self.expect_token_type(TokenType::Semicolon)?;

        Ok(result)
    }

    fn try_parse_expression(&mut self) -> Result<HuleExpression, AstParserError> {
        let current_index = self.tokens.get_current_token_index();

        let test = self.try_parse_bracket_expression()
            .or_reset(self, current_index).or_else(|_| self.try_parse_bracket_expression());

        test
    }

    fn try_parse_var_decl(&mut self) -> Result<HuleStatement, AstParserError> {
        let var_type = self.expect_token_type(TokenType::Identifier)
            .map_err(|_| AstParserError::EndOfFile)?;

        let var_name = self.expect_token_type(TokenType::Identifier)?;

        let mut statement_type_token = self.expect_token_type(TokenType::Semicolon)
            .or_else(|_| self.expect_token_type(TokenType::Assign))?;

        let mut var_value = None;
        if statement_type_token.get_token_type() == TokenType::Assign {
            var_value = Some(self.try_parse_expression()?);
        }

        Ok(HuleStatement::VariableDecl {
            data_type: var_type.value,
            name: var_name.value,
            value: var_value
        })
    }

    fn try_parse_local_body(&mut self) -> Result<Vec<HuleStatement>, AstParserError> {
        let result : Vec<HuleStatement> = vec![];
        let test = self.try_parse_var_decl() // Try other internal parsers
            ;//.or_else(|_| self.try_parse_if_statement());

        Err(AstParserError::IncompatibleStatement)
    }

    fn try_parse_func_params(&mut self) -> Result<Vec<HuleParameter>, AstParserError> {
        let mut result : Vec<HuleParameter> = vec![];
        let remember_start = self.tokens.remember();
        loop {
            let mut param_type = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
            if param_type.get_token_type() != TokenType::Identifier {
                self.tokens.forget_until(remember_start);
                return Err(AstParserError::IncompatibleStatement);
            }

            let mut param_name = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
            if param_name.get_token_type() != TokenType::Identifier {
                self.tokens.forget_until(remember_start);
                return Err(AstParserError::TokenExpected("identifier".to_string(), param_name.value));
            }

            // ended, more params or invalid token
            let mut general_token = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
            if  general_token.get_token_type() == TokenType::Comma || general_token.get_token_type() == TokenType::BracketClose {
                result.push(HuleParameter {
                    data_type: param_type.value.to_string(),
                    name: param_name.value.to_string(),
                });

                if general_token.get_token_type() == TokenType::BracketClose {
                    self.tokens.prev();
                    break;
                }
            } else {
                self.tokens.forget_until(remember_start);
                return Err(AstParserError::TokenExpected(", or )".to_string(), general_token.value));
            }
        }

        Ok(result)
    }

    fn try_parse_function_decl(&mut self) -> Result<HuleStatement, AstParserError> {
        let remember_start = self.tokens.remember();

        // return type
        // let mut func_ret_type = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
        // if func_ret_type.get_token_type() != TokenType::Identifier {
        //     return Err(AstParserError::IncompatibleStatement);
        // }

        let mut func_ret_type = self.expect_token_type(TokenType::Identifier)
            .map_err(|_| AstParserError::IncompatibleStatement)?;

        // function name
        let mut func_name = self.expect_token_type(TokenType::Identifier)?;

        // bracket open
        let mut general_token = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
        if general_token.get_token_type() != TokenType::BracketOpen {
            return Err(AstParserError::TokenExpected("(".to_string(), general_token.value));
        }

        // parameters
        let mut parameters = self.try_parse_func_params()?.clone();
        println!("found {} params", parameters.len());


        // bracket close
        let mut general_token = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
        if general_token.get_token_type() != TokenType::BracketClose {
            return Err(AstParserError::TokenExpected(")".to_string(), general_token.value));
        }

        // curly bracket open
        let mut general_token = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
        if general_token.get_token_type() != TokenType::CurlyBracketOpen {
            return Err(AstParserError::TokenExpected("{".to_string(), general_token.value));
        }


        // body

        // curly bracket close
        let mut general_token = self.tokens.next().ok_or_else(|| AstParserError::IncompatibleStatement)?.clone();
        if general_token.get_token_type() != TokenType::CurlyBracketClose {
            return Err(AstParserError::TokenExpected("}".to_string(), general_token.value));
        }

        Ok(HuleStatement::FunctionDef {
            name: func_name.value.clone(),
            return_type: func_ret_type.value.clone(),
            parameters,
            body: vec![],
        })
    }

    pub fn new(source : &str) -> AstParser {
        AstParser {
            tokens: Tokenized::new(),
            source: "".to_string(),
        }
    }

    pub fn parse(&mut self, source : &str) -> Result<HuleProgramAst, AstParserError> {
        let mut result = HuleProgramAst {
            body: vec![]
        };

        self.source = source.to_owned();

        let mut tokenizer = Tokenizer::new();
        self.tokens = tokenizer.tokenize(&self.source);

        let last_error : Option<ParseError> = None;

        loop {
            let current_index = self.tokens.get_current_token_index();

            let res = self.try_parse_var_decl()
                .or_reset(self, current_index).or_else(|_| self.try_parse_function_decl())
                .or_reset(self, current_index).or_else(|_| self.try_parse_function_decl());

            match res {
                Ok(statement) => {
                    result.body.push(statement.clone());
                    println!("Added statement to program: {:?}", statement);
                }
                Err(err) => {
                    if let AstParserError::IncompatibleStatement = err {
                        break;
                    } else if let AstParserError::EndOfFile = err {
                        break;
                    } else {
                        println!("Error: {:?}", err.to_message());
                        return Err(err);
                    }
                }
            }

            if !self.tokens.is_currently_in_range() {
                break;
            }
        }

        Ok(result)
    }
}