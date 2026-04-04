//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use mysql::{Conn, Opts, OptsBuilder, error::Error};

/// An [`r2d2`] connection manager for [`mysql`] connections.
#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    params: Opts,
}

impl MySqlConnectionManager {
    /// Constructs a new MySQL connection manager from `params`.
    pub fn new(params: OptsBuilder) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
        }
    }
}

impl r2d2::ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Conn, Error> {
        Conn::new(self.params.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Error> {
        conn.ping()
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}
