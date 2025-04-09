// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use alloy_primitives::{Address, Bytes, B256, U256};
use alloy_provider::{network::Ethereum, Provider};

fn is_minimal_proxy(code: &Bytes) -> bool {
    code.len() <= 45 && code[..].starts_with(&hex::decode("363d3d373d3d3d363d73").unwrap())
}

fn parse_minimal_impl(code: &Bytes) -> Option<Address> {
    if !is_minimal_proxy(code) {
        return None;
    }
    if code.len() < 30 {
        return None;
    }
    let impl_bytes = &code[10..30];
    Some(Address::from_slice(impl_bytes))
}

fn b256_to_address(word: B256) -> Address {
    Address::from_slice(&word.as_slice()[12..])
}

async fn first_non_zero_slot(
    slots: &[&str],
    proxy: Address,
    provider: &impl Provider<Ethereum>,
) -> anyhow::Result<Option<(B256, Address)>> {
    for slot in slots {
        let raw = provider
            .get_storage_at(proxy, U256::from_str(slot)?)
            .await?
            .into();
        let addr = b256_to_address(raw);
        if addr != Address::ZERO {
            return Ok(Some((raw, addr)));
        }
    }
    Ok(None)
}

