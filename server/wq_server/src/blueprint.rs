use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::{constructor::Lifecycle, router::{GET, POST}, Blueprint};
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);

    add_telemetry_middleware(&mut bp);

    bp.constructor(f!(crate::app::App::pavex_task_sender), Lifecycle::Singleton);
    bp.constructor(f!(crate::app::queue::new), Lifecycle::Singleton);

    bp.route(GET, "/", f!(crate::routes::root));
    bp.route(GET, "/api/ping", f!(crate::routes::status::ping));
    bp.route(POST, "/demo", f!(crate::routes::demo::new_demo_task));
    bp.route(POST, "/api/events/:queue", f!(crate::routes::events::ingest_events));
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
}
