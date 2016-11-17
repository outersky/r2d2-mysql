use mysql::error::Error;
use mysql::conn::Conn;
use mysql::Opts;
use std::result::Result;
use r2d2;

#[derive(Debug)]
pub struct MysqlConnectionManager {
    params: Opts,
}


impl MysqlConnectionManager {
    /// Creates a new `MysqlConnectionManager`.
    ///
    /// See `postgres::Connection::connect` for a description of the parameter
    /// types.
    pub fn new<T: Into<Opts>>(params: T) -> Result<MysqlConnectionManager, Error> {
        self.param = try!(Opts::from(T));
    }

    fn do_connect(&self) -> Result<Conn,Error> {
        Conn::new(self.param);
    }
}

impl r2d2::ManageConnection for MysqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Conn,Error> {
    	self.do_connect()
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Error> {
        conn.query("select 1").map(|_| () )
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}
