use dalamud;
use sysinfo::{ProcessExt, SystemExt};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::test]
async fn dalamud_starts() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let dalamud_root = std::path::Path::new("./testing/dalamud_root");
    // let _ = std::fs::remove_dir_all(dalamud_root);
    // assert!(!dalamud_root.exists());
    std::fs::create_dir_all(dalamud_root).unwrap();
    let settings = dalamud::Settings::default();
    let mut s = sysinfo::System::new();
    s.refresh_processes();
    let ffxiv = s
        .processes()
        .into_iter()
        .find(|(pid, p)| p.name() == "ffxiv_dx11.exe")
        .map(|x| *x.0);
    //assert!(ffxiv.is_some());
    let ffxiv_pid = ffxiv.unwrap_or(0);
    // TODO (Chiv) Split tests and such and make this portable
    let game = std::path::Path::new(r"C:\Games\SquareEnix\FINAL FANTASY XIV - A Realm Reborn\");
    dalamud::download_and_start(
        dalamud::RootPath(dalamud_root),
        dalamud::GamePath(game),
        libxl::game::ClientLanguage::English,
        settings,
        ffxiv_pid,
    )
    .await
    .unwrap();
    //let _ = std::fs::remove_dir_all(dalamud_root);
}
