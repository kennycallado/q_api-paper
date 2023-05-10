use crate::app::modules::papers::controller::routes as paper_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/paper", paper_routes())
    })
}
