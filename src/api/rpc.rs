use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::block::Block;
use crate::network::p2p::P2PNetwork;

/// Struct to represent an RPC response.
#[derive(Serialize, Deserialize)]
pub struct RpcResponse<T> {
    pub status: String,
    pub result: Option<T>,
}

/// Struct to represent an RPC error response.
#[derive(Serialize, Deserialize)]
pub struct RpcError {
    pub status: String,
    pub error: String,
}

/// Starts the RPC server.
pub async fn start_rpc_server(_network: P2PNetwork) {
    // Route to get the latest block.
    let get_block = warp::path!("block" / "latest")
        .map(move || {
            let latest_block = get_latest_block();
            match latest_block {
                Some(block) => warp::reply::json(&RpcResponse {
                    status: "success".to_string(),
                    result: Some(block),
                }),
                None => warp::reply::json(&RpcError {
                    status: "error".to_string(),
                    error: "No block found".to_string(),
                }),
            }
        });

    // Route to submit a transaction.
    let submit_tx = warp::path!("transaction" / "submit")
        .and(warp::body::json())
        .map(move |tx: Transaction| {
            let result = submit_transaction(tx);
            match result {
                true => warp::reply::json(&RpcResponse {
                    status: "success".to_string(),
                    result: Some("Transaction submitted".to_string()),
                }),
                false => warp::reply::json(&RpcError {
                    status: "error".to_string(),
                    error: "Transaction failed".to_string(),
                }),
            }
        });

    // Combine the routes.
    let routes = get_block.or(submit_tx);

    // Start the server on port 3030.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// Example function to get the latest block.
fn get_latest_block() -> Option<Block> {
    // Replace this with real logic to fetch the latest block.
    Some(Block::new(1, "prev_hash".to_string(), vec![], 0))
}

/// Validates a transaction before acceptance. Mempool / P2P broadcast is not wired yet.
fn submit_transaction(tx: Transaction) -> bool {
    if !tx.validate() {
        log::warn!("Rejected invalid transaction {}", tx.id);
        return false;
    }
    log::info!("Accepted transaction {} for later broadcast", tx.id);
    true
}
