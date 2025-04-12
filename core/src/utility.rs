use std::str::FromStr;

pub trait ParseValue {
    fn parse_value<T: FromStr>(&self, err_msg: &str) -> Result<T, String>;
}

impl ParseValue for &str {
    fn parse_value<T: FromStr>(&self, err_msg: &str) -> Result<T, String> {
        self.parse().map_err(|_| err_msg.to_string())
    }
}