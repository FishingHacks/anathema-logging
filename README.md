# Simple logging system for Anathema

This repository contains a very basic logging system for Anathema.


## Setup

The setup is rather easy. You have to register a logging component using `logging::register_logger`, `logging::register_logger_with_custom_name`, `logging::register_custom_logger` or `logging::register_custom_logger_simple`, as otherwise the messages won't be saved. The logging functions will **not** return an error when called before the logger was initialised but will instead throw away any messages.

- `register_logger` expects you to provide a mutable reference to the `RuntimeBuilder` and will register a logging viewer component with the default `log-viewer` component name.
- `register_logger_with_custom_name` will do the same thing except with the specified component name.
- `register_custom_logger` expects you to provide a ComponentId for a logging component. This component will have to handle all incoming messages itself. For this, your component needs to be able to receive messages of type `LogEntry` (see [struct definition](#struct-definitions) below)
- `register_custom_logger` expects you to provide a ComponentId for a logging component. This component will have to handle all incoming messages itself. For this, your component needs to be able to receive messages of type `String`

You also have to add the component using `@componentname` to your AML-template. Do this in a spot with space, because overlap between the log viewer and another component will probably cause issues.

Now you can use the `logging::Logger` struct or the global functions to log values. Note: For convenience, I am exporting the macros `send`, `info`, `warn` and `error` from the `logging` module. These will accept additional arguments in a `println!`-type of manner

### Struct Definitions

```rs
#[derive(Debug)]
pub struct LogEntry {
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

// LogEntry but in a form that you can use it as a part of your state
#[derive(State)]
pub struct StateLogEntry {
    pub level: Value<LogLevel>,
    pub msg: Value<String>,
    pub sender: Value<&'static str>,
}
```


### NOTE

This package is currently broken as there is - as of right now - no way to ship components with anathema.