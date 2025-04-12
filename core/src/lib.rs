pub enum TestStruct {
    Both,
}

impl TestStruct {
    pub fn test(self) {
        println!("I took ownership");
    }
}