pub mod others {
    #[get("/world")]
    pub fn index_1() ->&'static str {
        "World!"
    }
}