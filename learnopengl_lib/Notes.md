
# Size of Rust structs


## mem::size_of_val
Example struct with Vecs

    pub struct MemTableEntry {
        pub key: Vec<u8>,
        pub value: Option<Vec<u8>>,
        pub timestamp: u128,
        pub deleted: bool,
    }
    
    let entry = MemTableEntry{
        key: b"Hello".to_vec(),
        value: Some(b"World".to_vec()),
        timestamp: 123u128,
        deleted: false
    };
    
    let size = mem::size_of_val(&entry.key)
               + mem::size_of_val(&entry.value)
               + mem::size_of_val(&entry.timestamp)
               + mem::size_of_val(&entry.deleted);
    
    assert_eq!(65, size);
