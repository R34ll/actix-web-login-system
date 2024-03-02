use crate::database::{CreateUser, Database, LoginUser, User, UserSession};
use crate::routes::load_html;
use actix_session::Session;
use actix_web::web::Redirect;
use actix_web::{get, post, web, Error, HttpResponse, Responder, Result};
use uuid::Uuid;
use askama::Template;


#[derive(askama::Template)]
#[template(path="login.html")]
struct LoginTemplate{logged:bool, user_infos:UserSession}



#[derive(Template)]
#[template(path="register.html")]
struct RegisterTemplate<'a>{name:&'a str}


#[get("/login")]
pub async fn login_get(session:Session) -> impl Responder {
    // HttpResponse::Ok().body(load_html("account/login.html").unwrap())
    let user_s = UserSession::new(true, "mike".to_string(),"hey".to_string(),"jo".to_string(),"ha".to_string(),"la".to_string());

    session.insert("user", user_s.clone()).unwrap();
    let html = LoginTemplate{logged:true, user_infos:user_s};
    HttpResponse::Ok().body(html.render().unwrap())


}

#[post("/login")]
pub async fn login_post(
    session: Session,
    web::Form(form): web::Form<LoginUser>,
    db: web::Data<Database>,
) -> Result<impl Responder, Error> {
    if let Ok(Some(user)) = db.fetch_user_by_username(form.username).await {
        
        if user.password == form.password {
            session.insert("username", user.username.clone())?;
            return Ok(HttpResponse::Found()
                .insert_header((
                    "Location",
                    format!("/account/profile?username={}", user.username).as_str(),
                ))
                .finish());
        } else {
            return Ok(HttpResponse::Ok().body("Wrong password"));
        }
    }
    // Hnadle the case where no user is found
    Ok(HttpResponse::Ok().body("User not found"))
}

#[get("/register")]
pub async fn register_get() -> impl Responder {
    // HttpResponse::Ok().body(load_html("account/register.html").unwrap())

    let html = RegisterTemplate{name:"Mike"};
    HttpResponse::Ok().body(html.render().unwrap())

}

#[post("/register")]
pub async fn register_post(
    web::Form(form): web::Form<CreateUser>,
    db: web::Data<Database>,
) -> Result<impl Responder, Error> {
    let mut buffer = Uuid::encode_buffer();
    let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

    let new_user = User::new(
        uuid.to_string(),
        form.username.clone(),
        form.email.clone(),
        form.password,
    );

    // Check if username is already used
    if (db.fetch_user_by_username(form.username.clone()).await).is_ok() {
        HttpResponse::Ok().body(format!(
            "Username '{:?}' is already used by another user",
            form.username
        ));
    }


    match db.add_user(&new_user).await {
        Ok(_) => Ok(HttpResponse::Ok().body("User was created. now make login!")),
        Err(e) => {
            Err(actix_web::error::ErrorInternalServerError(format!("{:?}",e)))
        }
      
        //HttpResponse::Ok().body("User was not created!"),
    }
}

#[derive(Debug, serde::Deserialize)]
struct QueryParam {
    username: String,
}
// If client has parameter 'username' is shw the profile of user in parameter
// If client is logged(with session username active) is showed his profile
// if client is not logged and no has username parameters is asked to him log
#[get("/profile")]
pub async fn profile_get(
    session: Session,
    query_params: Result<web::Query<QueryParam>>,
    db: web::Data<Database>,
) -> impl Responder {
    if let Ok(param) = query_params {
        let user = db.fetch_user_by_username(param.username.clone()).await;
        match user {
            Ok(p_user) => {
                match p_user{
                    Some(user) =>{
                        let mut html: String = load_html("account/profile.html").unwrap();
                        html.push_str(format!("{:?}", user.username).as_str());
                        HttpResponse::Ok().body(html)
                    },
                    None => HttpResponse::Ok().body("user not find")
                } 
            }
            Err(_e) => HttpResponse::Ok().body("Error querying user!"),
        }
    } else {
        // Check session
        match check_session(&session) {
            Some(user) => {
                let mut html: String = load_html("account/profile.html").unwrap();
                html.push_str(format!("Welcome: {:?}", user).as_str());
                HttpResponse::Ok().body(html)
            }
            None => HttpResponse::Ok().body(load_html("account/profile.html").unwrap()),
        }
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    Redirect::to("/").permanent()
}

fn check_session(session: &Session) -> Option<String> {
    match session.get::<String>("username") {
        Ok(username) => username,
        Err(_) => None,
    }
}
