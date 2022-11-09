#![allow(unused)]

#[macro_use]
extern crate visulize;

// 声明导出的 module
pub mod compiler;
pub mod error;

// 声明本 crate 的 module tree
mod ast;
mod lexer;
mod parser;
mod utils;
