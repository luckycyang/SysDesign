use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // default cache
    let mut cache_dir = dirs::cache_dir().expect("get Cache dir fault!!!");
    cache_dir.push("lowloght");
    cache_dir.push("lowloght.db");
    println!("cache dir: {:?}", &cache_dir);

    if let Some(parent) = cache_dir.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
        }
    }

    // 创建或连接数据库
    let conn = Connection::open(cache_dir)?;

    // 创建表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id    INTEGER PRIMARY KEY,
                  name  TEXT NOT NULL,
                  data  BLOB
                  )",
        [],
    )?;

    // 插入数据
    let data: Vec<u8> = vec![1, 2, 3];
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params!["Alice", &data],
    )?;

    // 查询数据
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // 处理查询结果
    for person in person_iter {
        println!("Found person {:?}", person?);
    }

    Ok(())
}

// 定义结构体来存储查询结果
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Vec<u8>,
}
