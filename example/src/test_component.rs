use anathema::component::*;

use anathema_logging::{LogLevel, send};

pub struct MyComponent {
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
        _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        if !mouse.lsb_up() {
            return;
        }
        let query = elements.at_position(mouse.pos());

        let mut should_change = false;
        query.first(|_, _| {
            should_change = true;
        });
        if should_change {
            state.is_selected.set(!state.is_selected.to_bool());
            send!(
                self.log_level,
                self.name,
                "Set State to {}",
                state.is_selected.to_bool()
            );
        }
    }
}
