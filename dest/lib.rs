use ic_cdk::update;

#[update]
fn empty() {}

#[update]
fn reply_vec(len: u32) -> Vec<u8> {
    vec![0; len as usize]
}

#[update(manual_reply)]
fn reject_vec(len: u32) {
    let bytes = vec![b'0'; len as usize];
    let message = String::from_utf8(bytes).expect("Failed to convert bytes to String");
    ic_cdk::api::msg_reject(message);
}
