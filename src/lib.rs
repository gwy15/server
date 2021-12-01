#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate anyhow;

pub mod cli;
#[macro_use]
pub mod error;
mod config;
pub mod db;
pub mod handlers;
pub mod models;

pub use config::Config;

pub mod prelude {
    pub use crate::error::{Error, ErrorExt, Result};
    pub use crate::{db, models};
    pub use actix_web::Error as ActixError;
    pub use actix_web::{
        http::{self, StatusCode},
        web::{self, Data, Form, Json, Path, Query},
    };
    pub use anyhow::Context;
}
