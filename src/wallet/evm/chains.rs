#[cfg(feature = "ssr")]
use cached::proc_macro::io_cached;

use crate::AppError;

#[cfg(feature = "ssr")]
use crate::cached_surrealdb::AsyncSurrealCache;

#[cfg(feature = "ssr")]
use crate::db_init;

#[cfg(feature = "ssr")]
use std::time::Duration;

use leptos::server;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(not(feature = "ssr"))]
use crate::RecordId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChainRPC {
    pub url: String,
    pub tracking: Option<String>,
    #[serde(rename = "isOpenSource")]
    pub is_open_source: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Feature {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ENS {
    pub registry: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Explorer {
    pub name: String,
    pub url: String,
    pub standard: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bridge {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Parent {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub chain: String,
    pub bridges: Option<Vec<Bridge>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Partial)]
#[partial("ChainCreate", derive(Deserialize, Serialize, Debug, Clone), omit(id))]
pub struct Chain {
    pub id: RecordId,
    pub name: String,
    pub chain: String,
    pub icon: Option<String>,
    pub rpc: Vec<ChainRPC>,
    pub features: Option<Vec<Feature>>,
    #[serde(default = "Vec::new")]
    pub faucets: Vec<String>,
    #[serde(rename = "nativeCurrency")]
    pub native_currency: NativeCurrency,
    #[serde(rename = "infoURL")]
    pub info_url: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "networkId")]
    pub network_id: Option<u64>,
    pub slip44: Option<u64>,
    pub ens: Option<ENS>,
    pub explorers: Option<Vec<Explorer>>,
    pub tvl: Option<f64>,
    #[serde(rename = "chainSlug")]
    pub chain_slug: Option<String>,
    pub status: Option<String>,
    pub parent: Option<Parent>,
}

#[cfg(feature = "ssr")]
impl crate::Storage<ChainCreate, Chain> for Chain {
    const TABLE_NAME: &'static str = "chain";
}

#[cfg(feature = "ssr")]
#[io_cached(
    map_error = r##"|e| format!("Cache error: {:?}", e)"##,
    ty = "AsyncSurrealCache<String, Vec<Chain>>",
    create = r##" {
        AsyncSurrealCache::new("cache_table", Duration::from_secs(60))
            .set_refresh(true)
            .build()
            .await
            .expect("Failed to build SurrealDB cache")
    } "##,
    convert = r#"{ "chainlist_get_rpcs".to_string() }"#
)]
async fn get_rpcs() -> Result<Vec<Chain>, AppError> {
    use crate::Storage;

    let from_db = Chain::get_many().await?;
    if !from_db.is_empty() {
        return Ok(from_db);
    }

    // fetch json from https://chainlist.org/rpcs.json
    // see rpcs.json for the structure

    let client = reqwest::Client::new();
    let search_response = client
        .get("https://chainlist.org/rpcs.json")
        .header("Accept", "application/json")
        .send()
        .await?;

    let search_response_body = search_response.text().await?;

    let result: Vec<ChainCreate> = serde_json::from_str(&search_response_body)?;

    let mut bulk: Vec<(String, ChainCreate)> = vec![];
    for ch in result {
        bulk.push((format!("chainid_{}", ch.chain_id.to_string()), ch));
    }

    let res = Chain::upsert_many(bulk.clone()).await?;

    Ok(res)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcInfo {
    pub chain_id: u64,
    pub name: String,
    pub network: String,
    pub native_currency: String,
    pub rpc_url: String,
    pub explorer_url: Option<String>,
    pub icon: Option<String>,
}

impl Chain {
    #[cfg(feature = "ssr")]
    pub async fn get_by_chain_id(chain_id: u64) -> Result<Self, AppError> {
        let chains = get_rpcs().await?;

        let chain = chains.into_iter().find(|chain| chain.chain_id == chain_id);

        match chain {
            Some(chain) => Ok(chain),
            None => Err(AppError::NotFound(format!(
                "Chain with ID {} not found",
                chain_id
            ))),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn from_str(name: &str) -> Result<Self, AppError> {
        get_rpcs().await?;

        let db = db_init().await?;

        let name = name.to_lowercase().to_string();

        let mut dbq = db
            .query("select * from chain where chainSlug = $name;")
            .bind(("name", name.clone()))
            .await?;

        let chain = dbq.take(0)?;

        match chain {
            Some(chain) => Ok(chain),
            None => Err(AppError::NotFound(format!(
                "Chain with name {} not found",
                name.to_string()
            ))),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn get_chain_by_network(network: &str) -> Result<Self, AppError> {
        let chains = get_rpcs().await?;

        // Try to find by short name or chain slug
        let network_lower = network.to_lowercase();
        let chain = chains.into_iter().find(|chain| {
            chain.short_name.to_lowercase() == network_lower
                || chain
                    .chain_slug
                    .as_ref()
                    .map_or(false, |slug| slug.to_lowercase() == network_lower)
                || chain.chain.to_lowercase() == network_lower
        });

        match chain {
            Some(chain) => Ok(chain),
            None => Err(AppError::NotFound(format!(
                "Chain with network {} not found",
                network
            ))),
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn search_rpcs(query: String) -> Result<Vec<RpcInfo>, AppError> {
    let chains = get_rpcs().await?;

    let query_lower = query.to_lowercase();

    let mut results: Vec<RpcInfo> = chains
        .into_iter()
        .filter(|chain| {
            // Filter by name, chain name, or chain ID
            chain.name.to_lowercase().contains(&query_lower)
                || chain.chain.to_lowercase().contains(&query_lower)
                || chain.chain_id.to_string().contains(&query_lower)
                || chain.short_name.to_lowercase().contains(&query_lower)
        })
        .filter_map(|chain| {
            // Only include chains with at least one RPC URL
            if chain.rpc.is_empty() {
                return None;
            }

            // Find the first open source RPC URL, or just the first one
            let rpc_url = chain
                .rpc
                .iter()
                .find(|rpc| rpc.is_open_source.unwrap_or(false))
                .or_else(|| chain.rpc.first())
                .map(|rpc| rpc.url.clone())?;

            // Get the explorer URL if available
            let explorer_url = chain
                .explorers
                .as_ref()
                .and_then(|explorers| explorers.first())
                .map(|explorer| explorer.url.clone());

            Some(RpcInfo {
                chain_id: chain.chain_id,
                name: chain.name,
                network: chain.chain,
                native_currency: format!(
                    "{} ({})",
                    chain.native_currency.name, chain.native_currency.symbol
                ),
                rpc_url,
                explorer_url,
                icon: chain.icon,
            })
        })
        .collect();

    // Sort by chain ID for consistency
    results.sort_by_key(|rpc| rpc.chain_id);

    // Limit results to prevent overwhelming the UI
    results.truncate(20);

    Ok(results)
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_get_rpcs() {
    let rpcs = get_rpcs().await;
    assert!(rpcs.is_ok());
    let rpcs = rpcs.unwrap();
    assert!(!rpcs.is_empty());
    for rpc in rpcs.clone() {
        assert!(!rpc.name.is_empty());
    }
    println!("Fetched {} RPCs", rpcs.len());
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_search_rpcs() {
    let results = search_rpcs("ethereum".to_string()).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert!(!results.is_empty());

    // Test searching by chain ID
    let results = search_rpcs("1".to_string()).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert!(results.iter().any(|r| r.chain_id == 1));
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_search_chain_from_str() -> Result<(), AppError> {
    let list = vec![
        ("ethereum", 1),
        ("binance", 56),
        ("polygon", 137),
        ("arbitrum", 42161),
        ("optimism", 10),
        ("avalanche", 43114),
        ("base", 8453),
    ];

    for (chain_name, chain_id) in list {
        let ch = Chain::from_str(chain_name).await?;
        assert!(
            ch.chain_id == chain_id,
            "Chain ID mismatch for {}",
            chain_name
        );
    }

    Ok(())
}

#[server]
pub async fn get_chain_from_str(name: String) -> Result<Chain, AppError> {
    let ch = Chain::from_str(&name).await?;
    Ok(ch)
}
