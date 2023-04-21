# r2d2-mysql

> [`mysql`](https://github.com/blackbeam/rust-mysql-simple) support library for the [`r2d2`](https://github.com/sfackler/r2d2) connection pool.

[![crates.io](https://img.shields.io/crates/v/r2d2-mysql?label=latest)](https://crates.io/crates/r2d2-mysql)
[![Documentation](https://docs.rs/r2d2-mysql/badge.svg?version=23)](https://docs.rs/r2d2-mysql/23)
![Version](https://img.shields.io/badge/rustc-1.59+-ab6000.svg)
![License](https://img.shields.io/crates/l/r2d2-mysql.svg)
[![Download](https://img.shields.io/crates/d/r2d2-mysql.svg)](https://crates.io/crates/r2d2-mysql)

## Install

Include `r2d2_mysql` in the `[dependencies]` section of your `Cargo.toml`:

```toml
[dependencies]
r2d2_mysql = "23"
```

## Usage

```rust
use std::{env, sync::Arc, thread};
use mysql::{prelude::*, Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;

fn main() {
    let url = env::var("DATABASE_URL").unwrap();
    let opts = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    let mut tasks = vec![];

    for _ in 0..3 {
        let pool = pool.clone();
        let th = thread::spawn(move || {
            let mut conn = pool.get().expect("error getting connection from pool");

            let _ = conn
                .query("SELECT version()")
                .map(|rows: Vec<String>| rows.is_empty())
                .expect("error executing query");
        });

        tasks.push(th);
    }

    for th in tasks {
        let _ = th.join();
    }
}
```

### Custom healthcheck function
If in case for some reason your server don't support `SELECT version()` you can override the default healthcheck function:
```rust
use std::{env, sync::Arc, thread};
use mysql::{prelude::*, Conn, Error, Opts, OptsBuilder};

fn healthcheck(_: MySqlConnectionManager, conn: &mut Conn) -> Result<(), Error> {
    conn.query("SELECT 1").map(|_: Vec<String>| ())
}

fn main() {
    // [ .. ]
    let manager = MySqlConnectionManager::with_custom_healthcheck(builder, &healthcheck);
    // [ .. ]
}
```
