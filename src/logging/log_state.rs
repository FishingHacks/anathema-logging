use std::fmt::Debug;

use anathema::{
    component::*,
    state::{CommonVal, Path, PendingValue},
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum LogLevel {
    Info,
    Warn,
    Err,
}

unsafe impl Send for LogLevel {}
unsafe impl Sync for LogLevel {}

impl LogLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Err => "ERR",
        }
    }
}

impl State for LogLevel {
    fn to_common(&self) -> Option<CommonVal<'_>> {
        return Some(CommonVal::Str(self.to_string()));
    }
}

#[derive(Clone)]
pub enum LogFilter {
    None,
    Level(LogLevel),
}

impl Into<LogFilterState> for LogFilter {
    fn into(self) -> LogFilterState {
        match self {
            Self::None => LogFilterState::None,
            Self::Level(v) => LogFilterState::Level(v.into()),
        }
    }
}

pub enum LogFilterState {
    None,
    Level(Value<LogLevel>),
}

impl Debug for LogFilterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Level(level) => f.write_fmt(format_args!("LogLevel {:?}", *level.to_ref())),
        }
    }
}

unsafe impl Send for LogFilterState {}
unsafe impl Sync for LogFilterState {}

impl State for LogFilterState {
    fn to_bool(&self) -> bool {
        !matches!(self, Self::None)
    }

    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            Self::None => Some("none".into()),
            Self::Level(..) => Some("level".into()),
        }
    }

    fn state_lookup(&self, path: Path<'_>) -> Option<PendingValue> {
        let Path::Key(k) = path else { return None; };
        if k != "value" { return None; }

        match self {
            Self::None => None,
            Self::Level(value) => Some(value.to_pending()),
        }
    }

    fn state_get(&self, path: Path<'_>, sub: anathema::state::Subscriber) -> Option<anathema::state::ValueRef> {
        let Path::Key(k) = path else { return None; };
        if k != "value" { return None; }

        match self {
            Self::None => None,
            Self::Level(value) => Some(value.value_ref(sub)),
        }
    }
}

#[derive(State, Debug)]
pub struct StateLogEntry {
    pub level: Value<LogLevel>,
    pub msg: Value<String>,
    pub sender: Value<&'static str>,
}

#[derive(State)]
pub struct LogViewerState {
    pub is_open: Value<bool>,
    pub log_entries: Value<List<StateLogEntry>>,
    pub length: Value<usize>,
    pub filter: Value<LogFilterState>,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            is_open: Default::default(),
            log_entries: List::empty(),
            length: Default::default(),
            filter: LogFilterState::None.into(),
        }
    }
}
