use std::{cell::Cell, io::Write};

use anathema::prelude::*;
use logging::{register_logger, LogLevel, Logger};
use test_component::{MyComponent, MyState};

pub(crate) mod logging;
pub(crate) mod test_component;

static TEMPLATE: &str = include_str!("./template.aml");

thread_local!(static LOGGER: Cell<Option<Logger>> = Cell::new(None));

pub fn get_logger() -> Logger {
    LOGGER.with(|v| unsafe { (v.as_ptr().as_ref()).unwrap().clone() }).expect("Logger uninitialised")
}

fn main() {
    let doc = Document::new(TEMPLATE);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .enable_mouse()
        .hide_cursor()
        .finish()
        .expect("not failing");

    let mut runtime_builder = Runtime::builder(doc, backend);

    let logger = register_logger(&mut runtime_builder).expect("Failed to register the logger");
    LOGGER.set(Some(logger.clone()));


    runtime_builder
        .register_prototype(
            "my-comp-1",
            "src/example_widget.aml",
            || MyComponent { logger: None, log_level: LogLevel::Info, name: "my-comp-1" },
            MyState::default,
        )
        .expect("Failed to register `my-comp-1` component");

    runtime_builder
        .register_prototype(
            "my-comp-2",
            "src/example_widget.aml",
            || MyComponent { logger: None, log_level: LogLevel::Warn, name: "my-comp-2" },
            MyState::default,
        )
        .expect("Failed to register `my-comp-2` component");

    runtime_builder
        .register_prototype(
            "my-comp-3",
            "src/example_widget.aml",
            || MyComponent { logger: None, log_level: LogLevel::Err, name: "my-comp-3" },
            MyState::default,
        )
        .expect("Failed to register `my-comp-3` component");

    runtime_builder.finish().unwrap().run();
}