use std::env::current_dir;

fn main() {
    let a = "some.text".split('.').collect::<Vec<_>>();
    println!("{:#?}", a)
}
