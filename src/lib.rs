// 这里编写一个两个数相加的函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hello() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
