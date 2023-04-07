use super::log::{info, Level};

use super::app;
use super::simple_logger;

fn running_logger() {
    info("This log is not going to be displayed");

    simple_logger::init(Level::Info);

    app::run();
}
