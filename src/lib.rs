// #![allow(unused)] // silence unused warnings while exploring (to comment out)

mod core;
mod delete;
mod insert;
mod select;
pub mod sqlx_exec;
mod update;
mod utils;
mod val;

pub use crate::core::Field;
pub use crate::core::HasFields;
pub use crate::core::SqlBuilder;
pub use crate::core::Whereable;
pub use crate::delete::delete;
pub use crate::delete::delete_all;
pub use crate::delete::DeleteSqlBuilder;
pub use crate::insert::insert;
pub use crate::insert::InsertSqlBuilder;
pub use crate::select::select;
pub use crate::select::SelectSqlBuilder;
pub use crate::update::update;
pub use crate::update::update_all;
pub use crate::update::UpdateSqlBuilder;
pub use crate::val::Raw;
pub use crate::val::SqlxBindable;
pub use sqlb_macros::Fields;
