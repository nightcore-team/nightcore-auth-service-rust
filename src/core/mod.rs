pub mod config;

pub mod env {
    use dotenv::dotenv;

    pub fn load_dotend() {
        dotenv().ok();
    }
}
