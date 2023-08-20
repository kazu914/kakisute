use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub struct TextInput {
    value: RefCell<String>,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            value: RefCell::new(String::new()),
        }
    }

    pub fn get(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn clear(&self) {
        self.value.borrow_mut().clear()
    }
    pub fn push(&self, c: char) {
        self.value.borrow_mut().push(c)
    }

    pub fn pop(&self) -> Option<char> {
        self.value.borrow_mut().pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input() {
        let text_input = TextInput::new();
        assert_eq!(text_input.get(), "");
        text_input.push('a');
        assert_eq!(text_input.get(), "a");
        text_input.push('b');
        assert_eq!(text_input.get(), "ab");
        text_input.pop();
        assert_eq!(text_input.get(), "a");
        text_input.clear();
        assert_eq!(text_input.get(), "");
    }
}
