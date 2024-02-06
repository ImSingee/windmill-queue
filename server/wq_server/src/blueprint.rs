use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::router::ANY;
use pavex::blueprint::{
    constructor::Lifecycle,
    router::{GET, POST},
    Blueprint,
};
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);

    add_telemetry_middleware(&mut bp);

    bp.constructor(f!(crate::configuration::Config::new_pavex), Lifecycle::Singleton).cloning(CloningStrategy::CloneIfNecessary);
    bp.constructor(
        f!(crate::app::db::Connection::new_pavex),
        Lifecycle::Singleton,
    ).cloning(CloningStrategy::CloneIfNecessary);
    bp.constructor(f!(crate::app::queue::new), Lifecycle::Singleton);

    bp.route(GET, "/", f!(crate::routes::root));
    bp.route(
        GET,
        "/openapi.json",
        f!(crate::routes::openapi::openapi_handler),
    );
    bp.route(
        ANY,
        "/swagger",
        f!(crate::routes::openapi::swagger_ui_handler_root_redirect),
    );
    bp.route(
        ANY,
        "/swagger/",
        f!(crate::routes::openapi::swagger_ui_handler_root),
    )
    .error_handler(f!(crate::utils::error::error_handler));
    bp.route(
        ANY,
        "/swagger/*path",
        f!(crate::routes::openapi::swagger_ui_handler_catch_all),
    )
    .error_handler(f!(crate::utils::error::error_handler));
    bp.route(GET, "/api/ping", f!(crate::routes::status::ping));
    bp.route(
        POST,
        "/api/ingest",
        f!(crate::routes::events::ingest_events),
    )
    .error_handler(f!(crate::utils::error::error_handler));
    bp
}

/// Add the telemetry middleware, as well as the constructors of its dependencies.
fn add_telemetry_middleware(bp: &mut Blueprint) {
    bp.constructor(
        f!(crate::telemetry::RootSpan::new),
        Lifecycle::RequestScoped,
    )
    .cloning(CloningStrategy::CloneIfNecessary);

    bp.wrap(f!(crate::telemetry::logger));
    bp.error_observer(f!(crate::telemetry::log_error));
}
