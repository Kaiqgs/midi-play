#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

pub mod components;
pub mod controllers;
pub mod models;
// pub mod tests;
#[cfg(test)]
pub mod tests;
