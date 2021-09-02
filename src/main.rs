mod token;

fn main() {
    let a = token::Token {
        cat: String::from("letter"),
        literal: String::from("A"),
    };

    println!("{} / {}", a.cat, a.literal);
    println!("{}", token::sum_in_module(100, 200))
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sum() {
        assert_eq!(100 + 200, super::sum(100, 200))
    }

    #[test]
    fn token_sum() {
        assert_eq!(100 + 200, super::token::sum_in_module(100, 200))
    }
}