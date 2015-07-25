# r2d2-mysql
[`rust-mysql-simple`](https://github.com/blackbeam/rust-mysql-simple) support library for the [`r2d2`](https://github.com/sfackler/r2d2) connection pool.
Documentation is available at [http://outersky.github.io/r2d2-mysql/doc/v0.2.0/r2d2_mysql](http://outersky.github.io/r2d2-mysql/doc/v0.2.0/r2d2_mysql)

#### Install
Just include another `[dependencies.*]` section into your Cargo.toml:

```toml
[dependencies.r2d2_mysql]
git = "https://github.com/outersky/r2d2-mysql"
version="0.1.0"
```
#### Example

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