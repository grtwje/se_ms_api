use std::env;
use std::fs;
use std::sync::Once;

pub const TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

const TEST_CREDENTIALS_FILE: &'static str = "test_credentials.txt";

static mut SITE_ID: String = String::new();
static mut API_KEY: String = String::new();
static INIT: Once = Once::new();

pub fn get_site_id_and_api_key() -> (&'static str, &'static str) {
    unsafe {
        INIT.call_once(|| {
            let path = env::current_dir().unwrap();
            let path = path.join("tests").join(TEST_CREDENTIALS_FILE);

            let contents = fs::read_to_string(path)
                .expect(&format!("Unable to read {}.", TEST_CREDENTIALS_FILE));
            let mut lines = contents.lines();
            if let Some(s) = lines.next() {
                SITE_ID = s.to_string();
            }
            if let Some(s) = lines.next() {
                API_KEY = s.to_string();
            }
            if SITE_ID.len() == 0 || API_KEY.len() == 0 {
                panic!("Ill formed credentials file.");
            }
        });
        (&SITE_ID, &API_KEY)
    }
}
