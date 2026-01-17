use std::char;

#[derive(Debug)]

pub enum Token{
    Ident(String), // 識別子
    Lparentheses, // (
    Rparentheses, // )
    Lbraces, // {
    Rbraces, // }
    Lbrackets, //[
    Rbrackets, //]
    Semicolon, // ;
    Colon, // :
    Quotation, // '
    DoubleQuotation, // "
    Sharp, // #
    BackSlash, // \
    Dot, // .
    Comma, // ,

    Flt(f64), // ふろぉと
    Int(i64), // いんと

    //Arithmetic Operators
    Add, // +
    Substract, // -
    Multiply, // *
    Division, // /
    Surplus, // %

    //Logical Operators
    Not, // !
    And, // &&
    Or, // ||

    //Comparison Operators
    Smaller, // <
    Greater, // >
    SmallerOrEqual, // <=
    GreaterOrEqual, // >=
    Equal, // =
    NotEqual, // !=

    Funcdef,
    LeftArrow,
    RightArrow,

    Eof, // End Of File
}

fn is_able_identifier(input:char) -> bool{
    ('a' <= input && input <='z') || ('A' <= input && input <= 'Z') || (input == '_')
}

pub fn lexer(input: String) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut chars = input.chars().peekable();
    let mut tokens = vec![];//Vec::new()
    while let Some(current) = chars.next() {
        if current.is_whitespace(){continue;}
        match current {
            '(' => tokens.push(Token::Lparentheses),
            ')' => tokens.push(Token::Rparentheses),
            '{' => tokens.push(Token::Lbraces),
            '}' => tokens.push(Token::Rbraces),
            '[' => tokens.push(Token::Lbrackets),
            ']' => tokens.push(Token::Rbrackets),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            '\'' => tokens.push(Token::Quotation),
            '\"' => tokens.push(Token::DoubleQuotation),
            '#' => tokens.push(Token::Sharp),
            '\\' => tokens.push(Token::BackSlash),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '+' => tokens.push(Token::Add),
            '-' => if let Some(next) = chars.peek() && *next == '>'{
                        chars.next();
                        tokens.push(Token::RightArrow)
                    }
                   else{
                       tokens.push(Token::Substract)
                    },
            '/' => if let Some(next) = chars.peek() && *next == '/' {
                    while let Some(consumed) = chars.next() && consumed != '\n'{}
                    }else{
                        tokens.push(Token::Division)
                    },
            '*' => tokens.push(Token::Multiply),
            '%' => tokens.push(Token::Surplus),
            '!' => if let Some(next) = chars.peek() && *next == '=' {
                        chars.next();
                        tokens.push(Token::NotEqual)
                    }
                   else{
                       tokens.push(Token::Not)
                    },
            '&' => if let Some(next) = chars.peek() && *next == '&' {
                        chars.next();
                        tokens.push(Token::And)
                    },
            '|' => if let Some(next) = chars.peek() && *next == '|' {
                        chars.next();
                        tokens.push(Token::Or)
                        }
                   else{
                        tokens.push(Token::Funcdef)
                    },
            '<' => if let Some(next) = chars.peek() &&  *next == '=' {
                        chars.next();
                        tokens.push(Token::SmallerOrEqual)
                    }
                   else if let Some(next) = chars.peek() && *next == '-'{
                       chars.next();
                       tokens.push(Token::LeftArrow)
                   }
                   else{
                       tokens.push(Token::Smaller)
                   },
            '>' => if let Some(next) = chars.peek() && *next == '=' {
                        chars.next();
                        tokens.push(Token::GreaterOrEqual)
                    }
                   else{
                       tokens.push(Token::Greater)
                    },
            '=' => tokens.push(Token::Equal),
            otherwise => {
                if is_able_identifier(otherwise){
                    let mut ident = String::from(otherwise);
                    while let Some(&next) = chars.peek(){
                        if is_able_identifier(next){
                            ident.push(next);
                            chars.next();
                        } else{
                            break;
                        }
                    }
                    tokens.push(Token::Ident(ident));

                } else if otherwise.is_ascii_digit(){
                    let mut num = String::from(otherwise);
                    while let Some(&next) = chars.peek() {
                        if next.is_ascii_digit()|| next == '.'{
                            num.push(next);
                            chars.next();
                        }
                        else{
                            break;
                        }
                    }
                    if num.contains("."){
                        println!("{}", num);
                        let flt_num = num.trim().parse::<f64>();
                        tokens.push(Token::Flt(flt_num?));
                    }else{
                        let int_num = num.trim().parse::<i64>();
                        tokens.push(Token::Int(int_num?));
                    }
                }
            },
        }
    }
    println!("{:?}",tokens);
    Ok(tokens)
}
