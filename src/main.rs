use std::fs;

fn main() {
    let f = fs::read("/Users/m3m0ry/fun/apfel/samples/hello_world");
    let contents = match f {
        Ok(bytes) => bytes,
        Err(e) => Vec::new(),
    };

    for byte in contents.iter() {
        print!("{:#02x} ",byte);
    }
}
