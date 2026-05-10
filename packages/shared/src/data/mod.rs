pub mod city;
pub mod prefecture;

use crate::location::RegionDatabase;

pub static PREFECTURE_DB: RegionDatabase = RegionDatabase::new(prefecture::PREFECTURES);
pub use city::TOKYO_DB;
