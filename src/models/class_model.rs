use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model as WitherModel;
use validator::Validate;

use crate::utils::models::ModelExt;

impl ModelExt for Classes {}

#[derive(Debug, Clone, Serialize, Deserialize, WitherModel, Validate)]
#[model(index(keys = r#"doc!{ "name": 1, "created_at": 1 }"#))]
pub struct Classes {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub teacher: String,
    pub students: Vec<String>,
}

impl Classes {
    pub fn new(name: String, teacher: String) -> Self {
        Self {
            id: None,
            name,
            teacher,
            students: Vec::new(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PublicClasses {
    pub id: ObjectId,
    pub name: String,
    pub teacher: String,
    pub students: Vec<String>,
}

impl From<Classes> for PublicClasses {
  fn from(cat: Classes) -> Self {
    Self {
      id: cat.id.unwrap(),
      name: cat.name,
      teacher: cat.teacher,
      students: cat.students,
    }
  }
}