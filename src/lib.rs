#[macro_use]
extern crate objc;
extern crate objc_foundation;

use objc::runtime::Class;

use objc_foundation::{INSArray, NSArray, NSString, INSString, NSObject, INSFastEnumeration};

#[link(name = "CoreWLAN", kind = "framework")]
extern {
}

pub fn current_ssid() -> Option<String> {
    let cls = match Class::get("CWWiFiClient") {
        Some(c) => c,
        None => return None,
    };

    unsafe {
        let client: *const NSObject = msg_send![cls, sharedWiFiClient];
        let interface: *const NSObject = msg_send![client, interface];
        if interface.is_null() {
            return None;
        }
        let ssid: *const NSString = msg_send![interface, ssid];
        if ssid.is_null() {
            return None;
        }
        Some(String::from((*ssid).as_str()))
    }
}

pub fn list_all_networks() -> Vec<String> {
    let cls = match Class::get("CWWiFiClient") {
        Some(c) => c,
        None => return vec![],
    };

    unsafe {
        let client: *const NSObject = msg_send![cls, sharedWiFiClient];
        let interface: *const NSObject = msg_send![client, interface];
        if interface.is_null() {
            return vec![];
        }

        // Could potentially use [interface cachedScanResults] instead
        let set: *const NSObject = msg_send![interface, scanForNetworksWithName:0 error:0];
        let arr: *const NSArray<NSObject> = msg_send![set, allObjects];

        (*arr)
            .enumerator()
            .map(|network| {
                let ssid: *const NSString = msg_send![network, ssid];
                String::from((*ssid).as_str())
            })
            .collect()
    }
}
