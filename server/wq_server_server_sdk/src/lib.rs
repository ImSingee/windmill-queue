//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
extern crate alloc;
struct ServerState {
    router: pavex_matchit::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: tokio::sync::mpsc::Sender<wq_server::app::Task>,
    s1: apalis_sql::postgres::PostgresStorage<wq_server::app::queue::NewEvents>,
}
pub async fn build_application_state(
    v0: wq_server::configuration::Config,
) -> crate::ApplicationState {
    let v1 = wq_server::app::queue::new(v0).await;
    let v2 = wq_server::app::App::pavex_task_sender();
    crate::ApplicationState {
        s0: v2,
        s1: v1,
    }
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> pavex_matchit::Router<u32> {
    let mut router = pavex_matchit::Router::new();
    router.insert("/", 0u32).unwrap();
    router.insert("/api/ingest", 1u32).unwrap();
    router.insert("/api/ping", 2u32).unwrap();
    router.insert("/demo", 3u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.target.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "*",
            );
            return route_4::middleware_0(
                    &allowed_methods,
                    matched_route_template,
                    &request_head,
                )
                .await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::path::RawPathParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_0::middleware_0(matched_route_template, &request_head).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_4::middleware_0(
                            &allowed_methods,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
            }
        }
        1u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/ingest",
            );
            match &request_head.method {
                &pavex::http::Method::POST => {
                    route_3::middleware_0(
                            matched_route_template,
                            server_state.application_state.s1.clone(),
                            request_body,
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::POST,
                        ])
                        .into();
                    route_4::middleware_0(
                            &allowed_methods,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
            }
        }
        2u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/ping",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_1::middleware_0(matched_route_template, &request_head).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_4::middleware_0(
                            &allowed_methods,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
            }
        }
        3u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/demo",
            );
            match &request_head.method {
                &pavex::http::Method::POST => {
                    route_2::middleware_0(
                            matched_route_template,
                            server_state.application_state.s0.clone(),
                            request_body,
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::POST,
                        ])
                        .into();
                    route_4::middleware_0(
                            &allowed_methods,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn middleware_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = wq_server::telemetry::RootSpan::new(v1, v0);
        let v3 = crate::route_0::Next0 {
            next: handler,
        };
        let v4 = pavex::middleware::Next::new(v3);
        wq_server::telemetry::logger(v4, v2).await
    }
    pub async fn handler() -> pavex::response::Response {
        let v0 = wq_server::routes::root();
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v0)
    }
    pub struct Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        next: fn() -> T,
    }
    impl<T> std::future::IntoFuture for Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)()
        }
    }
}
pub mod route_1 {
    pub async fn middleware_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = wq_server::telemetry::RootSpan::new(v1, v0);
        let v3 = crate::route_1::Next0 {
            next: handler,
        };
        let v4 = pavex::middleware::Next::new(v3);
        wq_server::telemetry::logger(v4, v2).await
    }
    pub async fn handler() -> pavex::response::Response {
        let v0 = wq_server::routes::status::ping();
        <http::StatusCode as pavex::response::IntoResponse>::into_response(v0)
    }
    pub struct Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        next: fn() -> T,
    }
    impl<T> std::future::IntoFuture for Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)()
        }
    }
}
pub mod route_2 {
    pub async fn middleware_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: tokio::sync::mpsc::Sender<wq_server::app::Task>,
        v2: pavex::request::body::RawIncomingBody,
        v3: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v4 = wq_server::telemetry::RootSpan::new(v3, v0);
        let v5 = <wq_server::telemetry::RootSpan as core::clone::Clone>::clone(&v4);
        let v6 = crate::route_2::Next0 {
            s_0: v1,
            s_1: v2,
            s_2: v4,
            s_3: v3,
            next: handler,
        };
        let v7 = pavex::middleware::Next::new(v6);
        wq_server::telemetry::logger(v7, v5).await
    }
    pub async fn handler(
        v0: tokio::sync::mpsc::Sender<wq_server::app::Task>,
        v1: pavex::request::body::RawIncomingBody,
        v2: wq_server::telemetry::RootSpan,
        v3: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v4 = <pavex::request::body::BodySizeLimit as std::default::Default>::default();
        let v5 = pavex::request::body::BufferedBody::extract(v3, v1, v4).await;
        let v6 = match v5 {
            Ok(ok) => ok,
            Err(v6) => {
                return {
                    let v7 = pavex::request::body::errors::ExtractBufferedBodyError::into_response(
                        &v6,
                    );
                    let v8 = pavex::Error::new(v6);
                    wq_server::telemetry::log_error(&v8, v2).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v7,
                    )
                };
            }
        };
        let v7 = pavex::request::body::JsonBody::extract(v3, &v6);
        let v8 = match v7 {
            Ok(ok) => ok,
            Err(v8) => {
                return {
                    let v9 = pavex::request::body::errors::ExtractJsonBodyError::into_response(
                        &v8,
                    );
                    let v10 = pavex::Error::new(v8);
                    wq_server::telemetry::log_error(&v10, v2).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v9,
                    )
                };
            }
        };
        let v9 = wq_server::routes::demo::new_demo_task(v8, v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v9)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: tokio::sync::mpsc::Sender<wq_server::app::Task>,
        s_1: pavex::request::body::RawIncomingBody,
        s_2: wq_server::telemetry::RootSpan,
        s_3: &'a pavex::request::RequestHead,
        next: fn(
            tokio::sync::mpsc::Sender<wq_server::app::Task>,
            pavex::request::body::RawIncomingBody,
            wq_server::telemetry::RootSpan,
            &'a pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2, self.s_3)
        }
    }
}
pub mod route_3 {
    pub async fn middleware_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: apalis_sql::postgres::PostgresStorage<wq_server::app::queue::NewEvents>,
        v2: pavex::request::body::RawIncomingBody,
        v3: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v4 = wq_server::telemetry::RootSpan::new(v3, v0);
        let v5 = <wq_server::telemetry::RootSpan as core::clone::Clone>::clone(&v4);
        let v6 = crate::route_3::Next0 {
            s_0: v1,
            s_1: v2,
            s_2: v4,
            s_3: v3,
            next: handler,
        };
        let v7 = pavex::middleware::Next::new(v6);
        wq_server::telemetry::logger(v7, v5).await
    }
    pub async fn handler(
        v0: apalis_sql::postgres::PostgresStorage<wq_server::app::queue::NewEvents>,
        v1: pavex::request::body::RawIncomingBody,
        v2: wq_server::telemetry::RootSpan,
        v3: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v4 = <pavex::request::body::BodySizeLimit as std::default::Default>::default();
        let v5 = pavex::request::body::BufferedBody::extract(v3, v1, v4).await;
        let v6 = match v5 {
            Ok(ok) => ok,
            Err(v6) => {
                return {
                    let v7 = pavex::request::body::errors::ExtractBufferedBodyError::into_response(
                        &v6,
                    );
                    let v8 = pavex::Error::new(v6);
                    wq_server::telemetry::log_error(&v8, v2).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v7,
                    )
                };
            }
        };
        let v7 = pavex::request::body::JsonBody::extract(v3, &v6);
        let v8 = match v7 {
            Ok(ok) => ok,
            Err(v8) => {
                return {
                    let v9 = pavex::request::body::errors::ExtractJsonBodyError::into_response(
                        &v8,
                    );
                    let v10 = pavex::Error::new(v8);
                    wq_server::telemetry::log_error(&v10, v2).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v9,
                    )
                };
            }
        };
        let v9 = wq_server::routes::events::ingest_events(v8, v0).await;
        let v10 = match v9 {
            Ok(ok) => ok,
            Err(v10) => {
                return {
                    let v11 = wq_server::utils::error::error_handler(&v10);
                    let v12 = pavex::Error::new(v10);
                    wq_server::telemetry::log_error(&v12, v2).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v11,
                    )
                };
            }
        };
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v10)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: apalis_sql::postgres::PostgresStorage<wq_server::app::queue::NewEvents>,
        s_1: pavex::request::body::RawIncomingBody,
        s_2: wq_server::telemetry::RootSpan,
        s_3: &'a pavex::request::RequestHead,
        next: fn(
            apalis_sql::postgres::PostgresStorage<wq_server::app::queue::NewEvents>,
            pavex::request::body::RawIncomingBody,
            wq_server::telemetry::RootSpan,
            &'a pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2, self.s_3)
        }
    }
}
pub mod route_4 {
    pub async fn middleware_0(
        v0: &pavex::router::AllowedMethods,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = wq_server::telemetry::RootSpan::new(v2, v1);
        let v4 = crate::route_4::Next0 {
            s_0: v0,
            next: handler,
        };
        let v5 = pavex::middleware::Next::new(v4);
        wq_server::telemetry::logger(v5, v3).await
    }
    pub async fn handler(
        v0: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        next: fn(&'a pavex::router::AllowedMethods) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
