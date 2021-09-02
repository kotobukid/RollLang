mod token;

fn main() {
    let a = token::Token {
        cat: String::from("letter"),
        literal: String::from("A"),
    };

    println!("{} / {}", a.cat, a.literal);
    println!("{:?}", token::Cat::ILLEGAL);
    // println!("{}", token::Cat::EOF);
    // println!("{}", token::Cat::IDENT);
    // println!("{}", token::Cat::INT);
    // println!("{}", token::Cat::ASSIGN);
    // println!("{}", token::Cat::PLUS);
    // println!("{}", token::Cat::COMMA);
    // println!("{}", token::Cat::SEMICOLON);
    // println!("{}", token::Cat::LPAREN);
    // println!("{}", token::Cat::RPAREN);
    // println!("{}", token::Cat::LBRACE);
    // println!("{}", token::Cat::RBRACE);
    // println!("{}", token::Cat::FUNCTION);
    // println!("{}", token::Cat::LET);
    println!("{}", token::sum_in_module(100, 200))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn token_sum() {
        assert_eq!(100 + 200, super::token::sum_in_module(100, 200))
    }
}