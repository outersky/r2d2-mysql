//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use std::fmt;

use mysql::{error::Error, prelude::*, Conn, Opts, OptsBuilder};

/// A type for custom health check function.
type HealthCheckFn = fn(MySqlConnectionManager, &mut Conn) -> Result<(), Error>;

/// A wrapper for ease of applying custom health check function.
#[derive(Clone)]
struct HealthCheckFnWrapper {
    function: HealthCheckFn,
}

/// An [`r2d2`] connection manager for [`mysql`] connections.
#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    params: Opts,
    health_check_fn: Option<HealthCheckFnWrapper>,
}

impl MySqlConnectionManager {
    /// Constructs a new MySQL connection manager from `params`.
    pub fn new(params: OptsBuilder) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
            health_check_fn: None,
        }
    }

    /// Constructs a new MySQL connection manager from `params` with custom health check function.
    pub fn with_custom_health_check(
        params: OptsBuilder,
        health_check_fn: HealthCheckFn,
    ) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
            health_check_fn: Some(HealthCheckFnWrapper::new(health_check_fn)),
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
        if let Some(health_check_fn) = self.health_check_fn.clone() {
            return (health_check_fn.function)(self.clone(), conn);
        }
        conn.query("SELECT version()").map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}

impl HealthCheckFnWrapper {
    /// Constructs a new health check function wrapper from `function`.
    fn new(function: HealthCheckFn) -> Self {
        Self { function }
    }
}

impl fmt::Debug for HealthCheckFnWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HealthCheckFnWrapper")
            .finish_non_exhaustive()
    }
}
