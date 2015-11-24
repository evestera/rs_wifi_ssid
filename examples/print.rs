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
