use dotenv::dotenv;
use web3::{
    futures::{future, StreamExt},
    types::{FilterBuilder, Address},   
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    
    // Get params from .env
    let ws_url = dotenv::var("WS_URL").expect("WS_URL not set");
    log::info!("WebSocket URL: {}", ws_url);
    let contract_address = dotenv::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    let contract_address = Address::from_str(&contract_address).unwrap();
    log::info!("Contract address: {}", contract_address);

    // Setup web3 using web socket
    let transport = web3::transports::WebSocket::new(&ws_url).await?;
    let web3 = web3::Web3::new(transport);
    log::debug!("web3: {:?}", web3);
    
    // Setup filter
    let filter = FilterBuilder::default()
    .address(vec![contract_address])
    .build();

    let sub = web3.eth_subscribe().subscribe_logs(filter).await?;

    sub.for_each(|log| {
        println!("{:?}", log);
        future::ready(())
    }).await;

    Ok(())
}