use std::error::Error;
//处理文件
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("没有输入足够的参数");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs.read_to_string(file_path) 返回包含其内容的 std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)?;
    println!("内容：{contents}");
    Ok(())
}