#![allow(unused)]

//! 解包tar文件

use std::io::prelude::*;
use std::fs::File;
use tar::Archive;

fn main() {
    // 打开 tar 文件
    let file = File::open("./assets/frontend.tar").unwrap();
    // 创建一个 Archive 对象来读取 tar 文件内容
    let mut a = Archive::new(file);

    // 遍历 tar 文件中的所有文件
    for file in a.entries().unwrap() {
        // 确保没有 I/O 错误
        let mut file = file.unwrap();

        // 获取文件的元数据并打印文件路径
        println!("{:?}", file.header().path().unwrap());
        // 打印文件大小
        println!("{}", file.header().size().unwrap());

        // 文件实现了 Read trait，可以读取内容
        // let mut s = String::new();
        // 将文件内容读取到字符串中
        // file.read_to_string(&mut s).unwrap();
        // 打印文件内容
        // println!("{}", s);
    }
}
