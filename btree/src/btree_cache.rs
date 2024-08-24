
struct BTreeCache<T> {
    cache: Vec<T>,
    max_size: i32,
}

impl<T: std::cmp::PartialEq> BTreeCache<T>{
    //TODO  Handle if max size is set to 0 or less, should throw an error
    pub fn new(max_size: i32) -> Self {
        BTreeCache {
            cache: Vec::new(),
            max_size,
        }
    }

    pub fn get_object(&mut self,obj: &T) -> Option<&T> {
        let index = self.cache.iter().position(|x| x == obj)?;
        let res = self.cache.remove(index);
        self.cache.insert(0, res);
        self.cache.get(0)
        
    }

    pub fn add_object(&mut self, obj: T) {
        if self.cache.len() as i32 == self.max_size {
            self.cache.pop();
        }
        self.cache.insert(0, obj)
    }

    pub fn remove_object(mut self) -> Option<T>{
        self.cache.pop()
    }

    pub fn clear_cache(mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_create_cache() {
        let mut cache: BTreeCache<i64> = BTreeCache::new(10);
        cache.add_object(1);
        assert_eq!(1,*cache.get_object(&1).unwrap());
    }

    #[test]
    fn test_btree_not_exisitng() {
        let mut cache: BTreeCache<i64> = BTreeCache::new(10);
        cache.add_object(1);
        assert_ne!(None, cache.get_object(&1));
    }

    #[test]
    fn test_btree_multiple_gets() {
        let mut cache: BTreeCache<i64> = BTreeCache::new(10);
        cache.add_object(1);
        cache.add_object(2);
        cache.add_object(3);
        cache.add_object(4);
        assert_eq!(3, *cache.get_object(&3).unwrap());
        assert_eq!(3, *cache.get_object(&3).unwrap());
        assert_eq!(3, *cache.get_object(&3).unwrap());

        assert_eq!(2, *cache.get_object(&2).unwrap());
        assert_eq!(2, *cache.get_object(&2).unwrap());
    }

    #[test]
    fn test_btree_fullcache() {
        let mut cache: BTreeCache<i64> = BTreeCache::new(2);
        cache.add_object(1);
        cache.add_object(2);
        cache.add_object(3);

        assert_eq!(None, cache.get_object(&1));
    }

}