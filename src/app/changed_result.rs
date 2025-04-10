use alloy_primitives::Address;

pub struct ChangeResult {
    pub changed: bool,
    pub current: Option<Address>,
}
