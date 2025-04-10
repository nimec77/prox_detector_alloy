// ─────────────────────────────────────────────────────────────────────────────
// Proxy enum
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyType {
    None,
    Minimal,
    Transparent,
    Uups,
    Beacon,
    Diamond,
    UnknownDelegate,
}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProxyType::None => "Not a proxy",
            ProxyType::Minimal => "EIP-1167 Minimal Proxy",
            ProxyType::Transparent => "Transparent/AdminUpgradeabilityProxy",
            ProxyType::Uups => "UUPS Proxy",
            ProxyType::Beacon => "Beacon Proxy",
            ProxyType::Diamond => "EIP-2535 Diamond Proxy",
            ProxyType::UnknownDelegate => "Custom DelegateCall Proxy",
        };
        write!(f, "{}", s)
    }
}
