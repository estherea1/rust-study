pub fn add_two(a: i32) -> i32 {
    a+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2不等于4"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4,add_two(2));
    }

    /*#[test]
    fn another() {
        panic!("测试失败");
    }*/
}
