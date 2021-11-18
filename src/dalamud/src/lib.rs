use std::error::Error;
// NOTE (Chiv) Needed to use them all as the package does not
// expose them in the modern way afaiu.
// (It was made during the #[macro_use] time)
use boolean_enums::_gen_boolean_enum_common;
use boolean_enums::_gen_boolean_enum_gen_enum;
use boolean_enums::_gen_boolean_enum_serde;
use boolean_enums::gen_boolean_enum;
use serde::{Deserialize, Serialize};

mod assets;
mod launch;
mod launch_old;
mod update;

// TODO (Chiv) structs, types and func in seperate modules

gen_boolean_enum!(pub serde UseTestVersion);
gen_boolean_enum!(pub serde NeedsRuntime);
gen_boolean_enum!(pub serde OptOutMbCollection);

//TODO(Chiv) Clone/Copy/Eq -> Beta_Kind string prevents Copy
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Settings {
    #[serde(rename = "DoDalamudTest")]
    testing: UseTestVersion,
    #[serde(rename = "DoDalamudRuntime")]
    runtime: NeedsRuntime,
    #[serde(rename = "DalamudBetaKind")]
    beta_kind: String,
    //TODO(Chiv) Is the `bool?` (nullable) in C# XL legacy?
    #[serde(rename = "OptOutMbCollection")]
    opt_out_mb_collection: Option<OptOutMbCollection>,
}

// TODO(Chiv) AsRef Path?
//pub struct DalamudRoot<'a, T: AsRef<std::path::Path>>(&'a T);
// TODO (Chiv) This seems to be more XIVLauncher root, runtime and ressources are not in 'addon'
pub struct RootPath<'a>(pub &'a std::path::Path);
pub struct GamePath<'a>(pub &'a std::path::Path);

// TODO Types, especially error
pub async fn download_and_start(
    root: RootPath<'_>,
    game: GamePath<'_>,
    language: libxl::game::language::ClientLanguage,
    settings: Settings,
    game_process_id: usize,
) -> Result<(), Box<dyn Error>> {
    let root = root.0;
    let game = game.0;
    // TODO paths should be constructed here?
    let (supported_version, injector) = update::update(root, &settings).await?;
    // TODO (Chiv) overlay to asset download
    let assets = root.join("dalamudAssets");
    assets::ensure(&assets).await?;
    tracing::trace!("[DUPDATE] All files and assets set, launching dalamud.");
    let dalamud = launch::launch(
        &root,
        game,
        &injector,
        &assets,
        supported_version,
        language,
        settings.opt_out_mb_collection.unwrap_or_default(),
        game_process_id,
    )
    .await?;
    // TODO (Chiv) expose Dalamud process child?
    Ok(())
}
