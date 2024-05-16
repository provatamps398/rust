use std::str::FromStr;

use ic_cdk::{update, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_cdk_macros::query;
use ic_web3::{transports::ICHttp, Web3, types::{BlockId, BlockNumber, U64, Address}, contract::{Contract, Options}};

// Constants
const ABI: &[u8] = include_bytes!("../../abi/registry.json");

const CONTRACT_ADDRESS: &'static str = "0x90E00ACe148ca3b23Ac1bC8C240C2a7Dd9c2d7f5";
const BASE_URL: &'static str = "eth-mainnet.g.alchemy.com";
const PATH: &'static str = "/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy";

fn get_rpc_endpoint() -> String {
    format!("https://{}{}", BASE_URL, PATH)
}

async fn get_block(number: Option<u64>) -> Result<String, String> {
    let w3 = match ICHttp::new(get_rpc_endpoint().as_str(), None, None) {
        Ok(v) => Web3::new(v),
        Err(e) => return Err(e.to_string())
    };
    let block_id = match number {
        Some(id) => { BlockId::from(U64::from(id)) },
        None => { BlockId::Number(BlockNumber::Latest) },
    };
    let block = w3.eth().block(block_id).await.map_err(|e| format!("get block error: {}", e))?;

    Ok(serde_json::to_string(&block.unwrap()).unwrap())
}

fn generate_contract_client(contract_addr: String, abi: &[u8]) -> Result<Contract<ICHttp>, String> {
    let w3 = match ICHttp::new(get_rpc_endpoint().as_str(), None, None) {
        Ok(v) => Web3::new(v),
        Err(e) => return Err(e.to_string())
    };
    let contract_address = Address::from_str(&contract_addr).unwrap();
    Contract::from_json(
        w3.eth(),
        contract_address,
        abi
    ).map_err(|e| format!("init contract failed: {}", e))

}

#[query(name = "transform")]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

#[update]
async fn get_latest_block() -> String {
    match get_block(None).await {
        Ok(msg) => msg,
        Err(msg) => msg,
    }
}

#[update]
async fn call_pool_count() -> u128 {
    let contract = generate_contract_client(CONTRACT_ADDRESS.to_owned(), ABI).unwrap();
    contract
        .query("pool_count", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}

#[update]
async fn call_coin_count() -> u128 {
    let contract = generate_contract_client(CONTRACT_ADDRESS.to_owned(), ABI).unwrap();
    contract
        .query("coin_count", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}
