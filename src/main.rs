mod model;

use crate::model::Function;
use hidapi::HidApi;
use model::generate_message;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                let vid = device.vendor_id();
                let pid = device.product_id();
                // println!("{:04x}:{:04x}", vid,pid );
                if vid == 0x1532 && pid == 0x00b4 {
                    if device.interface_number() != 0 {
                        continue;
                    }

                    let file = File::open("funcs.json").unwrap();
                    let reader = BufReader::new(file);

                    // Read the JSON contents of the file as an instance of `User`.
                    let messages: Vec<Function> = serde_json::from_reader(reader).unwrap();

                    let json = serde_json::to_string_pretty(&messages).unwrap();
                    let f = File::create("funcs.json").expect("Unable to create file");
                    let mut f = BufWriter::new(f);
                    f.write_all(json.as_bytes()).expect("Unable to write data");

                    println!("path: {}", device.path().to_string_lossy());
                    let mousey = api.open_path(device.path()).unwrap();

                    for msg in messages {
                        let message = generate_message(&msg);
                        // match mousey.send_feature_report(&message) {
                        //     Ok(_) => {
                        //         println!("awesome!")
                        //     }
                        //     Err(_) => {
                        //         println!("poo");
                        //         continue;
                        //     }
                        // }

                        let mut buf = vec![0u8; 91];
                        let _ = mousey.get_feature_report(&mut buf).unwrap();
                        if buf[1] == 4 {
                            // Four means the send report operation didn't work
                            println!("Failed to send message. Make sure your mouse isn't asleep.");
                            break;
                        }
                        println!("{:02x?}", buf);
                        println!("{:02x?}", message);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
