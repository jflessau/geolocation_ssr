#[cfg(not(feature = "ssr"))]
pub fn main() {
    use geolocation_ssr::app::App;
    use leptos::*;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {  <App/> }
    });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use geolocation_ssr::{app::App, fileserv::file_and_error_handler};
    use leptos::{get_configuration, view};
    use leptos_axum::{generate_route_list, handle_server_fns, LeptosRoutes};

    // setup logging

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    // configure leptos

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // compose axum router

    let app = Router::new()
        .route("/api/*fn_name", get(handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App /> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // serve

    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
