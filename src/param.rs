use std::result;
use std::error;
use std::io;

use super::url;
use super::url::Url;

use mysql::conn::MyConn;
use mysql::conn::MyOpts;
use mysql::error::MyResult;

/// Authentication information.
#[derive(Clone,Debug)]
pub struct UserInfo {
    /// The username
    pub user: String,
    /// An optional password
    pub password: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ConnectParams {
    /// The target server
    pub target: String,
    /// The target port.
    ///
    /// Defaults to 5432 if not specified.
    pub port: Option<u16>,
    /// The user to login as.
    ///
    /// `Connection::connect` requires a user but `cancel_query` does not.
    pub user: Option<UserInfo>,
    /// The database to connect to. Defaults the value of `user`.
    pub database: Option<String>,
    /// Runtime parameters to be passed to the Postgres backend.
    pub options: Vec<(String, String)>,
}


/// Reasons a new Postgres connection could fail.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ConnectError {
    /// The provided URL could not be parsed.
    InvalidUrl(String),
    /// The URL was missing a user.
    MissingUser,
    /// A password was required but not provided in the URL.
    MissingPassword,
    /// The Postgres server requested an authentication method not supported
    /// by the driver.
    UnsupportedAuthentication,
    /// The Postgres server does not support SSL encryption.
    NoSslSupport,
    /// There was an error initializing the SSL session.
    SslError(Box<error::Error+Sync+Send>),
    /// There was an error communicating with the server.
    IoError(io::Error),
}

/// Convienent method to connect to database
///
/// ```
/// let mut conn = connect("mysql://root:12345678@localhost:3306/db_name");
/// ```
pub fn connect<T>(into_params : T) -> MyResult<MyConn>
    where T:IntoConnectParams+Sized {
    let params = into_params.into_connect_params().unwrap();
    params.connect()
}

impl ConnectParams {
    pub fn connect(&self)-> MyResult<MyConn> {
        let user = self.user.clone();
        let opts = MyOpts {
            user: user.clone().map(|u| u.user),
            pass: user.clone().unwrap().password,
            db_name: self.database.clone(),
            ..Default::default()
        };
        MyConn::new(opts)
    }
}

/// A trait implemented by types that can be converted into a `ConnectParams`.
pub trait IntoConnectParams {
    /// Converts the value of `self` into a `ConnectParams`.
    fn into_connect_params(self) -> result::Result<ConnectParams, ConnectError>;
}

impl IntoConnectParams for ConnectParams {
    fn into_connect_params(self) -> result::Result<ConnectParams, ConnectError> {
        Ok(self)
    }
}

impl<'a> IntoConnectParams for &'a str {
    fn into_connect_params(self) -> result::Result<ConnectParams, ConnectError> {
        match Url::parse(self) {
            Ok(url) => url.into_connect_params(),
            Err(err) => return Err(ConnectError::InvalidUrl(err)),
        }
    }
}

impl IntoConnectParams for Url {
    fn into_connect_params(self) -> result::Result<ConnectParams, ConnectError> {
        let Url {
            host,
            port,
            user,
            path: url::Path { mut path, query: options, .. },
            ..
        } = self;

        let _maybe_path = try!(url::decode_component(&host).map_err(ConnectError::InvalidUrl));
        let target =  host;

        let user = user.map(|url::UserInfo { user, pass }| {
            UserInfo { user: user, password: pass }
        });

        let database = if path.is_empty() {
            None
        } else {
            // path contains the leading /
            path.remove(0);
            Some(path)
        };

        Ok(ConnectParams {
            target: target,
            port: port,
            user: user,
            database: database,
            options: options,
        })
    }
}