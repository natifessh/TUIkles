use cursive::views::{Dialog, TextView};
use cursive::Cursive;
pub mod dialog{
    use cursive::{views::{Dialog, TextView}, Cursive};


pub fn confirm_exit(s: &mut Cursive) {
    let popup = Dialog::new()
        .title("Confirm Exit")
        .content(TextView::new("Are you sure you want to exit?"))
        .button("No, I'll stay", |s| { s.pop_layer(); })
        .button("Yes, exit", |s| {
            s.pop_layer();
            s.quit();
        });

    s.add_layer(popup);
}
}
