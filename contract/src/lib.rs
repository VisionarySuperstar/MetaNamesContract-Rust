#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;

mod actions;
pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

#[cfg(test)]
mod tests;
