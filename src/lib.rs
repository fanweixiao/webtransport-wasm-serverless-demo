extern "C" {
    fn yomo_observe_datatag(tag: u8);
    fn yomo_load_input(pointer: *mut u8);
    fn yomo_dump_output(tag: u8, pointer: *const u8, length: usize);
}

#[macro_use]
extern crate serde_derive;
extern crate base64;

use std::str;
use base64::{decode, encode};

#[no_mangle]
pub extern "C" fn yomo_init() {
    unsafe {
        // Observe the data tag of the input data from https://webtransport.day/serverless
        yomo_observe_datatag(0x30);
    }
}

#[no_mangle]
pub extern "C" fn yomo_handler(input_length: usize) {
    println!("yomo wasm sfn received {} bytes", input_length);

    // load input data
    let mut input = Vec::with_capacity(input_length);
    unsafe {
        yomo_load_input(input.as_mut_ptr());
        input.set_len(input_length);
    }

    // parse Vec<u8> to String
    let str = str::from_utf8(&input).unwrap();
    println!("> received: {}", str);

    // Step 1: Deserilize input data as struct `Message`
    // deserilize String to struct `Message`
    let mut parse: Message = serde_json::from_slice(&input).unwrap();
    // base64 decode the `data` field
    let data = parse.data;
    let res = decode(&data).unwrap();
    let ddd = str::from_utf8(&res).unwrap();
    println!("> data: {:?}", ddd);

    // Step 2: Process the input data, we just uppercase all inputs here
    let response = ddd.to_ascii_uppercase();
    // then base64 encode the response
    parse.data = encode(response.to_string());

    // Step 3: Serilize the response as Vec<u8>
    let result = serde_json::to_string(&parse).unwrap();
    println!("< response: {}", result);
    let output = result.as_bytes();

    // dump output data and set tag to 0x31 to response to https://webtransport.day/serverless
    unsafe {
        yomo_dump_output(0x31, output.as_ptr(), output.len());
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageMeta {
    mode: String,
    stream_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    meta: MessageMeta,
    data: String,
}