use std::str::FromStr;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery};
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;

#[tokio::main]
async fn main() {
    let sui = SuiClientBuilder::default()
        .build("https://rpc.devnet.sui.io:443")
        .await.unwrap();
    let address = SuiAddress::from_str("0x7023b75914253c64ac8b1d9023419e590033d0770a5c4762a982d4eca54118c4").unwrap();
    let objects = sui
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await.unwrap();
    println!("{:?}", objects);
}
