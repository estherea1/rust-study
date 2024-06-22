//迭代器iterator
use std::env;
//处理文件
use std::fs;

fn main() {
    //获取参数值并存入Vec<String>
    let args: Vec<String> = env::args().collect();
    
    //获取两个参数存为query, file_path
    let query = &args[1];
    let file_path = &args[2];

    println!("搜索关键词：{query}");
    println!("在文件{file_path}中搜索");

    //fs.read_to_string(file_path) 返回包含其内容的 std::io::Result<String>
    let contents = fs::read_to_string(file_path)
        .expect("文件应该被可读");

    println!("内容：{contents}");

}