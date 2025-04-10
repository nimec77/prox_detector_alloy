// ─────────────────────────────────────────────────────────────────────────────
// Detection & helpers (generic over Provider)
// ─────────────────────────────────────────────────────────────────────────────
use std::str::FromStr;

use alloy_primitives::{Address, Bytes, U256};
use alloy_provider::Provider;
use serde_json::json;

use crate::app::{
    app_constant::{ADMIN_SLOTS, EIP1967_BEACON_SLOT, IMPLEMENTATION_SLOTS},
    changed_result::ChangeResult,
    proxy_enum::ProxyType,
};

use super::low_level_helpers::{
    b256_to_address, first_non_zero_slot, has_delegatecall, is_diamond, is_minimal_proxy,
    is_uups_implementation, parse_minimal_impl,
};

pub async fn detect_proxy<P: Provider>(addr: Address, provider: &P) -> anyhow::Result<ProxyType> {
    let code = provider.get_code_at(addr).await?;
    if code.is_empty() {
        anyhow::bail!("No contract code at address");
    }

    if is_minimal_proxy(&code) {
        return Ok(ProxyType::Minimal);
    }

    let impl_opt = first_non_zero_slot(&IMPLEMENTATION_SLOTS, addr, provider).await?;
    let admin_opt = first_non_zero_slot(&ADMIN_SLOTS, addr, provider).await?;
    let beacon_raw = provider
        .get_storage_at(addr, U256::from_str(EIP1967_BEACON_SLOT)?)
        .await?
        .into();
    let beacon = b256_to_address(beacon_raw);

    if beacon != Address::ZERO {
        return Ok(ProxyType::Beacon);
    }

    if let Some((_, impl_addr)) = impl_opt {
        if admin_opt.is_some() {
            return Ok(ProxyType::Transparent);
        }
        if is_uups_implementation(impl_addr, provider).await? {
            return Ok(ProxyType::Uups);
        }
    }

    if is_diamond(addr, provider).await? {
        return Ok(ProxyType::Diamond);
    }
    if has_delegatecall(&code) {
        return Ok(ProxyType::UnknownDelegate);
    }

    Ok(ProxyType::None)
}

pub async fn get_implementation_address<P: Provider>(
    proxy: Address,
    provider: &P,
) -> anyhow::Result<Option<Address>> {
    match detect_proxy(proxy, provider).await? {
        ProxyType::Minimal => {
            let code = provider.get_code_at(proxy).await?;
            Ok(parse_minimal_impl(&code))
        }
        ProxyType::Transparent | ProxyType::Uups => {
            let impl_opt = first_non_zero_slot(&IMPLEMENTATION_SLOTS, proxy, provider).await?;
            Ok(impl_opt.map(|(_, a)| a))
        }
        ProxyType::Beacon => {
            let beacon_raw = provider
                .get_storage_at(proxy, U256::from_str(EIP1967_BEACON_SLOT)?)
                .await?
                .into();
            let beacon = b256_to_address(beacon_raw);
            if beacon == Address::ZERO {
                return Ok(None);
            }
            let data = alloy_primitives::hex!("5c60da1b"); // implementation()
            let params =
                json!([ {"to": beacon, "data": format!("0x{}", hex::encode(data)) }, "latest" ]);
            let res = provider
                .raw_request::<_, Bytes>("eth_call".into(), params)
                .await?;
            if res.0.len() >= 32 {
                return Ok(Some(Address::from_slice(&res.0[12..32])));
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

pub async fn has_implementation_changed<P: Provider>(
    proxy: Address,
    provider: &P,
    last_impl: Address,
) -> anyhow::Result<ChangeResult> {
    let current = get_implementation_address(proxy, provider).await?;
    let changed = matches!(current, Some(curr) if curr != last_impl);
    Ok(ChangeResult { changed, current })
}
