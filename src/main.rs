mod args;

fn main() {
    let path = env!("PATH");
    println!("The PATH is: {}", path);
    println!("Hello, world!");
}



// //// Used by `args.rs`.
// fn rppm_version() -> &'static str {
//     env!("RPPM_VERSION")
// }
