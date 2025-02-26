use ethers::signers::LocalWallet;
use ethers::types::H160;
use log::info;

use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    InfoClient,
};
use std::{thread::sleep, time::Duration};
use uuid::Uuid;

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
    let user: H160 = "0x7271b723F864d77Db16C20dDf0eC8b78Df05aeb2"
        .to_string()
        .parse()
        .unwrap();

    let balances = info_client.user_token_balances(user).await.unwrap();
    info!("User token balances: {balances:?}");
    let open_orders = info_client.open_orders(user).await.unwrap();
    info!("Open orders: {open_orders:?}");
    let user_state = info_client.user_state(user).await.unwrap();
    info!("User state: {user_state:?}");

    let order1 = ClientOrderRequest {
        asset: "ETH".to_string(),
        is_buy: true,
        reduce_only: false,
        limit_px: 1570.0,
        sz: 0.01,
        cloid: Some(Uuid::new_v4()),
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Alo".to_string(),
        }),
    };
    info!("Order 1 uuid:{:?}", order1.cloid);
    let order2 = ClientOrderRequest {
        asset: "ETH".to_string(),
        is_buy: true,
        reduce_only: false,
        limit_px: 1580.0,
        sz: 0.01,
        cloid: Some(Uuid::new_v4()),
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Alo".to_string(),
        }),
    };
    info!("Order 2 uuid:{:?}", order2.cloid);
    let order3 = ClientOrderRequest {
        asset: "ETH".to_string(),
        is_buy: true,
        reduce_only: false,
        limit_px: 1590.0,
        sz: 0.01,
        cloid: Some(Uuid::new_v4()),
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Alo".to_string(),
        }),
    };
    info!("Order 3 uuid:{:?}", order3.cloid);
    let orders = vec![order1, order2, order3];
    let response = exchange_client.bulk_order(orders, None).await.unwrap();
    info!("PERP Market open order placed: {response:?}");

    let balances = info_client.user_token_balances(user).await.unwrap();
    info!("User token balances: {balances:?}");
    let open_orders = info_client.open_orders(user).await.unwrap();
    let open_ids = open_orders.iter().map(|x| x.oid).collect::<Vec<u64>>();
    info!("Open orders: {open_orders:?}");
    let order0 = info_client
        .query_order_by_oid(user, open_orders[0].oid)
        .await
        .unwrap();
    info!("Order 0: {order0:?}");
    let order1 = info_client
        .query_order_by_oid(user, open_orders[1].oid)
        .await
        .unwrap();
    info!("Order 1: {order1:?}");
    let order2 = info_client
        .query_order_by_oid(user, open_orders[2].oid)
        .await
        .unwrap();
    info!("Order 2: {order2:?}");

    // So you can see the order before it's cancelled
    sleep(Duration::from_secs(10));

    let cancels = open_ids
        .iter()
        .map(|x| ClientCancelRequest {
            asset: "ETH".to_string(),
            oid: *x,
        })
        .collect::<Vec<ClientCancelRequest>>();
    let response = exchange_client.bulk_cancel(cancels, None).await.unwrap();
    info!("PERP Market open order cancelled: {response:?}");

    // This response will return an error if order was filled (since you can't cancel a filled order), otherwise it will cancel the order
    //let response = exchange_client.cancel(cancel, None).await.unwrap();
    //info!("Order potentially cancelled: {response:?}");
    //let order0 = info_client.query_order_by_oid(user,24904552936).await.unwrap();
    //info!("Order 0: {order0:?}");
}
