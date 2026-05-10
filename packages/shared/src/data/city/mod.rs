pub mod tokyo;

use crate::location::RegionDatabase;

pub static TOKYO_DB: RegionDatabase = RegionDatabase::new(tokyo::TOKYO);
