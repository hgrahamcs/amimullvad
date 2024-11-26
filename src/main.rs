use std::io::{stdout, Write};
use std::process::exit;
use curl::easy::Easy;

fn main(){

    let mut easy = Easy::new();
    easy.url("https://am.i.mullvad.net/connected").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        if find_sub(data, b"not") {
            println!("Not connected to Mullvad VPN");
            exit(1);
        } else {
            println!("Connected to Mullvad VPN");
            exit(0)
        }
        // I should be returning Result<> here but just exit we don't do anything with it.
    }).unwrap();
    easy.perform().unwrap();
}


fn find_sub(main: &[u8], sub: &[u8]) -> bool {

    // Check that we aren't doing something impossible
    if sub.len() > main.len() {
        return false;
    }

    // For each byte in the slice, check if consecutive bytes match the sub slice
    for i in 0..main.len() {
        if main[i..].starts_with(sub) {
            return true;
        }
    }

    false
}