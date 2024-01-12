use crate::error_template::{AppError, ErrorTemplate};
use leptos::{leptos_dom::helpers::IntervalHandle, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_geolocation, use_window, UseGeolocationReturn};
use std::time::Duration;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="geolocation_ssr"/>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=LocateUser />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn LocateUser() -> impl IntoView {
    let (location, set_location) = create_signal(None as Option<(f64, f64)>);

    create_effect(move |prev_handle: Option<IntervalHandle>| {
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        log::info!("run effect");

        let locate = move || {
            let window = use_window();
            log::info!("window is some: {:?}", window.is_some()); // logs "window is some: false" to browser console

            log::info!("run locate");
            let UseGeolocationReturn { coords, error, .. } = use_geolocation();
            if let Some(coords) = coords.get() {
                log::info!(
                    "lat: {:?}, long: {:?}",
                    coords.latitude(),
                    coords.longitude()
                );
                set_location(Some((coords.latitude(), coords.longitude())));
            } else {
                log::info!("no coords, error: {:?}", error.get()); // logs "no coords, error: None" to browser console
            };
        };

        set_interval_with_handle(locate, Duration::from_millis(500))
            .expect("could not create interval")
    });

    view! {
        <h1>User location</h1>
        <p>{move || format!("location: {:?}", location.get())}</p>
    }
}
