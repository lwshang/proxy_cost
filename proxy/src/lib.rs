use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ProxyCallArgs {
    pub canister_id: Principal,
    pub method: String,
    pub args: Vec<u8>,
    pub attach_cycles: Nat,
}
