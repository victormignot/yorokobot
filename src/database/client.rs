use std::collections::HashSet;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_bson, Bson, Document},
    Client as MongoClient, Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::environment::get_env_variable;
use crate::errors::ClientError;
use crate::DatabaseCredentials;

use super::models::{YorokobotModel, COLLECTIONS_NAMES};

/// Database client
pub struct Client {
    mongo_client: Option<MongoClient>,
    database: Option<Database>,
    credentials: DatabaseCredentials,
}

impl Client {
    /// Create a new database client
    pub fn new(credentials: DatabaseCredentials) -> Client {
        return Client {
            credentials,
            mongo_client: None,
            database: None,
        };
    }

    /// Connect the client
    pub async fn connect(&mut self) -> Result<(), ClientError> {
        self.mongo_client = match MongoClient::with_options(self.credentials.to_owned()) {
            Ok(c) => Some(c),
            Err(e) => return Err(ClientError::Database(e)),
        };

        self.database = Some(
            self.mongo_client
                .as_ref()
                .unwrap()
                .database(get_env_variable("MONGO_DEFAULT_DB").as_str()),
        );

        // TODO:
        // Complete error kind to be more specific.
        // Ex: DatabaseConnection

        self.check_init_error().await;

        Ok(())
    }

    async fn check_init_error(&mut self) {
        self.check_collections_presence().await;
    }

    async fn check_collections_presence(&mut self) {
        let mut missing_collections: Vec<&str> = vec![];
        let collections: HashSet<String> = match self
            .database
            .as_ref()
            .unwrap()
            .list_collection_names(None)
            .await
        {
            Ok(n) => n.into_iter().collect(),
            Err(e) => panic!("Could not list collections: {e}"),
        };

        for col in COLLECTIONS_NAMES {
            if !collections.contains(col) {
                missing_collections.push(col);
            }
        }

        if missing_collections.len() != 0 {
            panic!(
                "Missing the following the following collections: {}",
                missing_collections.join(", ")
            );
        }
    }

    fn get_collection<T: YorokobotModel>(&self) -> Collection<Document> {
        self.database
            .as_ref()
            .expect("Could not retrieve database")
            .collection(&T::get_collection_name())
    }

    fn get_typed_collection<T: YorokobotModel>(&self) -> Collection<T> {
        self.database
            .as_ref()
            .expect("Could not retrieve database")
            .collection::<T>(&T::get_collection_name())
    }

    #[allow(dead_code)]
    pub async fn get_by_id<T: YorokobotModel + for<'de> Deserialize<'de>>(&self, id: &str) -> T {
        self.get_one(doc! {"_id": id}).await
    }

    #[allow(dead_code)]
    pub async fn get_one<T: YorokobotModel + for<'de> Deserialize<'de>>(
        &self,
        filter: Document,
    ) -> T {
        let result = self
            .get_collection::<T>()
            .find_one(filter, None)
            .await
            .expect("Could not issue request")
            .expect("Could not find matching data");

        return from_bson(Bson::Document(result)).expect("Could not deserialize data");
    }

    #[allow(dead_code)]
    pub async fn get_all<T: YorokobotModel + for<'de> Deserialize<'de>>(
        &self,
        filter: Option<Document>,
    ) {
        let mut result: Vec<T> = vec![];

        let mut cursor = match filter {
            Some(f) => self.get_collection::<T>().find(f, None).await,
            None => self.get_collection::<T>().find(doc! {}, None).await,
        }
        .expect("Could not issue request");

        while let Some(document) = cursor.try_next().await.expect("Could not fetch results") {
            result
                .push(from_bson(Bson::Document(document)).expect("Could not deserialize document"));
        }
    }

    #[allow(dead_code)]
    // TODO: Set true error handling
    pub async fn insert_one<T: YorokobotModel + Serialize>(&self, document: T) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .insert_one(document, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    // TODO: Set true error handling
    pub async fn insert_many<T: YorokobotModel + Serialize>(
        &self,
        documents: Vec<T>,
    ) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .insert_many(documents, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    // TODO: Set true error handling
    pub async fn delete_one<T: YorokobotModel>(&self, document: Document) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .delete_one(document, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    // TODO: Set true error handling
    pub async fn delete_by_id<T: YorokobotModel>(&self, id: &str) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .delete_one(doc! {"_id": id}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    // TODO: Set true error handling
    pub async fn delete_many<T: YorokobotModel>(&self, document: Document) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .delete_many(document, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    //TODO: Set true error handling
    pub async fn update_one<T: YorokobotModel>(
        &self,
        document: Document,
        update: Document,
    ) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .update_one(document, update, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    //TODO: Set true error handling
    pub async fn update_by_id<T: YorokobotModel>(
        &self,
        document_id: &str,
        update: Document,
    ) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .update_one(doc! {"_id": document_id}, update, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    #[allow(dead_code)]
    //TODO: Set true error handling
    pub async fn update_many<T: YorokobotModel>(
        &self,
        document: Document,
        update: Document,
    ) -> Result<(), ()> {
        match self
            .get_typed_collection::<T>()
            .update_many(document, update, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
