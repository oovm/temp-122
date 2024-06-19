use enhance::{EnhanceLevel, EnhanceMap};
use std::collections::BTreeMap;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut mappings = BTreeMap::default();
    mappings.insert(0, EnhanceLevel::simple(0.5, 0.5, 0));
    mappings.insert(1, EnhanceLevel::simple(0.5, 0.5, 0));
    mappings.insert(2, EnhanceLevel::simple(0.5, 0.5, 0));
    mappings.insert(3, EnhanceLevel::simple(0.5, 0.5, 0));
    mappings.insert(4, EnhanceLevel::simple(0.5, 0.5, 0));
    let map = EnhanceMap::<String> { mapping: mappings };
    println!("{}", map.as_matrix().as_wolfram(false, true));
    // let json = serde_json::to_string_pretty(&map).unwrap();
    // println!("{}", json);
}
