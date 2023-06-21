use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};

#[derive(Debug, Deserialize)]
pub struct User {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_object_id")]
    // NOTE: this rename is just internal. Rust expects a JSON with the _id field
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

// fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: Option<String> = Option::deserialize(deserializer)?;
//     match s {
//         Some(s) => ObjectId::parse_str(&s).map_err(de::Error::custom).map(Some),
//         None => Ok(None),
//     }
// }

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 4)?;
        if let Some(ref object_id) = &self.id {
            s.serialize_field("_id", &object_id.to_string())?;
        } 
        
        s.serialize_field("name", &self.name)?;
        s.serialize_field("location", &self.location)?;
        s.serialize_field("title", &self.title)?;
        s.end()
    }
}