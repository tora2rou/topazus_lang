use crate::tokeniser::Token;

pub enum ParseError{
    //関数が存在しない
    ExpectedNotExistenceFunction,
    //定義エラー
    ExpectedDefinitionError,
    //main関数未定義エラー
    ExpectedMainFunctionDefinitionError,
}

enum Literal{
    Str(String),
    Int(i64),
    Flt(f64),
}

trait Expression{
    //後で考える
}


fn parse_expression(iterator: impl Iterator<Item = Token>) -> Box<dyn Expression>{
    if let Some(top) = iterator.next(){
        match top{
            Token::Int(num) => Literal::Int(num),
            Token::Flt(num) => Literal::Flt(num),
            Token::Add => ,
            Token::Substract => ,
            Token::Multiply => ,
            Token::Division => ,
            Token::Surplus => ,
            Token::Ident(any) => {},
            _ => ,
         }
    }
}

pub fn parse(input:Vec<Token>){
    let mut tokens = input.iter().peekable();
    //(
    if let (Some(Token::Lparentheses)) = (tokens.next()) {
        Ok(())
    }
    else{
        Err(ParseError::ExpectedNotExistenceFunction)
    }?;
    if let Some(next) = tokens.peek(){
        match(next){
            Token::Ident(any) => if any == "let"{
                //(let [type] name <- data)
                // (let [fn] name <- [type] [type var] -> | process |)
                tokens.next();
                //[type]
                if let (Some(Token::Lbrackets),Some(Token::Ident(kind)),Some(Token::Rbrackets)) = (tokens.next(),tokens.next(),tokens.next()){
                    //name <-
                    if let(Some(Token::Ident(ident_name)),Some(Token::LeftArrow)) = (tokens.next(),tokens.next()){
                        if let Some(Token::Lbrackets) =  tokens.next(){

                        }
                        else{

                        }
                    }
                }
            },
            Token::Ident(any) => None,
                    //(any data),
            //[ToDo]無名関数作成用の関数を作成する
            Token::Lbrackets => None,
            _ => None,
        }
    }
}
