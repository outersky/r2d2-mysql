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
//!
//! # Custom Health Check
//! ```
//! # use r2d2_mysql::{
//! #     mysql::{prelude::*, *},
//! #     MySqlConnectionManager,
//! # };
//! fn health_check(
//!     _: MySqlConnectionManager,
//!     conn: &mut mysql::Conn
//! ) -> Result<(), mysql::Error> {
//!     conn.query("SELECT 1").map(|_: Vec<String>| ())
//! }
//!
//! // ...
//!
//! # let url = std::env::var("DATABASE_URL").unwrap();
//! # let opts = mysql::Opts::from_url(&url).unwrap();
//! # let builder = OptsBuilder::from_opts(opts);
//! let manager = MySqlConnectionManager::with_custom_health_check(builder, &health_check);
//! ```

pub use mysql;
pub use r2d2;

mod pool;
pub use self::pool::MySqlConnectionManager;

#[cfg(test)]
mod test {
    use std::{env, sync::Arc, thread};

    use mysql::{prelude::*, Conn, Error, Opts, OptsBuilder};

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

    #[test]
    fn query_pool_with_custom_health_check() {
        fn health_check(_: MySqlConnectionManager, conn: &mut Conn) -> Result<(), Error> {
            conn.query("SELECT 1").map(|_: Vec<String>| ())
        }

        let url = env::var("DATABASE_URL").unwrap();
        let opts = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::with_custom_health_check(builder, &health_check);
        let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let th = thread::spawn(move || {
                let mut conn = pool.get().expect("error getting connection from pool");

                let _ = conn
                    .query("SELECT 1")
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
