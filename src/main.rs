mod data_base;

fn main() {
    println!("Hello, world!");
    #[allow(unused_variables)]
    let code = try!(data_base::create("test.sqlite"));
}
