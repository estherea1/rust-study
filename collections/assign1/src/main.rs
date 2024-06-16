//返回中位数和众数
use std::collections::HashMap;

fn median(numbers: &Vec<i32>) -> f32 {
    let mut numbers =numbers.clone();
    //排序
    numbers.sort();
    let len =numbers.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (numbers[mid - 1] + numbers [mid]) as f32 / 2.0
    } else {
        numbers[len/2] as f32
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = numbers[0];
    let mut max = 0;
    for(&num,&count) in &map {
        if count > max {
            max = count;
            mode = num;
        }
    }
    mode
}

fn main() {
    let numbers = vec![1,2,3,3,3,4,5,5,6,7,8,9,10];
    let median = median(&numbers);
    let mode = mode(&numbers);
    println!("中位数: {}, 众数: {}", median, mode);
}
