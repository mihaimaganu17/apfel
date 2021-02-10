use std::fs;

mod mach;
pub mod utils;


fn main() {
    let f = fs::read("/Users/m3m0ry/fun/apfel/samples/hello_world");
    let contents = match f {
        Ok(bytes) => bytes,
        Err(e) => Vec::new(),
    };

    let magic = utils::read_magic(contents.as_slice());
    if magic == mach::header::MH_MAGIC {
        println!("You found a Mach-O! Wee");
    }

    let mach_header_64 = mach::header::MachHeader64::parse(contents.as_slice());

    println!("{}", mach_header_64);

}
