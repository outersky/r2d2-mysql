use mysql::error::Error;
use mysql::{Conn, Opts, OptsBuilder};
use std::result::Result;
use r2d2;

#[derive(Clone, Debug)]
pub struct MysqlConnectionManager {
    params: Opts,
}

impl MysqlConnectionManager {
    pub fn new(params: OptsBuilder) -> MysqlConnectionManager {
        MysqlConnectionManager {
            params: Opts::from(params),
        }
    }
}

impl r2d2::ManageConnection for MysqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Conn, Error> {
        Conn::new(self.params.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Error> {
        if conn.ping() {
            return Ok(());
        } else {
            return Err(self::Error(None));
        }
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}
