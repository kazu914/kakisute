use super::App;

impl App {
    pub fn list(&self) {
        self.kakisute_list.print_list();
    }
}
