#[derive(Copy, Clone)]
pub struct AppConfig {
    single_pic_size: (i32, i32),
}

impl AppConfig {

    pub fn new(width: i32, height: i32) -> AppConfig {
        AppConfig {
            single_pic_size: (width, height),
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.single_pic_size
    }

}