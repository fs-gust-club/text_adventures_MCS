extern crate simplelog;

use simplelog::*;
use std::fs::File;
use text_adventure::start;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("mylog.log").unwrap(),
        ),
    ])
    .unwrap();
    start();
}