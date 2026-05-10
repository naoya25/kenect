pub mod city;
pub mod prefecture;

use crate::location::RegionDatabase;

pub static PREFECTURE_DB: RegionDatabase = RegionDatabase::new(prefecture::PREFECTURES);
pub static CITY_DB: RegionDatabase = RegionDatabase::new(city::CITIES);
