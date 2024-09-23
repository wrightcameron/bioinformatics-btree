use std::rc::Rc;
use std::cell::RefCell;
use crate::btree_node::Node;

// TODO Instead of turning cache into Node specific, could make it a interface and create methods in Node
pub struct BTreeCache {
    cache: Vec<Rc<RefCell<Node>>>,
    max_size: u32,
}

impl BTreeCache {
    //TODO  Handle if max size is set to 0 or less, should throw an error
    pub fn new(max_size: u32) -> Self {
        BTreeCache {
            cache: Vec::new(),
            max_size,
        }
    }

    // TODO To make this cache the best, should probably return Rc,RefCell
    pub fn get_object(&mut self, offset: u32) -> Option<Rc<RefCell<Node>>> {
        let index = self.cache.iter().position(|x| x.borrow().offset == offset)?;
        let res = self.cache.remove(index);
        //Move node to front of the cache
        self.cache.insert(0, res);
        self.cache.get(0).cloned()
        
    }

    // TODO What if the value already exists in the cache?
    pub fn add_object(&mut self, obj: Rc<RefCell<Node>>) {
        if self.cache.len() as u32 == self.max_size {
            self.cache.pop();
        }
        self.cache.insert(0, obj)
    }

    pub fn remove_object(mut self) -> Option<Rc<RefCell<Node>>>{
        self.cache.pop()
    }

    pub fn clear_cache(mut self) {
        self.cache.clear();
    }
}

// TODO Ya the tests will have to be redone, shame but easier than trying to make generics work atm
// #[cfg(test)]
// mod tests {
//     use super::*;#[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_btree_create_cache() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         assert_eq!(1,*cache.get_object(&1).unwrap());
//     }

//     #[test]
//     fn test_btree_not_exisitng() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         assert_ne!(None, cache.get_object(&1));
//     }

//     #[test]
//     fn test_btree_multiple_gets() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         cache.add_object(2);
//         cache.add_object(3);
//         cache.add_object(4);
//         assert_eq!(3, *cache.get_object(&3).unwrap());
//         assert_eq!(3, *cache.get_object(&3).unwrap());
//         assert_eq!(3, *cache.get_object(&3).unwrap());

//         assert_eq!(2, *cache.get_object(&2).unwrap());
//         assert_eq!(2, *cache.get_object(&2).unwrap());
//     }

//     #[test]
//     fn test_btree_fullcache() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(2);
//         cache.add_object(1);
//         cache.add_object(2);
//         cache.add_object(3);

//         assert_eq!(None, cache.get_object(&1));
//     }

// }

//     #[test]
//     fn test_btree_create_cache() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         assert_eq!(1,*cache.get_object(&1).unwrap());
//     }

//     #[test]
//     fn test_btree_not_exisitng() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         assert_ne!(None, cache.get_object(&1));
//     }

//     #[test]
//     fn test_btree_multiple_gets() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(10);
//         cache.add_object(1);
//         cache.add_object(2);
//         cache.add_object(3);
//         cache.add_object(4);
//         assert_eq!(3, *cache.get_object(&3).unwrap());
//         assert_eq!(3, *cache.get_object(&3).unwrap());
//         assert_eq!(3, *cache.get_object(&3).unwrap());

//         assert_eq!(2, *cache.get_object(&2).unwrap());
//         assert_eq!(2, *cache.get_object(&2).unwrap());
//     }

//     #[test]
//     fn test_btree_fullcache() {
//         let mut cache: BTreeCache<i64> = BTreeCache::new(2);
//         cache.add_object(1);
//         cache.add_object(2);
//         cache.add_object(3);

//         assert_eq!(None, cache.get_object(&1));
//     }

// }