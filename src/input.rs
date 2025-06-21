use std::ops::Deref;

use anathema::component::*;
use anathema::default_widgets::Overflow;
use anathema::prelude::*;

// -----------------------------------------------------------------------------
//   - Change -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum InputChange {
    Insert(char, usize),
    Remove(char, usize),
}

impl InputChange {
    pub fn remove(character: char, pos: i32) -> Self {
        Self::Remove(character, pos as usize)
    }

    pub fn insert(character: char, pos: i32) -> Self {
        Self::Insert(character, pos as usize)
    }
}

// -----------------------------------------------------------------------------
//   - Text -
// -----------------------------------------------------------------------------
pub struct Text(String);

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

// -----------------------------------------------------------------------------
//   - State -
// -----------------------------------------------------------------------------
#[derive(Debug, Default, State)]
pub struct InputState {
    text: Value<String>,
    screen_cursor: Value<i32>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            text: String::new().into(),
            screen_cursor: 0.into(),
        }
    }

    // Update visible cursor pos
    fn update_cursor(&mut self, screen_cursor: i32) {
        self.screen_cursor.set(screen_cursor);
    }

    fn char_count(&self) -> usize {
        self.text.to_ref().chars().count()
    }
}

// -----------------------------------------------------------------------------
//   - Component -
// -----------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Input {
    cursor: i32,
}

impl Input {
    const ON_BLUR: &str = "on_blur";
    const ON_CHANGE: &str = "on_change";
    const ON_ENTER: &str = "on_enter";
    const ON_FOCUS: &str = "on_focus";
    pub const TEMPLATE: &'static str = include_str!("templates/input.aml");

    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn template() -> SourceKind {
        Self::TEMPLATE.to_template()
    }

    fn update_cursor(&mut self, state: &mut InputState, overflow: &mut Overflow, width: i32) {
        let mut display_cursor = self.cursor - overflow.offset().x;

        if display_cursor < 0 {
            overflow.scroll_right_by(display_cursor);
            display_cursor = 0;
        }

        if display_cursor >= width {
            let offset = display_cursor + 1 - width;
            overflow.scroll_right_by(offset);
            display_cursor = width - 1;
        }

        state.update_cursor(display_cursor);
    }

    fn insert_char(&mut self, c: char, state: &mut InputState) {
        if self.cursor == state.char_count() as i32 {
            state.text.to_mut().push(c);
        } else {
            state.text.to_mut().insert(self.cursor as usize, c);
        }
        self.cursor += 1;
    }
}

impl Component for Input {
    type Message = ();
    type State = InputState;

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        mut children: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        children.elements().by_tag("overflow").first(|el, _| {
            let width = el.size().width as i32;
            let overflow = el.to::<Overflow>();

            match key.code {
                KeyCode::Char(c) => {
                    self.insert_char(c, state);

                    let change = InputChange::insert(c, self.cursor);
                    context.publish(Self::ON_CHANGE, change);
                }
                KeyCode::Backspace if self.cursor > 0 => {
                    self.cursor -= 1;
                    let c = state.text.to_mut().remove(self.cursor as usize);

                    let change = InputChange::remove(c, self.cursor);
                    context.publish(Self::ON_CHANGE, change);
                }
                KeyCode::Enter if !state.text.to_ref().is_empty() => {
                    let clear_on_enter = context
                        .attributes
                        .get_as::<bool>("clear_on_enter")
                        .unwrap_or(true);
                    let text = match clear_on_enter {
                        true => {
                            self.cursor = 0;
                            std::mem::take(&mut *state.text.to_mut())
                        }
                        false => state.text.to_ref().clone(),
                    };

                    context.publish(Self::ON_ENTER, Text(text));
                }
                KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
                KeyCode::Right if state.char_count() as i32 > self.cursor => self.cursor += 1,
                KeyCode::Home => self.cursor = 0,
                KeyCode::End => self.cursor = state.char_count() as i32,
                KeyCode::Delete if state.char_count() > 0 => {
                    let c = state.text.to_mut().remove(self.cursor as usize);
                    let change = InputChange::remove(c, self.cursor);
                    context.publish(Self::ON_CHANGE, change);
                    return;
                }
                _ => (),
            }

            self.update_cursor(state, overflow, width);
        });
    }

    fn on_blur(
        &mut self,
        _: &mut Self::State,
        _: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        context.publish(Self::ON_BLUR, ());
    }

    fn on_focus(
        &mut self,
        _: &mut Self::State,
        _: Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        context.publish(Self::ON_FOCUS, ());
    }
}
