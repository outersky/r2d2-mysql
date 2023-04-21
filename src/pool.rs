//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use mysql::{error::Error, prelude::*, Conn, Opts, OptsBuilder};
use std::{fmt, sync::Arc};

#[doc(hidden)]
#[deprecated(since = "23.0.0", note = "Renamed to `MySqlConnectionManager`.")]
pub type MysqlConnectionManager = MySqlConnectionManager;

/// A type for custom healthcheck function.
type HealthcheckFn =
    dyn Fn(MySqlConnectionManager, &mut Conn) -> Result<(), Error> + Send + Sync + 'static;

/// A wrapper for ease of applying custom healthcheck function.
#[derive(Clone)]
struct HealthcheckFnWrapper {
    pub function: Arc<HealthcheckFn>,
}

/// An [`r2d2`] connection manager for [`mysql`] connections.
#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    params: Opts,
    healthcheck_fn: Option<HealthcheckFnWrapper>,
}

impl MySqlConnectionManager {
    /// Constructs a new MySQL connection manager from `params`.
    pub fn new(params: OptsBuilder) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
            healthcheck_fn: None,
        }
    }

    /// Constructs a new MySQL connection manager from `params` with custom healthcheck function.
    pub fn with_custom_healthcheck(
        params: OptsBuilder,
        healthcheck_fn: &'static HealthcheckFn,
    ) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
            healthcheck_fn: Some(HealthcheckFnWrapper::new(healthcheck_fn)),
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
        if let Some(healthcheck_fn) = self.healthcheck_fn.clone() {
            return (healthcheck_fn.function)(self.clone(), conn);
        }
        conn.query("SELECT version()").map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}

impl HealthcheckFnWrapper {
    /// Constructs a new healthcheck function wrapper from `function`
    pub fn new(function: &'static HealthcheckFn) -> Self {
        Self {
            function: Arc::new(function),
        }
    }
}

impl fmt::Debug for HealthcheckFnWrapper {
    /// Format the struct so its parent can use #[derive(Debug)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
