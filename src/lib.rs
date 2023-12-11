pub mod first;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn consume_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
            f(x)
        }

        let result = add(2, 2);
        assert_eq!(result, 4);
        let take_ownership = |y| {
            println!("Taking ownership of {}", y);
            y * 2
        };
        let result = consume_once(take_ownership, 3);
    }
}
