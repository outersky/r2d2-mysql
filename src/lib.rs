//! MySQL support for the [`r2d2`] connection pool (Rust).
//!
//! # Examples
//! ```
//! use std::{env, sync::Arc, thread};
//!
//! use r2d2_mysql::{
//!     mysql::{prelude::*, Opts, OptsBuilder},
//!     r2d2, MySqlConnectionManager,
//! };
//!
//! let url = env::var("DATABASE_URL").unwrap();
//! let opts = Opts::from_url(&url).unwrap();
//! let builder = OptsBuilder::from_opts(opts);
//! let manager = MySqlConnectionManager::new(builder);
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

pub use mysql;
pub use r2d2;

mod pool;
pub use self::pool::MySqlConnectionManager;

#[cfg(test)]
mod test {
    use std::{env, sync::Arc, thread};

    use mysql::{prelude::*, Opts, OptsBuilder};

    use super::MySqlConnectionManager;

    #[test]
    fn query_pool() {
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
}
