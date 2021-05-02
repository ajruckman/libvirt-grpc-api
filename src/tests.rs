extern crate libudev;

mod schema;

use std::any::Any;

use std::collections::HashMap;
use std::ops::Deref;
use virt::connect::Connect;

pub fn main() {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("usb").unwrap();

    let mut result: Vec<*const schema::schema::USBDevice> = vec![];

    for device in enumerator.scan_devices().unwrap() {
        let mut props: HashMap<String, String> = HashMap::new();

        for x in device.properties() {
            props.insert(
                x.name().to_string_lossy().to_string(),
                x.value().to_string_lossy().to_string(),
            );
        }

        if !props.contains_key("ID_MODEL_ID") || !props.contains_key("ID_VENDOR_ID") {
            continue;
        }

        // for p in props {
        //     println!("{} -> {}", p.0, p.1);
        // }

        let this = &schema::schema::USBDevice {
            device: props["DEVNAME"].clone(),
            model: props["ID_MODEL"].clone(),
            vendor_id: props["ID_VENDOR_ID"].clone(),
            product_id: props["ID_MODEL_ID"].clone(),
            vendor_name: match props.contains_key("ID_VENDOR_FROM_DATABASE") {
                true => Some(props["ID_VENDOR_FROM_DATABASE"].clone()),
                false => None,
            },
            model_name: match props.contains_key("ID_MODEL_FROM_DATABASE") {
                true => Some(props["ID_MODEL_FROM_DATABASE"].clone()),
                false => None,
            },
        };

        result.push(this);

        println!("{}", this);
    }

    return;

    let conn = Connect::open("qemu:///system").unwrap();

    let devices = conn.list_all_node_devices(0);

    for v in devices {
        for dev in v {
            println!("{:?}", dev.get_xml_desc(0).unwrap());
        }
    }
}
