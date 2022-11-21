// #![allow(unused)]
#![allow(dead_code, unused_variables, unused_imports)]

#[warn(unused_must_use)]
#[macro_use]
extern crate visulize;

// 声明导出的 module
pub mod compiler;
pub mod error;

// 声明本 crate 的 module tree
mod ast;
mod lexer;
mod parser;
mod symbol;
mod utils;
