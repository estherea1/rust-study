fn main() {
    //创建s
    let mut s1 = String::new();

    let data = "to_String创建";

    //使用to_string方法初始化数据
    let s2 = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s3 = "字符串to_String".to_string();

    //使用String::from创建
    let mut hi = String::from("你好");
    //使用push_str来附加slice
    let name="123";
    hi.push_str(name);
    println!("名字是{name},{hi}");

    //使用push将一个字符加入String
    let mut lose = String::from("lo");
    lose.push('l');

    //使用add，s1+&s2
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let add = hello + &world;

    //使用format! marco
    let word1 = String::from("tic");
    let word2 = String::from("tac");
    let word3 = String::from("toe");

    let full = format!("{word1}-{word2}-{word3}");

    //字符串索引slice
    let temp = "你好你好";
    let cut_temp = &temp[0..4];
    //chars方法
    for c in cut_temp.chars() {
        println!("{c}");
    }
    //bytes方法
    for b in cut_temp.bytes() {
        println!("{b}");
    }
}
