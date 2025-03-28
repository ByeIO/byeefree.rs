#![allow(unused)]

//! 用户登录
//! 登录流程: 用户

// 标准库
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::fs::File;

// 嵌入文件
use embed_file::embed_bytes;

// sm9算法
use sm9::*;

// 唯一识别码
use uuid::Uuid;

// /// 用户账户数据
// #[derive(Default, Debug, PartialEq)]
// pub struct UserAccountStruct {
//     /// 用户唯一识别码
//     pub uuid: Uuid,
//     /// 用户名
//     pub name: String,
//     /// sm9公钥
//     pub sm9_pub_key: String,
// }

// implement_from_tuple!(
//     UserAccountStruct, (
//         uuid: Uuid => |inner: &mut UserAccountStruct, value| {
//             if let DataValue::Utf8 { value, .. } = value {
//                 if let Ok(u) = Uuid::parse_str(&value) {
//                     inner.uuid = u;
//                 }
//             }
//         },
//         name: String => |inner: &mut UserAccountStruct, value| {
//             if let DataValue::Utf8 { value, .. } = value {
//                 inner.name = value;
//             }
//         },
//         sm9_pub_key: String => |inner: &mut UserAccountStruct, value| {
//             if let DataValue::Utf8 { value, .. } = value {
//                 inner.sm9_pub_key = value;
//             }
//         }
//     )
// );

// pub fn account_login_cmd(name : Option<String>) -> Result<(), DatabaseError> {
//     // TODO 如果用户名为空...
//     // let name = name.ok_or(Ok(()));
//     if name.clone().expect("REASON").is_empty() {
//         return Ok(());
//     }
    
//     // 嵌入二进制文件到编译产物中 
//     let master_public_key_bytes = embed_bytes!("../../../assets/master_public_key.pem");
//     let bob_private_key_bytes = embed_bytes!("../../../assets/bob_private_key.pem");
//     let database_bytes = embed_bytes!("../../../assets/database_login.db");
    
//     // 获取公私钥
//     let master_public_key = String::from_utf8_lossy_owned(master_public_key_bytes.into());
//     let bob_private_key =  String::from_utf8_lossy_owned(bob_private_key_bytes.into());
    
//     /* start 创建用户 */
//     // 创建数据库实例
//     let database = DataBaseBuilder::path("./result/database_login.db").build()?;
    
//     // 创建用户表（如果不存在）
//     database.run(
//         "CREATE TABLE IF NOT EXISTS user_accounts (
//             uuid TEXT PRIMARY KEY,
//             name TEXT UNIQUE,
//             sm9_pub_key TEXT
//         )"
//     )?.done()?;

//     // 检查用户Bob是否存在
//     let mut exists = false;
//     let check_iter = database.run("SELECT * FROM user_accounts WHERE name = 'Bob'")?;
//     for _ in check_iter {
//         exists = true;
//         break;
//     }

//     // 不存在则插入新用户
//     if !exists {
//         let new_uuid = Uuid::new_v4().hyphenated().to_string();
//         let insert_sql = format!(
//             "INSERT INTO user_accounts VALUES ('{}', 'Bob', '{}')",
//             new_uuid, master_public_key
//         );
//         database.run(&insert_sql)?.done()?;
//         println!("用户{}创建成功", name.clone().expect("REASON"));
//     }
//     /* end 创建用户 */
    
//     /* start 模拟用户登录 */
//     // 使用主公钥加密测试数据
//     const USR_ID: &[u8] = b"Bob";
//     const TXT: &[u8] = b"Chinese IBE standard";
//     let encrypted = Sm9::encrypt2(&master_public_key, USR_ID, TXT);
    
//     // 使用私钥解密验证
//     let decrypted = Sm9::decrypt2(&bob_private_key, USR_ID, encrypted)
//         .expect("解密失败，密钥不匹配");
    
//     // 验证解密结果
//     assert_eq!(decrypted.as_slice(), TXT);
//     println!("用户Bob登录验证成功!");
//     /* end 模拟用户登录 */
    
//     Ok(())
// }
