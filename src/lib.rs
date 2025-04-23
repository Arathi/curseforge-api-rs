pub mod curseforge;
pub mod error;

pub use curseforge::client::{Client, ClientBuilder};
pub use curseforge::schemas::{Category, File, Game, Mod};
