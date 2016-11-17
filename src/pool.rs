use mysql::error::Error;
use mysql::conn::Conn;
use mysql::Opts;
use mysql::OptsBuilder;
use std::result::Result;
use r2d2;

#[derive(Debug)]
pub struct MysqlConnectionManager {
    params: Opts,
}

pub trait CreateManager<T> {
 type Manager;
 fn new(params: T) -> Result<Self::Manager, Error>;
}

impl CreateManager<OptsBuilder> for MysqlConnectionManager {
    type Manager = MysqlConnectionManager;
     fn new(params: OptsBuilder) -> Result<Self::Manager, Error> {
         Ok(
             MysqlConnectionManager {
                 params: Opts::from(params)
             }
         )
    }
}

impl <'a> CreateManager<&'a str> for MysqlConnectionManager {
     type Manager = MysqlConnectionManager;
     fn new(params: &'a str) -> Result<Self::Manager, Error> {
         Ok(
             MysqlConnectionManager {
                 params: Opts::from(params)
             }
         )
    }
}

impl r2d2::ManageConnection for MysqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Conn,Error> {
    	    Conn::new(self.params.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Error> {
        conn.query("SELECT version()").map(|_| () )
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}
