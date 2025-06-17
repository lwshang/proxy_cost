use candid::Principal;
use pocket_ic::{call_candid, PocketIc};
use proxy::ProxyCallArgs;
use std::{path::PathBuf, process::Command};

#[test]
fn basic() {
    let pic = pocket_ic::PocketIc::new();

    let proxy_wasm = build_canister("proxy");
    let proxy_canister_id = pic.create_canister();
    pic.add_cycles(proxy_canister_id, 100_000_000_000_000);
    pic.install_canister(proxy_canister_id, proxy_wasm, vec![], None);
    let dest_wasm = build_canister("dest");
    let dest_canister_id = pic.create_canister();
    pic.add_cycles(dest_canister_id, 100_000_000_000_000);
    pic.install_canister(dest_canister_id, dest_wasm, vec![], None);

    println!("1. Specific length inputs, fixed length replies:");
    for i in 0..10 {
        let len = 100 * i;
        let cycles = make_proxy_request(
            &pic,
            proxy_canister_id,
            dest_canister_id,
            "var_arg", // the method name takes 7 bytes
            vec![0; len],
        );
        // The byte length of the input must be accurate here, because we will use the intercept as the base fee.
        println!("{},  {}", len + 7, cycles);
    }

    println!("2. Fixed length inputs, specific length replies:");
    for i in 0..10 {
        let len: u16 = 100 * i;
        let args = len.to_be_bytes().to_vec();
        let cycles =
            make_proxy_request(&pic, proxy_canister_id, dest_canister_id, "reply_vec", args);
        println!("{}, {}", len, cycles);
    }

    println!("3. Fixed length inputs, specific length rejects:");
    for i in 0..10 {
        let len: u16 = 100 * i;
        let args = len.to_be_bytes().to_vec();
        let cycles = make_proxy_request(
            &pic,
            proxy_canister_id,
            dest_canister_id,
            "reject_vec",
            args,
        );
        println!("{}, {}", len, cycles);
    }
}

fn make_proxy_request(
    pic: &PocketIc,
    proxy_canister_id: Principal,
    dest_canister_id: Principal,
    method: &str,
    args: Vec<u8>,
) -> u128 {
    let args = ProxyCallArgs {
        canister_id: dest_canister_id,
        method: method.to_string(),
        args,
        attach_cycles: 0u32.into(),
    };
    let balance00 = pic.cycle_balance(proxy_canister_id);

    let _res: Result<(Vec<u8>,), _> = call_candid(
        pic,
        proxy_canister_id,
        pocket_ic::common::rest::RawEffectivePrincipal::None,
        "proxy_call",
        (args,),
    );
    let balance01 = pic.cycle_balance(proxy_canister_id);
    balance00 - balance01
}

fn build_canister(name: &str) -> Vec<u8> {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "build",
        "--target",
        "wasm32-unknown-unknown",
        "-p",
        name,
        "--profile",
        "canister-release",
    ]);
    cmd.output().expect("failed to compile the proxy canister");
    let wasm_path = PathBuf::from(format!(
        "../target/wasm32-unknown-unknown/canister-release/{name}.wasm"
    ));
    std::fs::read(&wasm_path)
        .unwrap_or_else(|_| panic!("failed to read the {name} canister wasm file"))
}
