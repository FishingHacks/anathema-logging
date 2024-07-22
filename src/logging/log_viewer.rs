use anathema::{component::*, geometry::Pos, runtime::RuntimeBuilder};

use super::{
    log_state::{LogFilter, LogViewerState, StateLogEntry},
    LogLevel,
};

#[derive(Debug)]
struct LogEntry {
    level: LogLevel,
    msg: String,
    sender: &'static str,
}

impl Into<StateLogEntry> for LogEntry {
    fn into(self) -> StateLogEntry {
        StateLogEntry {
            level: self.level.into(),
            msg: self.msg.into(),
            sender: self.sender.into(),
        }
    }
}

#[derive(Clone)]
enum LogViewerAction {
    OpenState(bool),
    SetFilter(LogFilter),
}

impl LogViewerAction {
    fn get(elements: &mut Elements<'_, '_>, pos: Pos) -> Option<Self> {
        let mut value: Option<Self> = None;
        elements.query().at_position(pos).each(|_, attribs| {
            if value.is_some() {
                return;
            }
            let Some(val) = attribs.get_val("click_action") else {
                return;
            };
            val.str_for_each(|v| {
                if value.is_some() {
                    return;
                }
                value = Self::from_str(v);
            });
        });

        value
    }

    fn from_str(str: &str) -> Option<Self> {
        Some(match str {
            "open_log" => Self::OpenState(true),
            "close" => Self::OpenState(false),
            "filter-level-set-warn" => Self::SetFilter(LogFilter::Level(LogLevel::Warn)),
            "filter-level-set-err" => Self::SetFilter(LogFilter::Level(LogLevel::Err)),
            "filter-level-set-info" => Self::SetFilter(LogFilter::Level(LogLevel::Info)),
            "filter-reset" => Self::SetFilter(LogFilter::None),
            _ => return None,
        })
    }
}

struct LogViewer;

impl Component for LogViewer {
    type State = LogViewerState;

    type Message = LogEntry;

    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: anathema::prelude::Context<'_>,
    ) {
        if !mouse.lsb_up() {
            return;
        }
        let Some(action) = LogViewerAction::get(&mut elements, mouse.pos()) else {
            return;
        };

        match action {
            LogViewerAction::OpenState(new_state) => state.is_open.set(new_state),
            LogViewerAction::SetFilter(new_filter) => state.filter.set(new_filter.into()),
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: anathema::prelude::Context<'_>,
    ) {
        state.log_entries.push(Value::new(message.into()));
        state.length.set(state.log_entries.len());
    }
}

pub fn register_logger<T>(builder: &mut RuntimeBuilder<T>) -> anathema::runtime::Result<Logger> {
    let component = builder.register_component(
        "log-viewer",
        "src/log_viewer.aml",
        LogViewer,
        LogViewerState::default(),
    )?;
    Ok(Logger {
        component,
        emitter: builder.emitter(),
    })
}

#[derive(Clone)]
pub struct Logger {
    emitter: Emitter,
    component: ComponentId<LogEntry>,
}

impl Logger {
    pub fn send(
        &self,
        level: LogLevel,
        sender: &'static str,
        message: impl Into<String>,
    ) -> Result<(), ()> {
        self.emitter
            .emit(
                self.component,
                LogEntry {
                    level,
                    msg: message.into(),
                    sender,
                },
            )
            .map_err(|_| ())
    }

    pub fn info(&self, sender: &'static str, message: impl Into<String>) -> Result<(), ()> {
        self.send(LogLevel::Info, sender, message)
    }

    pub fn warn(&self, sender: &'static str, message: impl Into<String>) -> Result<(), ()> {
        self.send(LogLevel::Warn, sender, message)
    }

    pub fn error(&self, sender: &'static str, message: impl Into<String>) -> Result<(), ()> {
        self.send(LogLevel::Err, sender, message)
    }
}
