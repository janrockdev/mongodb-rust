use bson::Document;
use mongodb::sync::Client;
use mongodb::sync::Collection;
use mongodb::bson::{doc, Bson};
use serde::Deserialize;
use serde::Serialize;

struct UsersManager {
    coll: Collection<Document>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<bson::oid::ObjectId>,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userPassword")]
    pub user_password: String,
    pub created: bson::DateTime,
}

fn main() {
    let conn_string = std::env::var_os("MONGODB_URL").expect("missing environment variable MONGODB_URL").to_str().expect("missing MONGODB_URL").to_owned();

    let users_db = std::env::var_os("MONGODB_DATABASE").expect("missing environment variable MONGODB_DATABASE").to_str().expect("missing MONGODB_DATABASE").to_owned();

    let users_collection = std::env::var_os("MONGODB_COLLECTION").expect("missing environment variable MONGODB_COLLECTION").to_str().expect("missing MONGODB_COLLECTION").to_owned();

    let um = UsersManager::new(conn_string,users_db.as_str(), users_collection.as_str());

    let ops: Vec<String> = std::env::args().collect();
    let operation = ops[1].as_str();

    match operation {
        "create" => um.add_user(ops[2].as_str()),
        "list" => um.list_users(ops[2].as_str()),
        "update" => um.update_user(ops[2].as_str(), ops[3].as_str()),
        "delete" => um.delete_user(ops[2].as_str()),
        _ => panic!("invalid user operation specified")
    }
}

impl UsersManager{
    fn new(conn_string: String, db_name: &str, coll_name: &str) -> Self{
        let mongo_client = Client::with_uri_str(&*conn_string).expect("failed to create client");
        println!("successfully connected to mongodb");

        let users_coll: Collection<Document> = mongo_client.database(db_name).collection(coll_name);    
        UsersManager{coll: users_coll}
    }

    fn add_user(&self, email: &str) {
        let new_user = User {
            user_id: None,
            user_email: String::from(email),
            user_name: String::from("test_user"),
            user_password: String::from("test_password"),
            created: bson::DateTime::now(),
        };

        let user_doc = bson::to_document(&new_user).expect("conversion failed");
        
        let insert_result = self.coll.insert_one(user_doc, None).expect("failed to add user");    
        println!("inserted user with id = {}", insert_result.inserted_id);
    }
    
    fn list_users(self, status_filter: &str) {

        let mut filter = doc!{};
        if status_filter == "enabled" ||  status_filter == "disabled"{
            println!("listing '{}' users",status_filter);
            filter = doc!{"status": status_filter}
        } else if status_filter != "all" {
            panic!("invalid user status")
        }    
        let mut users = self.coll.find(filter, None).expect("failed to find users");
    
        while let Some(result) = users.next() {
            let user_doc = result.expect("user not present");
    
            let user: User = bson::from_bson(Bson::Document(user_doc)).expect("conversion failed");
    
            println!("user_id: {} \nuser_name: {} \nuser_password: {} \nuser_email: {} \ncreated: {} \n===========", user.user_id.expect("user id missing"), user.user_name, user.user_password, user.user_email, user.created);
        }
    }

    fn update_user(self, user_id: &str, status: &str) {

        if status != "enabled" && status != "disabled" {
            panic!("invalid user status")
        }
    
        println!("updating user {} status to {}", user_id, status);
    
        let id_filter = doc! {"_id": bson::oid::ObjectId::parse_str(user_id).expect("user_id is not valid ObjectID")};
    
        let r = self.coll.update_one(id_filter, doc! {"$set": { "status": status }}, None).expect("user update failed");
    
        if r.modified_count == 1 {
            println!("updated status for user id {}",user_id);
        } else if r.matched_count == 0 {
            println!("could not update. check user id {}",user_id);
        }
    }

    fn delete_user(self, user_id: &str) {
        let id_filter = doc! {"_id": bson::oid::ObjectId::parse_str(user_id).expect("user_id is not valid ObjectID")};
        self.coll.delete_one(id_filter, None).expect("delete failed").deleted_count;
        
        println!("deleted user {}", user_id);
    }

}