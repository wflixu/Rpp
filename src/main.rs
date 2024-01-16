use clap::App;
fn main() {
    let _matches = App::new("echor")
        .version("0.0.1")
        .author("Luke")
        .about("Rust echo")
        .get_matches();
    println!("hello, world!!!")
}
