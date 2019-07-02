#![feature(crate_visibility_modifier)]
#![feature(in_band_lifetimes)]
#![feature(async_await)]
#![feature(try_trait)]
#![feature(bind_by_move_pattern_guards)]
#![feature(box_syntax)]

mod cli;
mod commands;
mod context;
mod env;
mod errors;
mod evaluate;
mod format;
mod git;
mod object;
mod parser;
mod plugin;
mod prelude;
mod shell;
mod stream;

pub use crate::commands::command::ReturnValue;
pub use crate::parser::parse::span::SpannedItem;
pub use crate::parser::Spanned;
pub use crate::plugin::{serve_plugin, Plugin};
pub use cli::cli;
pub use errors::ShellError;
pub use object::base::{Primitive, Value};
pub use parser::parse::text::Text;
pub use parser::registry::{Args, CommandConfig};
