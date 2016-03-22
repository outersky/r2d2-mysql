use super::param::ConnectParams;
use super::param::IntoConnectParams;
use mysql::error::Error;
use mysql::conn::Conn;
use std::result::Result;
use std::io;
use r2d2;

#[derive(Debug)]
pub struct MysqlConnectionManager {
    params: ConnectParams,
}


impl MysqlConnectionManager {
    /// Creates a new `MysqlConnectionManager`.
    ///
    /// See `postgres::Connection::connect` for a description of the parameter
    /// types.
    pub fn new<T: IntoConnectParams>(params: T)
            -> Result<MysqlConnectionManager, Error> {
        Ok(MysqlConnectionManager {
            params: try!(params.into_connect_params().map_err(|_| Error::IoError(io::Error::new(io::ErrorKind::ConnectionRefused, "connect error"))))
        })
    }

    fn do_connect(&self) -> Result<Conn,Error> {
    	self.params.connect().map_err(|_| Error::IoError(io::Error::new(io::ErrorKind::ConnectionRefused, "connect error")))
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
