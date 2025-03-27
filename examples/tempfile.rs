#![allow(unused)]

//! 使用临时文件

// 标准库
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::fs::File;

// 嵌入文件
use embed_file::embed_bytes;
// 临时文件
use tempfile::tempdir;

// sm9算法
use sm9::*;

fn main(){
    // 嵌入二进制文件到编译产物中 
    let mut master_public_key_bytes = embed_bytes!("../assets/master_public_key.pem");
    let mut bob_private_key_bytes = embed_bytes!("../assets/bob_private_key.pem");
    
    // 创建临时目录
    let temp_dir = tempdir().expect("创建临时目录失败");
    let master_public_key_path = temp_dir.path().join("master_public_key.pem");
    let bob_private_key_path = temp_dir.path().join("bob_private_key.pem");
    
    // 将嵌入的字节写入临时文件
    let mut master_public_key_file = File::create(&master_public_key_path).expect("创建临时文件失败");
    let mut bob_private_key_file = File::create(&bob_private_key_path).expect("创建临时文件失败");
    master_public_key_file.write_all(&master_public_key_bytes).expect("写入临时文件失败");
    bob_private_key_file.write_all(&bob_private_key_bytes).expect("写入临时文件失败");
    
    let usr_id = b"Bob";
    let txt = b"Chinese IBE standard";
    
    // 使用临时路径
    let m = Sm9::encrypt(master_public_key_path, usr_id, txt);
    println!("{:02X?}", m);

    // 使用临时路径
    let msg = Sm9::decrypt(bob_private_key_path, usr_id, m).expect("decrypt error");
    println!("{:02X?}", msg);
    assert_eq!(msg.len(), txt.len());
    assert_eq!(txt, msg.as_slice());
}
