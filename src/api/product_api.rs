use crate::{models::{product::Product}, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::{ObjectId};
// use mongodb::results::InsertOneResult;
use log::{error, debug};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Serialize, Deserialize};

#[post("/", data = "<new_product>")]
pub fn create_product(
    db: &State<MongoRepo>,
    new_product: Json<Product>,
) -> Result<Json<Product>, Status> {
    let data = Product {
        id: None,
        name: new_product.name.to_owned(),
        vendor: new_product.vendor.to_owned(),
        sku: new_product.sku.to_owned(),
        origin: new_product.origin.to_owned(),
        price: new_product.price.to_owned(),
    };
    debug!("Creating product");
    let insert_result = db.create_product(data);
    match insert_result {
        Ok(res) => {
            if let Some(inserted_id) = res.inserted_id.as_object_id() {
                debug!("Product created successfully");
                let updated_product_info = db.get_product(&inserted_id);
                return match updated_product_info {
                    Ok(product) => Ok(Json(product)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                error!("Error occurred: Cannot retrieve ID for inserted object");
                Err(Status::InternalServerError)
            }
        },
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>")]
pub fn get_product(
    db: &State<MongoRepo>,
    id: &str,
) -> Result<Json<Product>, Status> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let product_detail = db.get_product(&obj_id);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/", data= "<product_data>")]
pub fn update_product(
    db: &State<MongoRepo>,
    product_data: Json<Product>,
) -> Result<Json<Product>, Status> {
    let product_instance: Product = product_data.into_inner();
    
    let product_id= product_instance.id.clone();
    debug!("Updating product...");
    let update_result = db.update_product(product_instance);
    match update_result {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                debug!("Product updated successfully. Retrieving it back");
                if let Some(ref object_id) = &product_id {
                    let updated_product_info = db.get_product(&object_id);
                    return match updated_product_info {
                        Ok(product) => Ok(Json(product)),
                        Err(_) => Err(Status::InternalServerError),
                    };
                } else {
                    error!("Couldn't retrieve updated object ID");
                    Err(Status::InternalServerError)
                }
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeleteMessage {
    pub _id: String,
}

#[post("/delete", data= "<msg>")]
pub fn delete_product(
    db: &State<MongoRepo>,
    msg: Json<DeleteMessage>
) -> Result<Json<String>, Status> {
    let msg_json = msg.into_inner();
    debug!("Deleting Product with ID {}", msg_json._id);
    let obj_id = ObjectId::parse_str(msg_json._id).unwrap();
    let delete_res = db.delete_product(&obj_id);
    match delete_res {
        Ok(_res) => Ok(Json(String::from("Ok"))),
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/all")]
pub fn get_all_product(
    db: &State<MongoRepo>,
) -> Result<Json<Vec<Product>>, Status> {
    let mut products: Vec<Product> = vec![];
    let get_all_res = db.get_all_products();
    
    let cursor = match get_all_res {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("Error occurred: {}", e);
            return Err(Status::InternalServerError)
        },
    };

    for res in cursor {
        match res {
            Ok(product) => {
                products.push(product);
            }
            Err(e) => {
                error!("Error occurred: {}", e);
                return Err(Status::InternalServerError)
            },
        }
    }

    Ok(Json(products))
}

