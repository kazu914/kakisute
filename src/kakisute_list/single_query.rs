pub struct SingleQuery {
    pub is_latest: bool,
    pub file_name: Option<String>,
}

impl SingleQuery {
    pub fn new(is_latest: bool, file_name: Option<String>) -> Self {
        SingleQuery {
            is_latest,
            file_name,
        }
    }
}
