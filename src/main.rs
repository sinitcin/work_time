mod data_base;

fn main() {
    println!("Hello, world!");
    data_base::create("test.sqlite").unwrap();
}
