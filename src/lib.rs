// Expose modules to entire crate (so we can use public modules in crate/tests (integration tests))
#![allow(non_snake_case)]
pub mod app;
pub mod board;
pub mod castling_rights;
pub mod chess_widget;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod displacement;
pub mod game;
pub mod moves;
pub mod pieces;
pub mod player;
pub mod timer;
#[cfg(any(feature = "web", feature = "server"))]
pub mod web;
