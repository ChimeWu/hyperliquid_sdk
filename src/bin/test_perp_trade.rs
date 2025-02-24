use ethers::signers::LocalWallet;
use log::info;
use ethers::types::H160;

use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    ExchangeDataStatus, ExchangeResponseStatus, InfoClient, MarketOrderParams,
};
use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "4eaab9c7f0230b232abeb23701b927c7190e4b424aeb7a5bfe92b60546aa4aa1"
        .parse()
        .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Testnet), None, None)
        .await
        .unwrap();

    let info_client = InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap();
    let user:H160 = "0x7271b723F864d77Db16C20dDf0eC8b78Df05aeb2".to_string().parse().unwrap();

    let balances = info_client.user_token_balances(user).await.unwrap();
    info!("User token balances: {balances:?}");
    let open_orders = info_client.open_orders(user).await.unwrap();
    info!("Open orders: {open_orders:?}");
    let user_state = info_client.user_state(user).await.unwrap();
    info!("User state: {user_state:?}");

    let order = ClientOrderRequest {
        asset: "ETH".to_string(),
        is_buy: true,
        reduce_only: false,
        limit_px: 1570.0,
        sz: 0.07,
        cloid: None,
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
    };
    let response = exchange_client
        .order(order,None)
        .await
        .unwrap();
    info!("PERP Market open order placed: {response:?}");

    let balances = info_client.user_token_balances(user).await.unwrap();
    info!("User token balances: {balances:?}");
    let open_orders = info_client.open_orders(user).await.unwrap();
    info!("Open orders: {open_orders:?}");
    let response = match response {
        ExchangeResponseStatus::Ok(exchange_response) => exchange_response,
        ExchangeResponseStatus::Err(e) => panic!("error with exchange response: {e}"),
    };
    let status = response.data.unwrap().statuses[0].clone();
    let oid = match status {
        ExchangeDataStatus::Filled(order) => order.oid,
        ExchangeDataStatus::Resting(order) => order.oid,
        _ => panic!("Error: {status:?}"),
    };


    // So you can see the order before it's cancelled
    sleep(Duration::from_secs(10));

    let cancel = ClientCancelRequest {
        asset: "ETH".to_string(),
        oid,
    };

    // This response will return an error if order was filled (since you can't cancel a filled order), otherwise it will cancel the order
    let response = exchange_client.cancel(cancel, None).await.unwrap();
    info!("Order potentially cancelled: {response:?}");
}