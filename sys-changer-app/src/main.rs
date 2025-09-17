use core_foundation::string::CFString;
#[cfg(target_os = "macos")]
use security_framework::authorization::Authorization;

#[cfg(target_os = "macos")]
pub fn main() {
    // constants
    let proc_name = CFString::new("sys-changer-app");
    let my_networkset_name = CFString::new("sys-changer-app-networkset");

    // grab authorization
    let authorization = Authorization::default().unwrap();
}

#[cfg(not(target_os = "macos"))]
pub fn main() {
    panic!("Non-macOS systems are not supported");
}
