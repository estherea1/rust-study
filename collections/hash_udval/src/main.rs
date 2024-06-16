fn main() {
    use std::collections::HashMap;

    let text = "hello world hello wonderful world hello esthereal hi esthereal";

    //创建可变hashmap
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
