# rs_wifi_ssid

Helpers for reading WiFi names.

Currently only works on OS X and Linux (only current ssid).

```rust
extern crate rs_wifi_ssid;

fn main() {
    match rs_wifi_ssid::current() {
        Some(ssid) => println!("Currently connected to '{}'", ssid),
        None => println!("No current wifi connection"),
    }

    let seeing = rs_wifi_ssid::list_all();
    println!("Currently seeing {} networks:", seeing.len());
    println!("{:?}", seeing);
}
```
