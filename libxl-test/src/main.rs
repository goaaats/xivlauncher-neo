use std::fmt;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let status = libxl::game::status::is_gate().await;

    match status {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Could not fetch:\n{}", err)
    }

    let login = libxl::game::oauth::login("test", "test", "", true, libxl::game::oauth::AccountRegion::Europe).await;
}