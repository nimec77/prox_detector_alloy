use std::str::FromStr;

use alloy_primitives::{Address, B256, Bytes, U256};

use alloy_provider::Provider;
use serde_json::json;

use crate::app::app_constant::EIP1967_IMPL_SLOT;

pub fn is_minimal_proxy(code: &Bytes) -> bool {
    code.len() <= 45 && code[..].starts_with(&alloy_primitives::hex::decode("363d3d373d3d3d363d73").unwrap())
}

pub fn parse_minimal_impl(code: &Bytes) -> Option<Address> {
    if !is_minimal_proxy(code) || code.len() < 30 {
        return None;
    }
    Some(Address::from_slice(&code[10..30]))
}

pub fn b256_to_address(word: B256) -> Address {
    Address::from_slice(&word.as_slice()[12..])
}

pub async fn first_non_zero_slot<P: Provider>(
    slots: &[&str],
    proxy: Address,
    provider: &P,
) -> anyhow::Result<Option<(B256, Address)>> {
    for slot in slots {
        let slot_u256 = U256::from_str(slot)?;
        let raw = provider.get_storage_at(proxy, slot_u256).await?.into();
        let addr = b256_to_address(raw);
        if addr != Address::ZERO {
            return Ok(Some((raw, addr)));
        }
    }
    Ok(None)
}

pub async fn is_uups_implementation<P: Provider>(
    impl_addr: Address,
    provider: &P,
) -> anyhow::Result<bool> {
    let params = json!([ {"to": impl_addr, "data": "0x52d1902d" }, "latest" ]); 
    let res = provider
        .raw_request::<_, Bytes>("eth_call".into(), params)
        .await?;
    if res.len() < 32 {
        return Ok(false);
    }
    Ok(res[..32] == alloy_primitives::hex::decode(&EIP1967_IMPL_SLOT[2..]).unwrap()[..32])
}

pub async fn is_diamond<P: Provider>(addr: Address, provider: &P) -> anyhow::Result<bool> {
    let params = json!([ {"to": addr, "data": "0x7a0ed627" }, "latest" ]); // facetAddresses()
    let res = provider
        .raw_request::<_, Bytes>("eth_call".into(), params)
        .await?;
    Ok(!res.is_empty())
}

pub fn has_delegatecall(code: &Bytes) -> bool { code.iter().take(300).any(|b| *b == 0xf4) }
