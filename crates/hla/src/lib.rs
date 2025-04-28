#![feature(iter_intersperse)]

pub mod backend;
pub mod builder;
pub mod codegen;
pub mod frontend;
pub mod instructions;
pub mod ir;
pub mod liveness;
pub mod reification;

pub use builder::build;
pub use frontend::*;
pub use instructions::*;
pub use ir::Variable;
