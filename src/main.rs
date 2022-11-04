use colored::Colorize;
use kts::{compiler::Compiler, error::err_exit};
use std::{env, process};

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("{}", String::from("Usage: kts <filename>").blue().bold());
        process::exit(1);
    }
    // 通过 cargo 命令行启动时当前路径是在 src 下
    // 通过测试启动时在 project 文件夹下
    let filename = args.nth(1).unwrap();
    let compiler = Compiler::new(&filename);
    let res = compiler.run();

    match res {
        Err(e) => err_exit(e),
        Ok(()) => println!("{}", "Compiliation succeeded !!!".green().bold()),
    }
}
