mod model;

use crate::model::Function;
use hidapi::HidApi;
use model::generate_message;
use ron::extensions::Extensions;
use ron::ser::PrettyConfig;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write, Read},
};

fn main() {
    println!("Searching for naga v2 hyperspeed...");

    match HidApi::new() {
        Ok(api) => {
            'searchloop: for device in api.device_list() {
                let vid = device.vendor_id();
                let pid = device.product_id();
                if vid == 0x1532 && pid == 0x00b4 {
                    if device.interface_number() != 0 {
                        continue;
                    }

                    // Read messages from ron
                    let mut data = String::new();
                    let f = File::open("funcs.ron").expect("Unable to open file");
                    let mut br = BufReader::new(f);
                    br.read_to_string(&mut data).expect("Unable to read string");
                    let messages: Vec<Function> = ron::from_str(&data).unwrap();

                    // Read messages from json.
                    // let file = File::open("funcs.json").unwrap();
                    // let reader = BufReader::new(file);
                    // let messages: Vec<Function> = serde_json::from_reader(reader).unwrap();

                    // Write messages to json
                    // let json = serde_json::to_string_pretty(&messages).unwrap();
                    // let f = File::create("funcs.json").expect("Unable to create file");
                    // let mut f = BufWriter::new(f);
                    // f.write_all(json.as_bytes()).expect("Unable to write data");

                    // Write messages to ron
                    // let ron_pretty = PrettyConfig::new()
                    //     .indentor("  ".into())
                    //     .new_line("\n".into())
                    //     .compact_arrays(true)
                    //     .separate_tuple_members(false)
                    //     .extensions(Extensions::UNWRAP_VARIANT_NEWTYPES);
                    // let json = ron::ser::to_string_pretty(&messages, ron_pretty).unwrap();
                    // let f = File::create("funcs.ron").expect("Unable to create file");
                    // let mut f = BufWriter::new(f);
                    // f.write_all(json.as_bytes()).expect("Unable to write data");

                    println!("path: {}", device.path().to_string_lossy());
                    let mousey = api.open_path(device.path()).unwrap();

                    for msg in messages {
                        let mut message = generate_message(&msg);
                        if mousey.send_feature_report(&message).is_err() {
                            // If we run into an error here, it typically means we have the wrong device, so try the next one.
                            continue 'searchloop;
                        }

                        let mut buf = vec![0u8; 91];
                        let _ = mousey.get_feature_report(&mut buf).unwrap();
                        if buf[1] == 4 {
                            // Four means the send report operation didn't work
                            println!("Failed to send message. Make sure your mouse isn't asleep.");
                            break 'searchloop;
                        }
                        message[1] = buf[1];
                        assert_eq!(buf, message);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
