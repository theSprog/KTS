#![allow(warnings, unused)]

use visulize::Visualizable;

extern crate kts;

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let ssssss = String::from("ELEabc");
        match ssssss.as_bytes() {
            [b'E', b'L', b'F', res @ ..] => {
                dbg!(res);
            }
            _ => panic!("not ELF"),
        }
    }
}
