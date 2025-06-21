use anathema::component::*;
use anathema::prelude::{SourceKind, ToSourceKind};

#[derive(Debug, State, Default)]
pub struct ButtonState {
    bg: Value<Color>,
}

#[derive(Debug, Default)]
pub struct Button {
    down: bool,
}

impl Button {
    const ON_BLUR: &str = "on_blur";
    const ON_FOCUS: &str = "on_focus";
    const ON_PRESS: &str = "on_press";
    pub const TEMPLATE: &'static str = include_str!("templates/button.aml");

    pub fn template() -> SourceKind {
        Self::TEMPLATE.to_template()
    }
}

impl Component for Button {
    type Message = ();
    type State = ButtonState;

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        let KeyCode::Enter = key.code else { return };
        context.publish(Self::ON_PRESS, ());
    }

    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        let pos = mouse.pos();

        // Mouse up
        if mouse.left_up() {
            if self.down {
                children
                    .elements()
                    .by_attribute("button", true)
                    .first(|el, attribs| {
                        attribs.set("inverse", false);
                        if el.bounds().contains(pos) {
                            context.publish(Self::ON_PRESS, ());
                        }
                    });
                self.down = false;
            }

            return;
        }

        // Mouse down
        if mouse.left_down() {
            children
                .elements()
                .at_position(pos)
                .by_attribute("button", true)
                .first(|el, attribs| {
                    self.down = true;
                    attribs.set("inverse", true);
                });
        }
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        context.publish(Self::ON_FOCUS, ());
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        context.publish(Self::ON_BLUR, ());
    }
}
