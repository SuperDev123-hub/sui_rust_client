use serde::Deserialize;
use std::str::FromStr;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    json::SuiJsonValue,
    types::base_types::{ObjectID, SuiAddress},
    SuiClient, SuiClientBuilder,
};
use sui_types::{intent::Intent, messages::ExecuteTransactionRequestType, messages::Transaction};

#[derive(Debug, Deserialize)]
struct Forge {
    id: ObjectID,
    swords_created: u64,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui: SuiClient = SuiClientBuilder::default()
        .build("https://fullnode.devnet.sui.io:443")
        .await?;
    let my_address = SuiAddress::from_str("0x7656fc4c24a42e7b18ab51d0b205b67a6c0a3bfc")?;
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };
    println!("key path {:?}", keystore_path);
    let call_tx = sui
        .transaction_builder()
        .move_call(
            my_address,
            ObjectID::from_str("0x34bf43083f656be56f93d034c6992ad0f4f3133e").unwrap(),
            "my_module",
            "sword_create",
            vec![],
            vec![
                SuiJsonValue::from_str("0xb62bc6e9df02b1b0f3ab1fb8fbc08c210ea82c67")?,
                SuiJsonValue::from_str("\"1\"")?,
                SuiJsonValue::from_str("\"2\"")?,
                SuiJsonValue::from_str("0x7656fc4c24a42e7b18ab51d0b205b67a6c0a3bfc")?,
            ],
            None,
            1000,
        )
        .await?;

    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign_secure(&my_address, &call_tx, Intent::default())?;

    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(
            Transaction::from_data(call_tx, Intent::default(), signature).verify()?,
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
