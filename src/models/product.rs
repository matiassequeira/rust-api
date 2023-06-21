use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};

#[derive(Debug, Deserialize)]
pub struct Product {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_object_id")]
    // NOTE: this rename is just internal. Rust expects a JSON with the _id field
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub vendor: String,
    pub sku: String,
    pub origin: String,
    pub price: f64,
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

impl Serialize for Product {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Product", 6)?;
        
        if let Some(ref object_id) = &self.id {
            s.serialize_field("_id", &object_id.to_string())?;
        } 
        s.serialize_field("name", &self.name)?;
        s.serialize_field("vendor", &self.vendor)?;
        s.serialize_field("sku", &self.sku)?;
        s.serialize_field("origin", &self.origin)?;
        s.serialize_field("price", &self.price)?;
        s.end()
    }
}