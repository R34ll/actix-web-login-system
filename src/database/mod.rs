mod models;
pub use models::{CreateUser, LoginUser, User, UserSession};

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client
            .use_ns("surreal")
            .use_db("login_system")
            .await?;

        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("login_system"),
        })
    }

    pub async fn add_user(&self, new_user: &User) -> Result<User, Error> {
    
        let existing_user = self.fetch_user_by_username(new_user.username.clone()).await;

        if existing_user.is_ok(){
            // return Err(Error::DuplicateEntry);
            todo!()
        }

        let created_user = self
            .client
            .create(("user",new_user.uuid.clone()))
            .content(new_user)
            .await?;

        match created_user {
            Some(created) => Ok(created),
            None => todo!()
        }
    }

    pub async fn fetch_user_by_username(&self, username: String) -> Result<Option<User>, Error> {

        match self
            .client
            .query(format!("select * from user where username='{}'",username))
            .await
            {
                Ok(mut users) => users.take(0),
                Err(e) => Err(e.into())
        }
    }
}
