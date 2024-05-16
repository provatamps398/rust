use std::str::FromStr;

use ic_cdk::{update, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_cdk_macros::query;
use ic_web3::{transports::ICHttp, Web3, types::{BlockId, BlockNumber, U64, Address}, contract::{Contract, Options}};

// Constants
const ERC20_ABI: &[u8] = include_bytes!("../../abi/erc20.json");

const BASE_URL: &'static str = "polygon-mainnet.g.alchemy.com";
const PATH: &'static str = "/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f";

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
async fn call_name(contract_addr: String) -> String {
    let contract = generate_contract_client(contract_addr, ERC20_ABI).unwrap();
    contract
        .query("name", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}

#[update]
async fn call_symbol(contract_addr: String) -> String {
    let contract = generate_contract_client(contract_addr, ERC20_ABI).unwrap();
    contract
        .query("symbol", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}

#[update]
async fn call_decimals(contract_addr: String) -> u8 {
    let contract = generate_contract_client(contract_addr, ERC20_ABI).unwrap();
    contract
        .query("decimals", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}

#[update]
async fn call_total_supply(contract_addr: String) -> u128 {
    let contract = generate_contract_client(contract_addr, ERC20_ABI).unwrap();
    contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}

#[update]
async fn call_balance_of(contract_addr: String, holder_addr: String) -> u128 {
    let contract = generate_contract_client(contract_addr, ERC20_ABI).unwrap();
    let addr = Address::from_str(&holder_addr).unwrap();
    contract
        .query("balanceOf", addr, None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e)).unwrap()
}
