use std::fs;
use std::path::PathBuf;


fn main() {
    // 遍历当前文件夹下的所有目录
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path_name = path.unwrap().path();
        let file_name = path_name.file_name().unwrap().to_str().unwrap();
        println!("{}", file_name);
    }
}
