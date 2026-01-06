use actix_web::http::StatusCode;

pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}
