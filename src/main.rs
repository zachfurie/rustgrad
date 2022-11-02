#[path = "bin/rust_grad_ops"] mod rust_grad_ops;

// use std::env;


fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    match rust_grad_ops::simple() {
        Ok(x) => {x}
        Err(e) => println!("ERROR:  {}", e)
    };
}