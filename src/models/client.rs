use bson::doc;
use bson::Document;
use mongodb::error::Error;
use mongodb::options::IndexOptions;
use mongodb::results::DeleteResult;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::Database;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

// Company details for Admin
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Client {
    pub email: String,
    pub hash_password: String,
}

impl Client {
    pub fn new(email: String, hash_password: String) -> Client {
        let email = email.to_lowercase();
        Client {
            email,
            hash_password,
        }
    }

    ///this ensures that only valid data can be inserted into this document
    pub async fn insert_one(data: &Client, db: &Database) -> Result<InsertOneResult, Error> {
        let client_collection = db.collection::<Client>("Client");
        client_collection.insert_one(data, None).await
    }

    pub async fn find_one(t: Document, db: &Database) -> Result<Option<Client>, Error> {
        let client_collection = db.collection::<Client>("Client");
        client_collection.find_one(t, None).await
    }

    pub async fn update_one(
        filter: Document,
        update: Document,
        db: &Database,
    ) -> Result<UpdateResult, Error> {
        let client_collection = db.collection::<Client>("Client");
        client_collection.update_one(filter, update, None).await
    }
    pub async fn create_index(db: &Database) {
        let mut options = IndexOptions::builder().unique(true).build();
        options.sparse = Some(true);
        let model = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(options)
            .build();
        db.collection::<Client>("Client")
            .create_index(model, None)
            .await
            .expect("error creating index! Client");
    }
    pub async fn replace_one(
        filter: Document,
        update: &Client,
        db: &Database,
    ) -> Result<UpdateResult, Error> {
        let setting_collection = db.collection::<Client>("Client");
        setting_collection.replace_one(filter, update, None).await
    }
    pub async fn delete_one(filter: Document, db: &Database) -> Result<DeleteResult, Error> {
        let client_collection = db.collection::<Client>("Client");
        client_collection.delete_one(filter, None).await
    }
}
