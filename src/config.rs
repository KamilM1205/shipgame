pub mod config {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Config {
        title: String,
        width: i32,
        height: i32,
        is_resizable: bool,
        is_fullscreen: bool,
    }

    impl Config {
       /* async fn load() -> Self {
            let config_file = String::from_utf8(
                load_file("config.toml").await.unwrap()).unwrap();
            let mut conf: Config;
            if config_file.is_empty() {
                conf = Config {
                    ..Default::default()
                };
            }
            Config {
            }
        }*/
    }
}