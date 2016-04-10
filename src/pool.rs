use mysql::error::Error;
use mysql::conn::Conn;
use mysql::Opts;
use std::result::Result;
use r2d2;

#[derive(Debug)]
pub struct MysqlConnectionManager {
    params: String,
}


impl MysqlConnectionManager {
    /// Creates a new `MysqlConnectionManager`.
    ///
    /// See `postgres::Connection::connect` for a description of the parameter
    /// types.
    pub fn new(params: &str)
            -> Result<MysqlConnectionManager, Error> {
        Ok(MysqlConnectionManager {
            params: params.to_owned(),
        })
    }

    fn do_connect(&self) -> Result<Conn,Error> {
        let opts = Opts::from_url(self.params.as_str()).expect("mysql url error!");
        Conn::new(opts)
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
