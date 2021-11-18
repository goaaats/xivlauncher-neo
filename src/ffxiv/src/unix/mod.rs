#[cfg(not(windows))]
pub(crate) async fn launch_with_fix(
    working_directory: &std::path::Path,
    executable: &std::path::Path,
    arguments: String,
    environmental_variables: String,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
