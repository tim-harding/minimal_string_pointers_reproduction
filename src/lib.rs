use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub type MyMap = HashMap<StaticStr, u8>;
pub const A: &str = "A";

pub struct StaticStr(&'static str);

impl Hash for StaticStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = self.0.as_ptr();
        println!("{:?}", ptr);
        ptr.hash(state)
    }
}

impl PartialEq for StaticStr {
    fn eq(&self, other: &Self) -> bool {
        let p1 = self.0.as_ptr();
        let p2 = other.0.as_ptr();
        println!("{:?} {:?}", p1, p2);
        p1 == p2
    }
}

impl Eq for StaticStr {}

pub fn make_map() -> MyMap {
    let mut map = MyMap::new();
    map.insert(StaticStr(A), 1);
    map
}

pub fn get_value(control: &MyMap) -> Option<u8> {
    control.get(&StaticStr(A)).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn map_made_in_lib() {
        let map = make_map();
        assert_eq!(get_value(&map), Some(1));
    }

    #[test]
    pub fn map_made_in_test() {
        // Same as make_map()
        let mut map = MyMap::new();
        map.insert(StaticStr(A), 1);

        // This check fails
        assert_eq!(get_value(&map), Some(1));
    }
}
