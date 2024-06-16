fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Team1"), 50);
    scores.insert(String::from("Team2"), 100);

    //覆盖旧值
    scores.insert(String::from("Team1"),99);

    //在键没有对应值时插入键值对
    scores.entry(String::from("Team1",)).or_insert(50);//无用
    scores.entry(String::from("Team3")).or_insert(50);

    //get获取值get(&key)
    //copied获取Option<i32>，而不是Option<&i32>
    //unwarp_or：没有该键对应的项是将其设置为0
    let team_name = String::from("Team1");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("队{team_name}的分数是{score}");

    //循环遍历
    for(key,value) in &scores {
        println!("队名:分数      {key}: {value}");
    }
}
