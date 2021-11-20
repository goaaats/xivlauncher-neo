// NOTE (Chiv) Needed to use them all as the package does not
// expose them in the modern way afaiu.
// (It was made during the #[macro_use] time)
use boolean_enums::_gen_boolean_enum_common;
use boolean_enums::_gen_boolean_enum_gen_enum;
use boolean_enums::_gen_boolean_enum_serde;
use boolean_enums::gen_boolean_enum;

mod blowfish;
mod arguments;
#[cfg(not(windows))]
pub mod unix;
#[cfg(windows)]
pub mod win;

#[cfg(not(windows))]
pub use self::unix as native;
#[cfg(windows)]
pub use self::win as native;

gen_boolean_enum!(pub serde SteamIntegration);
gen_boolean_enum!(pub serde SteamServiceAccount);
gen_boolean_enum!(pub serde DX11);
gen_boolean_enum!(pub serde EncryptArguments);
pub struct Region(pub u32);
pub struct ExpansionLevel(pub u32);
pub struct FfxivVersion<'a>(pub &'a str);
pub struct SessionId<'a>(pub &'a str);

// TODO (Chiv) Libxl again for ClientLanguage
// TODO (Chiv) Types, Error type
// TODO (Chiv) struct for args
/// # Errors
/// TODO
/// # Panics
/// TODO
pub fn launch(
    session_id: SessionId,
    region: Region,
    expansion_level: ExpansionLevel,
    steam_integration: SteamIntegration,
    steam_account: SteamServiceAccount,
    additional_arguments: Option<&str>,
    game: &std::path::Path,
    dx11: DX11,
    language: libxl::game::language::ClientLanguage,
    encrypt: EncryptArguments,
    version: FfxivVersion<'_>,
) -> Result<native::Process, Box<dyn std::error::Error>> {
    tracing::info!(
        "Launch FFXIV with (steamIntegration:{:?}, steamServiceAccount:{:?}, args:{})",
        steam_integration,
        steam_account,
        additional_arguments.unwrap_or_default()
    );

    //TODO (Chiv) Maybe steam should be init before launch call + own crate
    initialise_steam(steam_integration)?;

    // TODO(Chiv) Arg builder struct (and maybe environment too)
    //  TODO (Chiv) Take expansion level etc directly as string if used that way?
    let arguments = {
        let mut a = arguments::Builder::default()
            .append("DEV.DataPathType", "1".into())
            .append(
                "DEV.MaxEntitledExpansionID",
                expansion_level.0.to_string().into(),
            )
            .append("DEV.TestSID", session_id.0.into())
            .append("DEV.UseSqPack", "1".into())
            .append("SYS.Region", region.0.to_string().into())
            //TODO ClientLanguage to Number again, same as Dalamud, needs to expand type
            .append("language", "1".into())
            .append("ver", version.0.into());
        if let Some(aa) = additional_arguments {
            //  TODO (Chiv) parse and add additional args, C#XL mentions string args are a hack, maybe
            //   do now a vec tuple or such?
        }
        a
    };

    // TODO (Chiv) C#XL check for the exe existence here and shows a message box on error (and aborts)
    //  That should be the responsibility of the frontend, we only check for existence and return error message
    //  via canonicalize (GET duce package), which needs
    // TODO be wrapped in a custom error
    let executable = game
        .join("game")
        .join(if dx11.into() {
            "ffxiv_dx11.exe"
        } else {
            "ffxiv.exe"
        })
        .canonicalize()?;
    // We know that parent exists
    let working_directory = executable.parent().unwrap();

    let arguments = if encrypt.into() {
        unsafe {
            arguments.build_encrypted()
        }
    } else {
        arguments.build()
    };
    // TODO Environmental args
    let ffxiv = native::launch_with_fix(working_directory, &executable, arguments, "".into())?;
    // TODO Error on error + msg box

    //TODO (Chiv) Maybe steam should be un-init after launch call + own crate
    uninitialise_steam(steam_integration)?;

    //  TODO Loop until child.MainWindowHandle is not zero aka window is created
    // TODO (Chiv) feels like this loop should be higher up the chain maybe?
    tracing::debug!("Starting main window loop");
    for i in 0..30 {
        tracing::debug!("main window loop {}", i);
        if ffxiv.has_exited() {
            // TODO Launcher.cs:L244
        }
        if ffxiv.main_window()?.0 == 0 {
            // TODO Higher level API
            // TODO async sleep/delay only with a runtime -> another reason why this thing might be needed
            //  higher up the chain
            std::thread::sleep(std::time::Duration::from_millis(1000));
            // TODO Again for good measure, the above is bad in an async world
            continue;
        }
        break;
    }
    // TODO error on exit + check for multibox
    Ok(ffxiv)
}

// TODO (Chiv) The function and I suppose it shall be in its own crate
fn initialise_steam(integration: SteamIntegration) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// TODO (Chiv) The function and I suppose it shall be in its own crate
fn uninitialise_steam(integration: SteamIntegration) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn dummy() {
        eprintln!("{:?}", SteamIntegration::Yes);
    }

    #[test]
    fn start_game() {
        let game_path = dbg!(env!("XL_TESTS_GAMEPATH"));

        launch(SessionId("1"), Region(1), ExpansionLevel(2), SteamIntegration::Yes, SteamServiceAccount::Yes, None,
         &PathBuf::from(game_path), DX11::Yes, libxl::game::language::ClientLanguage::Japanese, EncryptArguments::Yes, FfxivVersion("0.0.0.0")).unwrap();
    }
}
