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
use mysql::prelude::Queryable;
use r2d2_mysql::MysqlConnectionManager;

fn main() {
	let url = env::var("DATABASE_URL").unwrap();
        let opts = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
        let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let th = thread::spawn(move || {
                let mut conn = pool.get()
                    .map_err(|err| {
                        println!(
                            "get connection from pool error in line:{} ! error: {:?}",
                            line!(),
                            err
                        )
                    })
                    .unwrap();
                let _ = conn.query("SELECT version()").map(|_: Vec<String>| ()).map_err(|err| {
                    println!("execute query error in line:{} ! error: {:?}", line!(), err)
                });
            });
            tasks.push(th);
        }

    for th in tasks {
        let _ = th.join();
    }
}
```
