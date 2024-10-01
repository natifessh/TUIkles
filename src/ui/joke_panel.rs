use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, DummyView, LinearLayout, TextView};
use crate::models::Joke;
use crate::ui::dialog;

pub fn create_joke_panel(joke: Joke) -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new(joke.setup).with_name("setup").fixed_width(60))
        .child(TextView::new(joke.punchline).with_name("punchline").fixed_width(60))
        .child(DummyView)
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(DummyView.fixed_width(5))
                .child(Button::new("Ohh", move |s| {
                  dialog::dialog::confirm_exit(s);
                }))
                .child(DummyView.fixed_width(3))
                .child(Button::new("Get New Joke 😂", move |s| {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    match rt.block_on(crate::api::get_joke()) {
                        Ok(new_joke) => {
                            s.call_on_name("setup", |view: &mut TextView| {
                                view.set_content(new_joke.setup);
                            });
                            s.call_on_name("punchline", |view: &mut TextView| {
                                view.set_content(new_joke.punchline);
                            });
                        }
                        Err(err) => {
                            s.add_layer(TextView::new(format!("Error fetching joke: {}", err)));
                        }
                    }
                }))
        )
}
