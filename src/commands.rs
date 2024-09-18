use atat::atat_derive::AtatCmd;
use heapless::String;
use super::responses::{OkResponse, OnOff};
use std::str::FromStr;

#[derive(AtatCmd, Clone)]
#[at_cmd("+CPIN", OkResponse)]
pub struct GetCpin;

/// 4.1.1 AT - Verify COM is working
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("AT", OkResponse, cmd_prefix = "", timeout_ms = 5000)]
pub struct VerifyComIsWorking {}

/// 4.1.3 Get ATE - Echo is on/off
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+ATE=?", OnOff)]
pub struct AteGet {}

/// 4.1.3 Set ATE - Echo is on/off
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+ATE", OnOff, quote_escape_strings = false, timeout_ms = 8000)]
pub struct AteSet {
    pub on: String<6>,
}

impl AteSet {
    pub fn on() -> Self {
        Self {
            on: String::<6>::from_str("ON").unwrap(),
        }
    }
    pub fn off() -> Self {
        Self {
            on: String::from_str("OFF").unwrap(),
        }
    }
}

/// 4.1.5 Get Sleep status
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+SLEEP=?", OnOff)]
pub struct SleepGet {}

/// 4.1.5 Set Sleep status
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+SLEEP", OnOff, quote_escape_strings = false, timeout_ms = 8000)]
pub struct SleepSet {
    pub on: String<6>,
}

impl SleepSet {
    pub fn on() -> Self {
        Self {
            on: String::<6>::from_str("ON").unwrap(),
        }
    }
    pub fn off() -> Self {
        Self {
            on: String::<6>::from_str("OFF").unwrap(),
        }
    }
}

///4.1.6 Reset
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+RESET", OkResponse, timeout_ms = 2000)]
pub struct Reset {}

#[cfg(test)]
mod tests {
    use super::{AteGet, AteSet, SleepSet, VerifyComIsWorking};
    use atat::AtatCmd;

    #[test]
    fn verify_com_is_working_serializes_correctly() {
        let k = VerifyComIsWorking {};
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 4);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)), "AT\r\n");
    }

    #[test]
    fn ate_get() {
        let k = AteGet {};
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 10);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)),"AT+ATE=?\r\n");
    }

    #[test]
    fn ate_set() {
        let k = AteSet::on();
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 11);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)),"AT+ATE=ON\r\n");
        let k = AteSet::off();
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 12);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)),"AT+ATE=OFF\r\n");
    }

    #[test]
    fn sleep_set() {
        let k = SleepSet::on();
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 13);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)),"AT+SLEEP=ON\r\n");
        let k = SleepSet::off();
        let mut buffer = vec![0;127];
        let bytes_written = k.write(&mut buffer);
        assert_eq!(bytes_written, 14);
        assert_eq!(String::from_utf8(buffer).unwrap().trim_matches(char::from(0)),"AT+SLEEP=OFF\r\n");
    }
}