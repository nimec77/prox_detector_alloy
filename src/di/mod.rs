use alloy_provider::{Provider, ProviderBuilder};

/// Build a JSON‑RPC provider from RPC_URL (or Infura fallback)
pub fn build_provider() -> anyhow::Result<impl Provider> {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| {
        let key = std::env::var("INFURA_API_KEY").unwrap_or_default();
        format!("https://mainnet.infura.io/v3/{key}")
    });

    // ProviderBuilder::<Http>::new() is inferred; the returned
    // value implements the `Provider` trait.
    Ok(ProviderBuilder::new().on_http(url.parse()?))
}
