use crate::ext::CFArrayExt;
use core_foundation::string::CFString;
use system_configuration::dynamic_store::SCDynamicStoreBuilder;

pub fn dynstore_display(name: &CFString) {
    let dynstore = SCDynamicStoreBuilder::new(name.clone()).build();
    let keys = dynstore.get_keys(".*").unwrap().into_collect::<Vec<_>>();
    println!("keys: {:#?}", keys);
}
