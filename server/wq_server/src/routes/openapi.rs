use crate::app;
use crate::routes;
use crate::utils::error::{HTTPError, HTTPResult};
use crate::utils::error_code::ErrorCode;
use crate::utils::pavex::json_response;
use pavex::request::path::PathParams;
use pavex::response::body::raw::{Bytes, Full};
use pavex::response::Response;
use std::sync::Arc;
use utoipa::openapi::schema::AdditionalProperties;
use utoipa::openapi::{Object, RefOr, Schema};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Windmill Events Queue", license(name = "MIT")),
    paths(routes::status::ping, routes::events::ingest_events),
    components(schemas(
        routes::events::IngestRequest,
        routes::events::IngestResponseSuccess,
        routes::events::IngestRequestEventWithMeta,
        routes::events::IngestRequestEventMeta,
        app::queue_events::EventsMeta,
    ))
)]
struct ApiDoc;

pub fn openapi_handler() -> Response {
    let mut doc = ApiDoc::openapi();

    let components = doc.components.as_mut().unwrap();

    // let mut components = doc.components.unwrap();

    // Object::default().additional_properties

    components.schemas.insert(
        "Event".to_string(),
        RefOr::T(Schema::Object({
            let mut obj = Object::default();
            obj.additional_properties = Some(Box::new(AdditionalProperties::FreeForm(true)));
            obj
        })),
    );

    json_response(doc)
}

pub fn swagger_ui_handler_root_redirect() -> Response {
    Response::permanent_redirect()
        .insert_header("Location".parse().unwrap(), "/swagger/".parse().unwrap())
}

pub fn swagger_ui_handler_root() -> HTTPResult<Response> {
    swagger_ui_handler("".to_string())
}
#[PathParams]
pub struct SwaggerUICatchAllParams {
    pub path: String,
}
pub fn swagger_ui_handler_catch_all(
    params: PathParams<SwaggerUICatchAllParams>,
) -> HTTPResult<Response> {
    let path = params.0.path;

    swagger_ui_handler(path)
}

fn swagger_ui_handler(path: String) -> HTTPResult<Response> {
    let config = utoipa_swagger_ui::Config::from("/openapi.json");
    match utoipa_swagger_ui::serve(&path, Arc::new(config)) {
        Ok(swagger_file) => swagger_file
            .map(|file| {
                let mut response = Response::ok();

                response
                    .headers_mut()
                    .insert("Content-Type", file.content_type.parse().unwrap());

                let raw_body: Full<Bytes> = Full::new(file.bytes.to_vec().into());
                let response = response.set_raw_body(raw_body);

                Ok(response)
            })
            .unwrap_or_else(|| Ok(Response::not_found())),
        Err(err) => {
            let err = anyhow::anyhow!("swagger error: {}", err);

            Err(HTTPError::internal_server_error(
                ErrorCode::E50002,
                "Failed to serve Swagger UI",
                err,
            ))
        }
    }
}
