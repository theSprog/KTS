#![allow(warnings, unused)]

use visulize::Visualizable;

extern crate kts;

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        println!("{}", 10u32.pow(9));
    }
}
