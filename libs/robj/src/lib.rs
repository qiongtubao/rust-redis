trait Obj {

}
trait  dictType {
    fn hashFunction(key: dictKey) -> u32;
//    fn keyDup(data: dyn Obj, key: dictKey);
//    fn valDup(data: dyn Obj, value: dictValue);
//    fn keyCompare(data: dyn Obj, key1: dictKey, key2: dictKey) -> u8;
//    fn keyDestructor(data: dyn Obj, key: dictKey);
//    fn valDestructor(data: dyn Obj, value: dictValue);
}

struct dict {
    table: Vec<dictEntry>,
    t: dyn dictType,
    size: u64,
    sizemask: u64,
    used: u64,
    data: dyn Obj
}

impl dict {
    pub fn new(t: dyn dictType, data: dyn Obj) -> dict {
        dict {
            table: None,
            t,
            size: 0,
            sizemask: 0,
            used: 0,
            data
        }
    }
    pub fn dictReset(&mut self) {
        self.table = None;
        self.size = 0;
        self.sizemask = 0;
        self.used = 0;
    }
    pub fn add(&mut self, key: dictKey, value: dictValue) -> u32{
        let index = self.key_index(key);
        if index == -1 {
            return 0;
        }
        let entry = dictEntry::new();
        entry.set_next(self.table[index]);
        self.table[index] = entry;
        self.set_hashKey(entry, key);
        self.set_hashValue(entry, key);
        self.used += 1;
        return 1;
    }
    pub fn key_index(&self, key: dictKey) -> u32{
        if self.expand_if_needs() == 1 {
            return -1;
        }
        let h = self.hash_key(key) & self.sizemask;
        let he = ht.table[h];
        loop {
            if self.compare_hash_keys(key, he.key) {
                return -1;
            }
            he = he.next();
        }
        return h;
    }
    pub fn expand_if_needs(&self) {

    }
    fn hash_key(&self, key: dictKey) -> u32{
        self.t.hashFunction(key)
    }
    fn compare_hash_keys(&self, key1: dictKey, key2: dictKey) {
        self.t.key_compare(self.data, key1, key2)
    }
    fn set_hash_key(&self, entry: Entry, key: dictKey) {
        entry.key  = self.f.key_dup(self.data, key);
    }
    fn set_hash_value(&self, entry: Entry, value: dictValue) {
        self.entryself.t.valdup(self.data, value);
    }

}

struct dictIterator {
    ht: dict,
    index: u8,
    entry: dictEntry,
    nextEntry: dictEntry,
}



struct dictEntry {
    key: dictKey,
    val: dictValue,
}

