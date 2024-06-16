fn main() {
    //创建Vec<i32>集合
    let v1: Vec<i32> = Vec::new();

    //vec! marco创建v2
    let v2 = vec![1, 2, 3];

    //使用for循环遍历不可变vector的元素
    for i in &v2 {
        println!("{i}");
    }

    //创建可变v3
    let mut v3 = Vec::new();
    //使用push向vector增加元素
    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.push(4);

    //通过索引读取vector v3中的值
    let third: &i32 = &v3[2];
    println!("v3第三个值是： {third}");

    //通过get获取vector v3的值
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("第三个值是： {third}"),
        None => println!("不存在第三个元素"),
    }
    //当v3第五个值不存在时
    let fifth: Option<&i32> =v3.get(5);
    match fifth {
        Some(fifth) => println!("第五个值是： {fifth}"),
        None => println!("不存在第五个元素"),    
    }

    //使用for遍历可变vector
    for i in &mut v3 {
        //给每个元素加50
        *i += 50;
    }

    //使用枚举存储不同类型的值
    enum MultiType {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let multi_data = vec![
        MultiType::Int(3),
        MultiType::Text(String::from("blue")),
        MultiType::Float(10.12),
    ];

    /*Implement the Display trait for the MultiType enum
    for i in &multi_data {
        println!("{i}");
    }
    */


}
