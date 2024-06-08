use apsak_cli_lib::apsak_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_apsak_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    apsak_cli(options, None).await?;
    Ok(())
}
