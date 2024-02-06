use pavex::http::StatusCode;

/// Respond with a `200 OK` status code to indicate that the server is alive
/// and ready to accept new requests.
#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "Server is online"),
    ),
)]
pub fn ping() -> StatusCode {
    StatusCode::OK
}
