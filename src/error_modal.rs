pub struct ErrorModal {
    pub title: String,
    pub caption: String,
    pub show: bool,
}
impl ErrorModal {
    pub fn new(title: String) -> Self {
        Self {
            title,
            caption: String::new(),
            show: false,
        }
    }
    pub fn set(&mut self, caption: String, show: bool) {
        self.caption = caption;
        self.show = show;
    }
    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            caption: self.caption.clone(),
            show: self.show,
        }
    }
}