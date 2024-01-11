mod model;

use hidapi::HidApi;
use model::{Function, MouseFunction, MouseButton, generate_message};

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
                        continue
                    }
                    println!("path: {}", device.path().to_string_lossy());
                    let mousey = api.open_path(device.path()).unwrap();
                    let func = Function::Mouse(MouseButton::Side4, MouseFunction::Button(MouseButton::Mouse5, 0));
                    let message = generate_message(&func);

                    // match mousey.send_feature_report(&message) {
                    //     Ok(ok) => {println!("awesome!"); break},
                    //     Err(err) => {println!("poo"); continue}
                    // }


                     // println!("message: {message:02x?}");
                    // println!("control: {m4bytes11:02x?}");
                    // assert_eq!(message, m4bytes11);
                    // println!("yay!");
                    // let mut buf = Vec::new();
                    // buf.push(0);
                    // let reply_len = mousey.get_feature_report(&mut buf).unwrap();
                    // println!("reply len: {reply_len} reply: {:04x?}", buf)
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}
