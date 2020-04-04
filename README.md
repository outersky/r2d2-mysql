# r2d2-mysql
[`rust-mysql-simple`](https://github.com/blackbeam/rust-mysql-simple) support library for the [`r2d2`](https://github.com/sfackler/r2d2) connection pool.
Documentation is available at [http://outersky.github.io/r2d2-mysql/doc/v3.0.0/r2d2_mysql](http://outersky.github.io/r2d2-mysql/doc/v3.0.0/r2d2_mysql)

#### Install
Just include another `[dependencies]` section into your Cargo.toml:

```toml
[dependencies]
r2d2_mysql="*"
```
#### Example

```rust,no_run
extern crate mysql;
extern crate r2d2_mysql;
extern crate r2d2;
use std::env;
use std::sync::Arc;
use std::thread;
use mysql::{Opts,OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;

fn main() {
	let db_url =  env::var("DATABASE_URL").unwrap();
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

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
