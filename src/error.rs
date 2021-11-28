use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use std::fmt;

pub struct Error {
    pub(crate) cause: anyhow::Error,
    pub(crate) code: Option<StatusCode>,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cause.fmt(f)
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.code)
            .field("cause", &self.cause)
            .finish()
    }
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(e: T) -> Self {
        Self {
            cause: e.into(),
            code: None,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.code {
            Some(code) => code,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        #[derive(serde::Serialize)]
        struct R {
            errmsg: String,
            #[cfg(debug_assertions)]
            details: String,
        }

        HttpResponseBuilder::new(self.status_code()).json(R {
            errmsg: self.cause.to_string(),
            #[cfg(debug_assertions)]
            details: format!("{:?}", self.cause),
        })
    }
}

/// add status code to `Result`
pub trait ErrorExt<T, E> {
    fn with_code(self, code: StatusCode) -> Result<T, Error>;
}
impl<T, E> ErrorExt<T, E> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_code(self, code: StatusCode) -> Result<T, Error> {
        self.map_err(|e| Error {
            cause: e.into(),
            code: Some(code),
        })
    }
}

/// make an error
#[macro_export]
macro_rules! web_error {
    ($msg:literal $(,)?) => {
        crate::error::Error {
            cause: anyhow::anyhow!($msg),
            code: None,
        }
    };
    ($code:expr, $msg:literal $(,)?) => {
        crate::error::Error {
            cause: anyhow::anyhow!($msg),
            code: Some($code)
        }
    };
    ($code:expr, $err:expr $(,)?) => {
        crate::error::Error {
            cause: anyhow::anyhow!($err),
            code: Some($code)
        }
    };
    ($code:expr, $fmt:expr, $($arg:tt)*) => {
        crate::error::Error {
            cause: anyhow::anyhow!($fmt, $($arg)*),
            code: Some($code)
        }
    };
}
