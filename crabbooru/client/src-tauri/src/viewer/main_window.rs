use image;

pub struct MainWindow {}

impl Default for MainWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl MainWindow {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&mut self) {}

    pub fn options() {
        todo!()
    }
    pub fn options_closed() {
        todo!()
    }
    pub fn save_folder() {
        todo!()
    }
    pub fn add_tab(_tag: &str, _save: bool) {
        todo!()
    }
    pub fn add_gallery_tab(_gallery: Vec<image::DynamicImage>, _save: bool) {
        todo!()
    }
    pub fn save_tab() {
        todo!()
    }

    pub fn load_md5(_path: &str, _is_new_tab: bool, _is_save: bool) {
        todo!()
    }
    pub fn load_tag(_tag: &str, _is_new_tab: bool, _is_save: bool) {
        todo!()
    }
    pub fn load_tag_tab(_tag: &str) {
        todo!()
    }
    pub fn load_tag_no_tab(_tag: &str) {
        todo!()
    }
    pub fn load_gallery(_gallery: Vec<image::DynamicImage>, _is_new_tab: bool, _is_save: bool) {
        todo!()
    }

    pub fn set_source(&mut self, _site: &str) {}
}
