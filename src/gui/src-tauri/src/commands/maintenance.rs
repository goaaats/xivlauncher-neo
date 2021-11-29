use crate::lib::error::XlResult;

/// Play the FF1 victory music
#[tauri::command]
pub async fn play_victory_beep() -> XlResult<()> {
  #[cfg(target_os = "windows")]
  async fn beep(hertz: u16, millis: u32) {
    winconsole::console::beep(hertz as u32, millis)
  }

  #[cfg(target_os = "linux")]
  async fn beep(hertz: u32, millis: u32) {
    beep::beep(hertz as u16);

    if millis > 0 {
      sleep(millis as u64).await;
      beep::beep(0);
    }
  }

  async fn sleep(millis: u64) {
    let duration = tokio::time::Duration::from_millis(millis);
    tokio::time::sleep(duration).await;
  }

  beep(523, 150).await;
  sleep(25).await;
  beep(523, 150).await;
  sleep(25).await;
  beep(523, 150).await;
  sleep(25).await;
  beep(523, 300).await;
  sleep(150).await;
  beep(415, 300).await;
  sleep(150).await;
  beep(466, 300).await;
  sleep(150).await;
  beep(523, 300).await;
  sleep(25).await;
  beep(466, 150).await;
  sleep(25).await;
  beep(523, 900).await;

  Ok(())
}
