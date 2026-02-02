use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub user_email: Option<String>, // None = default category
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub color: Option<String>,
}

pub fn default_categories() -> Vec<Category> {
    vec![
        Category {
            id: "1".to_string(),
            user_email: None,
            name: "Food & Dining".to_string(),
            color: Some("#ef4444".to_string()),
        },
        Category {
            id: "2".to_string(),
            user_email: None,
            name: "Transportation".to_string(),
            color: Some("#f97316".to_string()),
        },
        Category {
            id: "3".to_string(),
            user_email: None,
            name: "Shopping".to_string(),
            color: Some("#eab308".to_string()),
        },
        Category {
            id: "4".to_string(),
            user_email: None,
            name: "Entertainment".to_string(),
            color: Some("#22c55e".to_string()),
        },
        Category {
            id: "5".to_string(),
            user_email: None,
            name: "Bills & Utilities".to_string(),
            color: Some("#3b82f6".to_string()),
        },
        Category {
            id: "6".to_string(),
            user_email: None,
            name: "Health".to_string(),
            color: Some("#8b5cf6".to_string()),
        },
        Category {
            id: "7".to_string(),
            user_email: None,
            name: "Other".to_string(),
            color: Some("#6b7280".to_string()),
        },
    ]
}
