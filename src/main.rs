use std::process::Command;

fn main() {
    println!("Checking scroll direction");

    let direction = get();

    println!("Set to {}", direction);
}

fn get() -> bool {
    let output = Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("com.apple.swipescrolldirection")
        .output()
        .expect("Could not read scroll direction 1");
    if output.status.code() == Some(0) {
        let value = String::from_utf8_lossy(&output.stdout);
        let value = value.trim();
        if value == "0".to_owned() {
            return false;
        } else if value == "1".to_owned() {
            return true;
        }
    }
    panic!("Could not read scroll direction");
}
