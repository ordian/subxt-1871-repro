use subxt::{OnlineClient, PolkadotConfig, dynamic::{tx, Value}};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "./artifacts/kusama.scale")]
pub mod kusama {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api =
        OnlineClient::<PolkadotConfig>::from_url("wss://kusama-rpc.polkadot.io").await?;

    let signer = dev::alice();

    let tx_payload = tx("System", "remark", vec![Value::from_bytes("getrandom_or_die")]);

    let _signed = api
        .tx()
        .create_signed_offline(&tx_payload, &signer, Default::default())?;
    Ok(())
}
