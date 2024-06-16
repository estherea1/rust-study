fn pig_latin(word: &str) -> String {
    let match_aph = ['a','e','i','o','u'];
    let mut result = String::new();
    //第一个字母
    let first_char = word.chars().next().unwrap();
    //元音
    if match_aph.contains(&first_char) {
        result.push_str(&format!("{}-hay", word));
    }else {//辅音
        let remainder: String = word.chars().skip(1).collect();
        result.push_str(&format!("{}-{}ay", remainder, first_char));
    }
    result.to_string()
}
fn main() {
    let test1 = "first";
    let test2 = "apple";
    println!("test1:{}",pig_latin(&test1));
    println!("test2:{}",pig_latin(&test2));
}
