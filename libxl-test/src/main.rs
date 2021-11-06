use std::{fmt, path::Path};

use libxl::game::oauth::AccountRegion;

#[tokio::main]
async fn main() {
    /*
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
    */

    let hash = libxl::game::version::get_version_report(Path::new("E:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn"), 4);
    match hash {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Could not fetch: {:?}", err),
    }

    let bootpatch = libxl::game::version::check_boot_version(Path::new("E:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn"), libxl::game::platform::Platform::Win32).await;
    match bootpatch {
        Ok(res) => println!("{}", res.patches.len()),
        Err(err) => println!("Could not fetch: {:?}", err),
    }
}
