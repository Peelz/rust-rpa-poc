use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Category {
    Consultation,
    Medicine,
}

impl Category {
    pub fn get_category_id(&self) -> i32 {
        match self {
            Category::Consultation => 1,
            Category::Medicine => 2,
        }
    }

    /// Note: To support CPF flow for now, recheck later if it needs to be this exact string
    pub fn to_legacy_category_name(&self) -> &'static str {
        match self {
            Category::Consultation => "APPOINTMENT",
            Category::Medicine => "PRODUCT_MEDICINE",
        }
    }
}
