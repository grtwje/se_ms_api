use se_ms_api::SolaredgeCredentials;
use std::env;
use std::fs;

pub const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

const TEST_CREDENTIALS_FILE: &str = "test_credentials.txt";

lazy_static! {
    pub static ref TEST_CREDENTIALS: SolaredgeCredentials = {
        let mut site_id = String::new();
        let mut api_key = String::new();

        let path = env::current_dir().unwrap();
        let path = path.join("tests").join(TEST_CREDENTIALS_FILE);

        let contents = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Unable to read {}.", TEST_CREDENTIALS_FILE));
        let mut lines = contents.lines();
        if let Some(s) = lines.next() {
            site_id = s.to_string();
        }
        if let Some(s) = lines.next() {
            api_key = s.to_string();
        }
        if site_id.is_empty() || api_key.is_empty() {
            panic!("Ill formed credentials file.");
        }

        SolaredgeCredentials::create(&site_id, &api_key)
    };
}
