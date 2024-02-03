use cargo_px_env::generated_pkg_manifest_path;
use wq_server::blueprint;
use pavex_cli_client::Client;
use std::error::Error;

/// Generate the `wq_server_server_sdk` crate using Pavex's CLI.
///
/// Pavex will automatically wire all our routes, constructors and error handlers
/// into the a "server SDK" that can be used by the final API server binary to launch
/// the application.
fn main() -> Result<(), Box<dyn Error>> {
    let generated_dir = generated_pkg_manifest_path()?.parent().unwrap().into();
    if let Err(e) = Client::new()
        .generate(blueprint(), generated_dir)
        .execute() {
        eprintln!("{e}");
        std::process::exit(1);
    }
    Ok(())
}
