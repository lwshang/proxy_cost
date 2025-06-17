use ic_cdk::update;

#[update]
fn empty() {}

#[update]
fn reply_vec(len: u32) -> Vec<u8> {
    vec![0; len as usize]
}
