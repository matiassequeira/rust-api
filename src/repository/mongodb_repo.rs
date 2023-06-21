use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection, Cursor},
};
use crate::models::{user::User, product::Product};

use log::{debug};

pub struct MongoRepo {
    users: Collection<User>,
    products: Collection<Product>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let users: Collection<User> = db.collection("User");
        let products: Collection<Product> = db.collection("Product");
        MongoRepo { users, products }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        debug!("Creating user in DB");
        let user = self
            .users
            .insert_one(new_user, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &ObjectId) -> Result<User, Error> {
        let user = self
            .users
            .find_one(doc!{"_id": id}, None)
            .ok()
            .expect("Error getting user");
        Ok(user.unwrap())
    }

    pub fn update_user(&self, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = new_user.id.unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        debug!("Updating user in DB");
        let updated_doc = self
            .users
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        info!("User updated successfully");
        Ok(updated_doc)

    }

    pub fn delete_user(&self, id: &ObjectId) -> Result<DeleteResult, Error> {
        let delete_res = self
            .users
            .delete_one(doc! {"_id": id}, None)
            .ok()
            .expect("Error removing user");
        Ok(delete_res)
    }

    pub fn get_all_users(&self) -> Result<Cursor<User>, Error> {
        let users = self
            .users
            .find(None, None)
            .ok()
            .expect("Error retrieving users list");
        Ok(users)
    }

    pub fn create_product(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        debug!("Creating product in DB");
        let product = self
            .products
            .insert_one(new_product, None)
            .ok()
            .expect("Error creating product");
        Ok(product)
    }

    pub fn get_product(&self, id: &ObjectId) -> Result<Product, Error> {
        let product = self
            .products
            .find_one(doc!{"_id": id}, None)
            .ok()
            .expect("Error getting product");
        Ok(product.unwrap())
    }

    pub fn update_product(&self, new_product: Product) -> Result<UpdateResult, Error> {
        let obj_id = new_product.id.unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_product.id,
                    "name": new_product.name,
                    "vendor": new_product.vendor,
                    "sku": new_product.sku,
                    "origin": new_product.origin,
                    "price": new_product.price,
                },
        };
        debug!("Updating product in DB");
        let updated_doc = self
            .products
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating product");
        info!("Product updated successfully");
        Ok(updated_doc)

    }

    pub fn delete_product(&self, id: &ObjectId) -> Result<DeleteResult, Error> {
        let delete_res = self
            .products
            .delete_one(doc! {"_id": id}, None)
            .ok()
            .expect("Error removing product");
        Ok(delete_res)
    }

    pub fn get_all_products(&self) -> Result<Cursor<Product>, Error> {
        let products = self
            .products
            .find(None, None)
            .ok()
            .expect("Error retrieving products list");
        Ok(products)
    }

}