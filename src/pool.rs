use super::param::ConnectParams;
use super::param::IntoConnectParams;
use mysql::error::MyError;
use mysql::conn::MyConn;
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
            -> Result<MysqlConnectionManager, MyError> {
        Ok(MysqlConnectionManager {
            params: try!(params.into_connect_params().map_err(|_| MyError::MyIoError(io::Error::new(io::ErrorKind::ConnectionRefused, "connect error"))))
        })
    }

    fn do_connect(&self) -> Result<MyConn,MyError> {
    	self.params.connect().map_err(|_| MyError::MyIoError(io::Error::new(io::ErrorKind::ConnectionRefused, "connect error")))
    }
}

impl r2d2::ManageConnection for MysqlConnectionManager {
    type Connection = MyConn;
    type Error = MyError;

    fn connect(&self) -> Result<MyConn,MyError> {
    	self.do_connect()
    }

    fn is_valid(&self, conn: &mut MyConn) -> Result<(), MyError> {
        conn.query("select 1").map(|_| () )
    }

    fn has_broken(&self, conn: &mut MyConn) -> bool {
        self.is_valid(conn).is_err()
    }
}
