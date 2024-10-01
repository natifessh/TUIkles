use cursive::Cursive;
use tokio::runtime::Runtime;

mod api;
mod models;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = cursive::default();
    let rt = Runtime::new().unwrap();
    let joke = rt.block_on(api::get_joke()).unwrap();
    ui::initialize_ui(&mut terminal, joke);
    terminal.run();
    Ok(())
}
