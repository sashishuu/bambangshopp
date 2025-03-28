pub mod notification;
pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![...])
            .mount("/notification", routes![
                notification::subscribe,
                notification::unsubscribe
            ])
    });
}
