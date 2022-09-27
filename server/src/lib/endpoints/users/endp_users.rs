use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

// The UserDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UserDataBody { user_name: String, email: String }

// The GET /user/<user_hash> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
//  a valid auth token is required.
#[actix_web::get("/user/{user_hash}")]
pub async fn get_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>
) -> impl Responder {
    // Get the access token from the request headers. 
    // This tokens is used to make sure that the incoming 
    // request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { return "{}".to_string()}

    // Once the request has been verified, query the
    // database for the provided user_hash. Once found,
    // return all the data from said user.
    let user = db.query_user_by_hash(&user_hash).await;
    // Check whether or not the user is invalid
    if user.is_none() { return "{}".to_string() }
    // Else, if the user is valid, unwrap the
    // object so it can be read
    let user = user.unwrap();

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"access_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"classes\": {}}}", 
            access_token, user_hash, user.name, "array of the users class_hashes (select from classes where user_hash = user_hash)"
    )
}

// The POST /user/{user_hash} endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[actix_web::post("/user/{user_hash}")]
pub async fn update_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>, body: web::Json<UserDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");
    let firebase_token: &str = global::get_header(&req, "Google Auth Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    if !lib::auth::verify_bearer(&user_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    // If the incoming request doesn't contain
    // a new user_name, return an empty json map
    if body.user_name.len() < 1 { return "{}".to_string() }

    // Else, update the users 'user_name' in the database
    let r: u64 = db.update_user_name(&user_hash, &body.user_name).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}

// The insert_user_data() function is used to insert
// a new row into the users column within the database
// containing the users unique hash, provided name,
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website
// using firebase google auth.
#[actix_web::put("/user/{user_hash}")]
async fn insert_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>, body: web::Json<UserDataBody>
) -> String {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");
    let firebase_token: &str = global::get_header(&req, "Google Auth Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    if !lib::auth::verify_bearer(&user_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    // Get the current system time. This is used
    // for inserting the users registration date
    // into the database.
    let time: u64 = global::get_time();
    
    // Insert the user into the database
    // Along with this insertion is the user_hash, user_name
    // user's email and the time of registration
    let r: u64 = db.insert_user(
        &user_hash, &body.user_name, 
        &body.email, time as i64
    ).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}
