use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::call::Call;
use std::convert::TryInto;

#[derive(CandidType, Clone, Debug, Deserialize)]
struct ProxyCallArgs {
    canister_id: Principal,
    method: String,
    args: Vec<u8>,
    attach_cycles: Nat,
}

#[ic_cdk::update]
async fn proxy_call(args: ProxyCallArgs) -> Vec<u8> {
    Call::bounded_wait(args.canister_id, &args.method)
        .with_raw_args(&args.args)
        .with_cycles(
            args.attach_cycles
                .0
                .try_into()
                .expect("Failed to convert Nat to u128"),
        )
        .await
        .expect("Failed to call canister")
        .into_bytes()
}
