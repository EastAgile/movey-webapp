use std::fmt;

use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

#[derive(AsJsonb, Debug, Deserialize, Serialize)]
pub struct I18nString {
    pub en: Option<String>,
    pub es: Option<String>,
    pub ja: Option<String>,
    pub cn: Option<String>,
    pub de: Option<String>
}

impl fmt::Display for I18nString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(en) = &self.en { return write!(f, "{}", en); }
        if let Some(es) = &self.es { return write!(f, "{}", es); }
        if let Some(ja) = &self.ja { return write!(f, "{}", ja); }
        if let Some(cn) = &self.cn { return write!(f, "{}", cn); }
        if let Some(de) = &self.de { return write!(f, "{}", de); }
        return write!(f, "");
    }
}
