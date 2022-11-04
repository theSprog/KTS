// extern crate colored;
// extern crate lazy_static;
// extern crate regex;

#[macro_use]
extern crate visulize;

// 声明导出的 module
pub mod compiler;
pub mod error;

// 声明本 crate 的 module tree
pub mod ast;
mod lexer;
pub mod parser;
mod utils;
