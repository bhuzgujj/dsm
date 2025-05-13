use log::{info, warn};

fn main() {
    let sets = interfaces::models::Settings::load();
    interfaces::logger::bind_logger(&sets).unwrap();
    info!("LMAO");
    warn!("LOLLL");
    sets.save().unwrap();
}
