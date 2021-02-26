use parity_scale_codec::{Encode, Decode};

#[derive(Encode, Decode)]
struct MessagePayload {
    spec_version: u32,
    weight: u64,
    origin: CallOrigin,
    call: Vec<u8>,
}

#[derive(Encode, Decode)]
enum CallOrigin {
    SourceRoot,
    TargetAccount([u8;32], [u8;64], [u8;64]),
    SourceAccount([u8;32]),
}

fn print_encoding<T: Encode>(id: &str, t: T) {
    let bytes = t.encode();
    println!("{}: 0x{}", id, hex::encode(&bytes))
}

fn main() {
    print_encoding("Number", 1_u64);

    let root = ("mmr", 1_u64);
    print_encoding("mmr_key", root);

    let hex: Vec<u8> = vec![
        12, 109, 109, 114, 1, 0, 0, 0, 0, 0, 0, 0
    ];
    println!("0x{}", hex::encode(&hex));

    print_encoding("vector", vec![0xf_u8]);

    let origin = hex_literal::hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    print_encoding("payload", MessagePayload {
        spec_version: 1,
        weight: 1279000,
        origin: CallOrigin::SourceAccount(origin),
        call: vec![4, 1, 84, 85, 110, 105, 120, 32, 116, 105, 109, 101, 58, 32, 49, 54, 49, 52, 50, 54, 48, 51, 50, 50],
    });
}
