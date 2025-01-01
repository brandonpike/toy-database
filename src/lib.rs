//! A very complete database!
//!
//! Upcoming features:
//! - Query planning
//! - Protocol definition and implementation
//! - Value types and checking

pub mod qp;
pub mod storage;
pub mod table;

use qp::Plan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    Io(#[from] std::io::Error),
    InvalidPlan(Plan),
    InvalidQuery(String),
    InvalidSchema(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(error) => f.write_str(&format!("Io error: {error}")),
            AppError::InvalidQuery(query) => f.write_str(&format!("Invalid query: {query}")),
            AppError::InvalidPlan(plan) => f.write_str(&format!("Invalid plan: {plan:?}")),
            AppError::InvalidSchema(_) => todo!(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
