use crate::HotKey;

pub trait Scanner {
    fn scan(&self) -> Vec<HotKey>;
}
