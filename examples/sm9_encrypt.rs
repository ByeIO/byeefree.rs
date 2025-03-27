#![allow(unused)]
#![feature(string_from_utf8_lossy_owned)]

//! 使用sm9非对称加解密算法加解密

// 标准库
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::fs::File;

// 嵌入文件
use embed_file::embed_bytes;

// sm9算法
use sm9::*;

const USR_ID : [u8; 3] = *b"Bob";
const TXT : [u8; 20] = *b"Chinese IBE standard";

fn main(){
    // 嵌入二进制文件到编译产物中 
    let mut master_public_key_bytes = embed_bytes!("../assets/master_public_key.pem");
    let mut bob_private_key_bytes = embed_bytes!("../assets/bob_private_key.pem");
    
    // 使用sm9加解密
    use std::fs;
    let mut master_public_key = String::from_utf8_lossy_owned(master_public_key_bytes.into());
    let m = Sm9::encrypt2(&master_public_key.as_str(), &USR_ID, &TXT);
    println!("{:02X?}", m);

    let mut bob_private_key =  String::from_utf8_lossy_owned(bob_private_key_bytes.into());
    let msg = Sm9::decrypt2(&bob_private_key, &USR_ID, m).expect("decrypt error");
    println!("{:02X?}", msg);
    assert_eq!(msg.len(), TXT.len());
    assert_eq!(TXT, msg.as_slice());
}
