use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model as WitherModel;
use validator::Validate;

use crate::utils::models::ModelExt;

impl ModelExt for Student {}

#[derive(Debug, Clone, Serialize, Deserialize, WitherModel, Validate)]
#[model(index(keys = r#"doc!{ "name": 1, "created_at": 1 }"#))]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub age: u8,
}

impl Student {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            id: None,
            name,
            age,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PublicStudent {
    pub id: ObjectId,
    pub name: String,
    pub age: u8,
}

impl From<Student> for PublicStudent {
  fn from(cat: Student) -> Self {
    Self {
      id: cat.id.unwrap(),
      name: cat.name,
      age: cat.age,
    }
  }
}