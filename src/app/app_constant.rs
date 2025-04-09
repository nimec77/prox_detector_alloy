// ─────────────────────────────────────────────────────────────────────────────
// Storage slot constants (bytes32 → B256)
// ─────────────────────────────────────────────────────────────────────────────

const EIP1967_IMPL_SLOT: &str =
    "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
const EIP1967_ADMIN_SLOT: &str =
    "0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103";
const EIP1967_BEACON_SLOT: &str =
    "0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";
const ZEP_IMPL_SLOT: &str =
    "0xe3caa0b6d0569366f92f8438abfdb232378cfd332e9dc8656f33322176d86c38";
const ZEP_ADMIN_SLOT: &str =
    "0xe59ed60ca6e4e7df6bd875f45f3199fa50d18f9f8eab0cbc73f528c3bf38189c";

static IMPLEMENTATION_SLOTS: [&str; 2] = [EIP1967_IMPL_SLOT, ZEP_IMPL_SLOT];
static ADMIN_SLOTS: [&str; 2] = [EIP1967_ADMIN_SLOT, ZEP_ADMIN_SLOT];

