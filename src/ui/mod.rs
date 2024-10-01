pub mod joke_panel;
pub mod dialog;
use colored::*;

use cursive::{views::{LinearLayout, PaddedView, Panel, TextView}, Cursive};
use crate::models::Joke;

pub fn initialize_ui(terminal: &mut Cursive, joke: Joke) {
    let joke_panel = joke_panel::create_joke_panel(joke);
    let layout = LinearLayout::vertical()
        .child(TextView::new("TUIkles").center())  
        .child(PaddedView::lrtb(1, 5, 2, 1, joke_panel));  
    terminal.add_layer(Panel::new(layout));
    terminal.add_global_callback('q', |s| s.quit());
}