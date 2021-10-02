// #![allow(unused)] // silence unused warnings while exploring (to comment out)

mod core;
mod delete;
mod insert;
mod select;
pub mod sqlx_exec;
mod update;
mod val;

pub use crate::core::Field;
pub use crate::core::HasFields;
pub use crate::core::SqlBuilder;
pub use crate::delete::delete;
pub use crate::delete::delete_all;
pub use crate::delete::SqlDeleteBuilder;
pub use crate::insert::insert;
pub use crate::insert::SqlInsertBuilder;
pub use crate::select::select;
pub use crate::select::SqlSelectBuilder;
pub use crate::update::update;
pub use crate::update::update_all;
pub use crate::update::SqlUpdateBuilder;
pub use crate::val::SqlxBindable;
pub use sqlb_macros::Fields;
