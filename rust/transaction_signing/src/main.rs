use anyhow;
use transaction_signing::*;
use definitions::constants::COLD_DB_NAME;


fn main() -> anyhow::Result<()> {
    let mock_action_line = r#"{"type":"sign_transaction","checksum":"3665731191"}"#;
    let seed_phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    let pwd_entry = "jaskier";
    let result = handle_action(&mock_action_line, seed_phrase, pwd_entry, COLD_DB_NAME)?;
    println!("{}", result);
    Ok(())
}
