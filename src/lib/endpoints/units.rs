use crate::lib::{
    self, global, handlers::Database, structs::UnitDataBody
};
use actix_web::{web, HttpRequest, Responder};

// The insert_class_unit() endpoint is used to create
// a new unit for the provided class. Using the
// provided class_id the function will generate
// a unique unit identifier using the following format:
// SHA256(class_id:current_time)
#[actix_web::put("/class/{class_id}/units/")]
async fn insert_class_unit(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Generate a new unit id
    let unit_id: String = global::generate_new_id(&class_id);

    // Insert the unit data into the database
    return match db.insert_class_unit(&bearer, &unit_id, &class_id, &body.unit_name).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Unit created successfully",
            "unit_id": unit_id
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to create unit"
        }).to_string()
    }
}

// The delete_class_unit() function is used to
// delete the provided unit from the database.
#[actix_web::delete("/class/{class_id}/units/{unit_id}")]
async fn delete_class_unit(
    req: HttpRequest, db: web::Data<Database>
) -> impl Responder {

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the unit id
    let unit_id: &str = match req.match_info().get("unit_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Insert the unit data into the database
    return match db.delete_class_unit(&bearer, class_id, unit_id).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Unit deleted successfully"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to delete unit"
        }).to_string()
    }
}

// The update_class_unit() endpoint is used to
// modify any data within the unit's database row.
#[actix_web::post("/class/{class_id}/units/{unit_id}")]
async fn update_class_unit(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UnitDataBody>
) -> impl Responder {

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the class id
    let unit_id: &str = match req.match_info().get("unit_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Update the unit data in the database
    return match db.update_class_unit(&bearer, class_id, unit_id, &body).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Unit updated successfully"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to update unit"
        }).to_string()
    }
}
