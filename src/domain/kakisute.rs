pub struct Kakisute {
    content: String,
}

impl Kakisute {
    pub fn new(content: String) -> Self {
        Kakisute { content }
    }

    pub fn content(self) -> String {
        self.content
    }
}
