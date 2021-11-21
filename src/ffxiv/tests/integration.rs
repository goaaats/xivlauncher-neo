use ffxiv;
use sysinfo::{ProcessExt, SystemExt};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[test]
fn dev_ffxiv_starts() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let mut s = sysinfo::System::new();
    s.refresh_processes();
    let ffxiv = s
        .processes()
        .into_iter()
        .find(|(pid, p)| p.name() == "ffxiv_dx11.exe");
    assert!(ffxiv.is_none());
    // TODO (Chiv) Split tests and such and make this portable
    let game = std::path::Path::new(dbg!(env!("XL_TESTS_GAMEPATH")));
    //TODO  DEV ARGS Start Args DEV.DataPathType=1 DEV.MaxEntitledExpansionID=3 DEV.TestSID=a DEV.UseSqPack=1 SYS.Region=0 language=1
    ffxiv::launch(
        ffxiv::SessionId("a"),
        ffxiv::Region(0),
        ffxiv::ExpansionLevel(3),
        ffxiv::SteamIntegration::No,
        ffxiv::SteamServiceAccount::No,
        None,
        game,
        ffxiv::DX11::Yes,
        libxl::game::language::ClientLanguage::English,
        ffxiv::EncryptArguments::Yes,
        ffxiv::FfxivVersion(""),
    )
    .unwrap();
}
