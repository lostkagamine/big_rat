use log::LevelFilter;

#[macro_use]
extern crate log;

mod window;

fn main() {
    let _logger = pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Trace)
        .init();

    info!("big rat algorithm starting up");
    info!("optimal big-ratification parameters have been found");

    window::make_window();
}
