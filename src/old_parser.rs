use crate::tokeniser::Token;

pub enum ASTNode {
    //Literal
    Str(String),
    Int(i64),
    Flt(f64),
    Identifier,
    Apply,
    Lambda,
    Control,
}

pub enum ParseError {
    //関数が存在しない
    ExpectedNotExistenceFunction,
    //定義エラー
    ExpectedDefinitionError,
    //main関数未定義エラー
    ExpectedMainFunctionDefinitionError,
}

pub trait Expression {}

pub fn parse(input: Vec<Token>) {
    let mut tokens = input.iter().peekable();
    //(
    if let (Some(Token::Lparentheses)) = (tokens.next()) {
        Ok(())
    } else {
        Err(ParseError::ExpectedNotExistenceFunction)
    }?;
    if let Some(next) = tokens.peek() {
        match (next) {
            Token::Ident(any) => {
                if any == "let" {
                    //(let [type] name <- data)
                    // (let [fn] name <- [type] [type var] -> | process |)
                    tokens.next();
                    //[type]
                    if let (
                        Some(Token::Lbrackets),
                        Some(Token::Ident(kind)),
                        Some(Token::Rbrackets),
                    ) = (tokens.next(), tokens.next(), tokens.next())
                    {
                        //name <-
                        if let (Some(Token::Ident(ident_name)), Some(Token::LeftArrow)) =
                            (tokens.next(), tokens.next())
                        {
                            if let Some(Token::Lbrackets) = tokens.next() {
                            } else {
                            }
                        }
                    }
                }
            }
            Token::Ident(any) => None,
            //(any data),
            //[ToDo]無名関数作成用の関数を作成する
            Token::Lbrackets => None,
            _ => None,
        }
    }
}
