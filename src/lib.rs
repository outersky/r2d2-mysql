//! MySQL support for the [`r2d2`] connection pool (Rust).
//!
//! # Examples
//! ```
//! use std::{env, sync::Arc, thread};
//! use mysql::{prelude::*, Opts, OptsBuilder};
//! use r2d2_mysql::MysqlConnectionManager;
//!
//! let url = env::var("DATABASE_URL").unwrap();
//! let opts = Opts::from_url(&url).unwrap();
//! let builder = OptsBuilder::from_opts(opts);
//! let manager = MysqlConnectionManager::new(builder);
//! let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
//!
//! let mut tasks = vec![];
//!
//! for _ in 0..3 {
//!     let pool = pool.clone();
//!     let th = thread::spawn(move || {
//!         let mut conn = pool.get().expect("error getting connection from pool");
//!
//!         let _ = conn
//!             .query("SELECT version()")
//!             .map(|rows: Vec<String>| rows.is_empty())
//!             .expect("error executing query");
//!     });
//!
//!     tasks.push(th);
//! }
//!
//! for th in tasks {
//!     let _ = th.join();
//! }
//! ```

pub mod pool;
pub use pool::MysqlConnectionManager;

#[cfg(test)]
mod test {
    use std::{env, sync::Arc, thread};

    use mysql::{prelude::*, Opts, OptsBuilder};

    use super::MysqlConnectionManager;

    #[test]
    fn query_pool() {
        let url = env::var("DATABASE_URL").unwrap();
        let opts = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
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
}
