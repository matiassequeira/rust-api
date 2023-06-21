use crate::{models::user::User, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::{ObjectId};
// use mongodb::results::InsertOneResult;
use log::{error, debug};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Serialize, Deserialize};

#[post("/", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    debug!("Creating user");
    let insert_result = db.create_user(data);
    match insert_result {
        Ok(res) => {
            if let Some(inserted_id) = res.inserted_id.as_object_id() {
                debug!("User created successfully");
                let updated_user_info = db.get_user(&inserted_id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
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
pub fn get_user(
    db: &State<MongoRepo>,
    id: &str,
) -> Result<Json<User>, Status> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let user_detail = db.get_user(&obj_id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/", data= "<user_data>")]
pub fn update_user(
    db: &State<MongoRepo>,
    user_data: Json<User>,
) -> Result<Json<User>, Status> {
    let user_instance: User = user_data.into_inner();
    
    let user_id= user_instance.id.clone();
    debug!("Updating user...");
    let update_result = db.update_user(user_instance);
    match update_result {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                debug!("User updated successfully. Retrieving it back");
                if let Some(ref object_id) = &user_id {
                    let updated_user_info = db.get_user(&object_id);
                    return match updated_user_info {
                        Ok(user) => Ok(Json(user)),
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
pub fn delete_user(
    db: &State<MongoRepo>,
    msg: Json<DeleteMessage>
) -> Result<Json<String>, Status> {
    let msg_json = msg.into_inner();
    debug!("Deleting User with ID {}", msg_json._id);
    let obj_id = ObjectId::parse_str(msg_json._id).unwrap();
    let delete_res = db.delete_user(&obj_id);
    match delete_res {
        Ok(_res) => Ok(Json(String::from("Ok"))),
        Err(e) => {
            error!("Error occurred: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/all")]
pub fn get_all_user(
    db: &State<MongoRepo>,
) -> Result<Json<Vec<User>>, Status> {
    let mut users: Vec<User> = vec![];
    let get_all_res = db.get_all_users();
    
    let cursor = match get_all_res {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("Error occurred: {}", e);
            return Err(Status::InternalServerError)
        },
    };

    for res in cursor {
        match res {
            Ok(user) => {
                users.push(user);
            }
            Err(e) => {
                error!("Error occurred: {}", e);
                return Err(Status::InternalServerError)
            },
        }
    }

    Ok(Json(users))
}

