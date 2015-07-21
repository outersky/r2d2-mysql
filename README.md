# r2d2-mysql
MySQL support for the r2d2 connection pool (Rust) . see [`r2d2`](http://github.com/sfackler/r2d2.git)  .

#### Install
Just include another `[dependencies.*]` section into your Cargo.toml:

```toml
[dependencies.r2d2_mysql]
git = "https://github.com/outersky/r2d2-mysql"
version="0.1.0"
```
#### Sample

```rust,no_run
extern crate r2d2_mysql;
extern crate r2d2;

use std::sync::Arc;
use std::thread;

fn main() {
	let db_url =  "mysql://root:12345678@localhost:3306/test";
    let config = r2d2::config::Builder::new().pool_size(5).build();   // r2d2::Config::default()
    let manager = r2d2_mysql::MysqlConnectionManager::new(db_url).unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

    let mut tasks = vec![];

    for i in 0..3 {
        let pool = pool.clone();
        let th = thread::spawn(move || {
            println!("thread {} running" , i );
            let mut conn = pool.get().unwrap();
            conn.query("select user()").unwrap();
            println!("thread {} end!" , i );
        });
        tasks.push(th);
    }

    for th in tasks {
        let _ = th.join();
    }
}
```

#### Sample2

convert a `Stmt` to `RowSet`, and fetch data using `Row.get("column_name")`

```rust,no_run
extern crate r2d2_mysql;
extern crate mysql;
extern crate r2d2;
extern crate time;

use std::fmt;
use std::sync::Arc;
use std::thread;
use time::Timespec;

pub struct Person {
    id: i64,
    name:String,
    create_time:Timespec,  // table column type: datetime
    not_exist:i64,
}

impl Person {
    fn _fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Person{{id:{}, name:{}, create_time:{}, not_exist:{} }}"#, self.id, self.name, time::at(self.create_time).rfc822(), self.not_exist)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._fmt(f)
    }
}

fn main() {
	let db_url =  "mysql://root:12345678@localhost:3306/test";
    let config = r2d2::config::Builder::new().pool_size(10).build();   // r2d2::Config::default()
    let manager = r2d2_mysql::MysqlConnectionManager::new(db_url).unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

    let mut tasks = vec![];

    for _ in 0..3 {
        let pool = pool.clone();
        let th = thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            test(&mut *conn);
        });
        tasks.push(th);
    }

    for th in tasks {
        let _ = th.join();
    }
}

fn test<'a>(conn: &'a mut mysql::conn::MyConn){
    conn.query(r#"CREATE TEMPORARY TABLE tbl_person(
                                id INT,
                                name varchar(30),
                                create_time DATETIME
                            )"#).is_ok();

    let _ = conn.prepare("INSERT INTO tbl_person(id,name,create_time) VALUES (?, ?, now())")
            .map(|mut stmt| {
                assert!(stmt.execute(&[&1,&b"tom".to_vec(),]).is_ok());
                assert!(stmt.execute(&[&2,&b"amy".to_vec(),]).is_ok());
            }).unwrap();

    let sql = "select id,name,create_time from tbl_person";
    conn.prepare(sql).map(|mut stmt|{
        let columns = r2d2_mysql::get_columns(& stmt.columns_ref()); // Vec<Column>
        let rowset = stmt.execute(&[]).map(|qr| r2d2_mysql::to_rowset(qr,columns) );  // Option<Rc<RowSet>>
        rowset.map(|rowset| {
            let rows = rowset.rows.borrow();
            let people = rows.iter().map(|row|
                Person {
                    id: row.get("id"),
                    name:row.get("name"),
                    create_time: row.get("create_time"),
                    not_exist:row.get_opt("not_exist").unwrap_or(-1),
                }
            ).collect::<Vec<Person>>();
            println!("People: {:?} ", people);
        })
    }).is_ok();
    
}
```