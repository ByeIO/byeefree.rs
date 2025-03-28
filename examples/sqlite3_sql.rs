#![allow(unused)]

//! 使用prsqlite数据库(sqlite3)数据库进行SQL语句测试

// 标准库
use std::path::Path;

// sqlite3数据库
use prsqlite::Connection;
use prsqlite::Value;
// use prsqlite::test_utils::create_sqlite_database;

// 临时文件
use tempfile::NamedTempFile;

pub fn create_sqlite_database(queries: &[&str]) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    let conn = Connection::open(file.path()).unwrap();
    for query in queries {
        // conn.execute(query, []).unwrap();
    }
    // conn.close().unwrap();
    file
}

fn main(){
    test_select_all_from_table();
    // // 连接数据库
    // let conn = Connection::open(Path::new("./assets/sqlite3_sql.db")).unwrap();
    
    // // 插入列数据
    // let stmt = conn.prepare("INSERT INTO example (col) VALUES (1), (2);").unwrap();
    // assert_eq!(stmt.execute().unwrap(), 2);
    
    // // 查询语句
    // let stmt = conn.prepare("SELECT * FROM example WHERE col = 1;").unwrap();
    // let mut rows = stmt.query().unwrap();
    
    // // 获取数据
    // let row = rows.next_row().unwrap().unwrap();
    // let columns = row.parse().unwrap();
    // assert_eq!(columns.get(0), Some(&Value::Integer(1)));
    // drop(row);
    
    // assert!(rows.next_row().unwrap().is_none());
}

fn test_select_all_from_table() {
    let mut queries = vec![
        "CREATE TABLE example(col);",
        "CREATE TABLE example2(col1, col2);",
        "CREATE TABLE example3(col1, col2, col3);",
        "INSERT INTO example3(col1, col2, col3) VALUES (null, true, false);",
        "INSERT INTO example3(col1, col3) VALUES (10000, \"hello\");",
    ];
    let blob_query = format!(
        "INSERT INTO example3(col1, col2) VALUES (X'{}', 20000);",
        "FF".repeat(10000)
    );
    queries.push(&blob_query);
    let file = create_sqlite_database(&queries);

    let conn = Connection::open(file.path()).unwrap();
    let stmt = conn.prepare("SELECT * FROM example3;").unwrap();
    let mut rows = stmt.query().unwrap();

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    assert_eq!(columns.len(), 3);
    assert_eq!(columns.get(0), None);
    assert_eq!(columns.get(1), Some(&Value::Integer(1)));
    assert_eq!(columns.get(2), Some(&Value::Integer(0)));
    assert_eq!(columns.get(3), None);
    drop(row);

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    assert_eq!(columns.len(), 3);
    assert_eq!(columns.get(0), Some(&Value::Integer(10000)));
    assert_eq!(columns.get(1), None);
    assert_eq!(
        columns.get(2),
        Some(&Value::Text(b"hello".as_slice().into()))
    );
    assert_eq!(columns.get(3), None);
    drop(row);

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    assert_eq!(columns.len(), 3);
    assert_eq!(
        columns.get(0),
        Some(&Value::Blob([0xFF; 10000].as_slice().into()))
    );
    assert_eq!(columns.get(1), Some(&Value::Integer(20000)));
    assert_eq!(columns.get(2), None);
    assert_eq!(columns.get(3), None);
    drop(row);

    assert!(rows.next_row().unwrap().is_none());
}
