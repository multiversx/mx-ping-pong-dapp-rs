use multiversx_my_sdk::{
    data::transaction::Transaction,
    gateway::{GatewayProxy, DEVNET_GATEWAY},
    wallet::Wallet,
};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_pem_file("./demoPem.pem").unwrap();
    let addr = wl.address();
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    let mut unsign_tx = Transaction {
        nonce: arg.nonce,
        value: "0".to_string(),
        receiver: addr.clone(),
        sender: addr.clone(),
        gas_price: arg.gas_price,
        gas_limit: arg.gas_limit,
        data: arg.data,
        signature: None,
        chain_id: arg.chain_id,
        version: arg.version,
        options: arg.options,
    };

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    let tx_hash = blockchain.send_transaction(&unsign_tx);
    // println!("tx_hash {tx_hash}");
}