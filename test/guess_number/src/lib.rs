pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("值必须小于等于1, 值是：{value}.");
        } else if value > 100 {
            panic!("值必须大于等于100，值是：{value}");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //为了使 should_panic 测试结果更精确
    //可以给 should_panic 属性增加一个可选的 expected 参数
    #[test]
    #[should_panic(expected = "小于等于100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}