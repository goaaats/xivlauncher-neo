use std::{fmt, path::Path};

use libxl::game::oauth::AccountRegion;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let status = libxl::game::status::is_gate().await;

    match status {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Could not fetch:\n{}", err),
    }

    let login = libxl::game::oauth::login(
        "test",
        "test",
        "",
        libxl::game::platform::Platform::Win32,
        true,
        AccountRegion::Europe,
    )
    .await;

    let hash = libxl::game::version::get_patch_gamever_info(Path::new("E:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn"));
    match hash {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Could not fetch"),
    }
}
