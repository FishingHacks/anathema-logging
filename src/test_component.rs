use anathema::component::*;

use crate::{get_logger, logging::{LogLevel, Logger}};

pub struct MyComponent {
    pub logger: Option<Logger>,
    pub name: &'static str,
    pub log_level: LogLevel,
}

#[derive(State, Default)]
pub struct MyState {
    pub is_selected: Value<bool>,
}

impl Component for MyComponent {
    type State = MyState;
    type Message = ();
    
    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
    ) {
        if !mouse.lsb_up() {
            return;
        }
        let query = elements.query().at_position(mouse.pos());

        let mut should_change = false;
        query.first(|_, _| {
            should_change = true;
        });
        if should_change {
            state.is_selected.set(!state.is_selected.to_bool());
            let logger = match self.logger {
                None => {
                    self.logger = Some(get_logger());
                    self.logger.as_ref().unwrap()
                },
                Some(ref v) => v,
            };

            let _ = logger.send(self.log_level, self.name, format!("Set State to {}", state.is_selected.to_bool()));
        }
    }
}