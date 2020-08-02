mod errors;
mod views;
use schemars::schema_for;

use crate::errors::JsonRpcError;
use crate::views::{
    AccountStateWithProofView, AccountView, BlockMetadata, CurrencyInfoView, EventView,
    StateProofView, TransactionView,
};
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub result: Option<ResponseViews>,
    pub error: Option<JsonRpcError>,

    // Libra additional fields
    pub libra_chain_id: u8,
    pub libra_ledger_version: u64,
    pub libra_ledger_timestampusec: u64,
}

#[derive(JsonSchema)]
pub enum ResponseViews {
    AccountView(AccountView),
    EventView(EventView),
    TransactionView(TransactionView),
    CurrencyInfoView(CurrencyInfoView),
    BlockMetadata(BlockMetadata),
    StateProofView(StateProofView),
    AccountStateWithProofView(AccountStateWithProofView),
}

fn main() {
    let schema = schema_for!(JsonRpcResponse);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
