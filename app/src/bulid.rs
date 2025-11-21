use std::env;

fn main() {
    let _ = dotenv::dotenv();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

    println!("cargo:rustc-env=SERVER_URL={}", server_url);
}
