use ic_cdk::update;

#[update(manual_reply)]
fn var_arg() {
    ic_cdk::api::msg_reply(vec![]);
}

#[update(decode_with = "parse_len", manual_reply)]
fn reply_vec(len: u16) {
    let reply = vec![0u8; len as usize];
    ic_cdk::api::msg_reply(&reply);
}

#[update(decode_with = "parse_len", manual_reply)]
fn reject_vec(len: u16) {
    let bytes = vec![b'0'; len as usize];
    let message = String::from_utf8(bytes).expect("Failed to convert bytes to String");
    ic_cdk::api::msg_reject(message);
}

fn parse_len(arg_bytes: Vec<u8>) -> u16 {
    u16::from_be_bytes(
        arg_bytes
            .as_slice()
            .try_into()
            .expect("Failed to convert bytes to u16"),
    )
}
