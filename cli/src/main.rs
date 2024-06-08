cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() {}
    } else {
        use apsak_cli_lib::{apsak_cli, TerminalOptions};

        #[tokio::main]
        async fn main() {
            let result = apsak_cli(TerminalOptions::new().with_prompt("$ "), None).await;
            if let Err(err) = result {
                println!("{err}");
            }
        }
    }
}
