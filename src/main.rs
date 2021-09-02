fn main() {
    println!("Hello, world!");
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
}