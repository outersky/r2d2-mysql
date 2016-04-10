//! # r2d2-mysql
//! MySQL support for the r2d2 connection pool (Rust) . see [`r2d2`](http://github.com/sfackler/r2d2.git)  .
//! 
//! #### Install
//! Just include another `[dependencies.*]` section into your Cargo.toml:
//!
//! ```toml
//! [dependencies.r2d2_mysql]
//! git = "https://github.com/outersky/r2d2-mysql"
//! version="0.2.0"
//! ```
//! #### Sample
//!
//! ```
//! extern crate r2d2_mysql;
//! extern crate r2d2;
//! 
//! use std::sync::Arc;
//! use std::thread;
//! 
//! fn main() {
//! 	let db_url =  "mysql://root:12345678@localhost:3306/test";
//!     let config = r2d2::config::Builder::new().pool_size(5).build();   // r2d2::Config::default()
//!     let manager = r2d2_mysql::MysqlConnectionManager::new(db_url).unwrap();
//!     let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
//! 
//!     let mut tasks = vec![];
//! 
//!     for i in 0..3 {
//!         let pool = pool.clone();
//!         let th = thread::spawn(move || {
//!             let mut conn = pool.get().unwrap();
//!             conn.query("select user()").unwrap();
//!             println!("thread {} end!" , i );
//!         });
//!         tasks.push(th);
//!     }
//! 
//!     for th in tasks {
//!         let _ = th.join();
//!     }
//! }
//! ```
//! 

#![doc(html_root_url="http://outersky.github.io/r2d2-mysql/doc/v0.2.0/r2d2_mysql/")]
#![crate_name="r2d2_mysql"]
#![crate_type="rlib"]
#![crate_type="dylib"]

extern crate mysql;
extern crate rustc_serialize as serialize;
extern crate r2d2;

pub mod url;
pub mod pool;

pub use pool::MysqlConnectionManager;

#[cfg(test)]
mod test {
    use r2d2;
    use std::sync::Arc;
    use std::thread;
    use super::{MysqlConnectionManager};

    const DB_URL : &'static str =  "mysql://root:12345678@localhost:3306/test";

    #[test]
    fn query_pool(){
        let config = r2d2::config::Builder::new().pool_size(30).build();   // r2d2::Config::default()
        let manager = MysqlConnectionManager::new(DB_URL).unwrap();
        let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let th = thread::spawn(move || {
                let mut conn = pool.get().map_err(|err| println!("get connection from pool error in line:{} ! error: {:?}", line!(), err) ).unwrap();
                conn.query("select user()").map_err(|err| println!("execute query error in line:{} ! error: {:?}", line!(), err) ).unwrap();
            });
            tasks.push(th);
        }

        for th in tasks {
            let _ = th.join();
        }

    }
}
