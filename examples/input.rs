use anathema::component::*;
use anathema::prelude::*;
use anathema_extras::{Input, InputChange, Text};

#[derive(Debug, Default, State)]
struct IndexState {
    items: Value<List<String>>,
    inserts: Value<usize>,
    deletes: Value<usize>,
}

#[derive(Debug, Default)]
struct Index;

impl Component for Index {
    type Message = ();
    type State = IndexState;

    #[allow(unused_variables, unused_mut)]
    fn on_event(
        &mut self,
        event: &mut UserEvent<'_>,
        state: &mut Self::State,
        _: Children<'_, '_>,
        _: Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "new_item" => state.items.push_back(event.data::<Text>().to_string()),
            "changed" => match event.data::<InputChange>() {
                InputChange::Insert(_, _) => *state.inserts.to_mut() += 1,
                InputChange::Remove(_, _) => *state.deletes.to_mut() += 1,
            },
            _ => (),
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

fn main() {
    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);
    builder
        .default::<Input>("input", Input::template())
        .unwrap();
    builder
        .default::<Index>("index", template_str().to_template())
        .unwrap();
    let res = builder.finish(&mut backend, |runtime, backend| runtime.run(backend));

    if let Err(e) = res {
        eprintln!("{e}");
    }
}

fn template_str() -> &'static str {
    "
    border
        vstack
            hstack
                border [sides: 'right']
                    text 'inserts '
                        span [foreground: 'yellow'] state.inserts ' '
                text ' deletes '
                    span [foreground: 'red'] state.deletes
            border
                @input (on_enter->new_item, on_change->changed) [clear_on_enter: true]
                    text [foreground: 'dark_grey'] 'placeholder'
            border
                vstack
                    for item in state.items
                        text item
    "
}
