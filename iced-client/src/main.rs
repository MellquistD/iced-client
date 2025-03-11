mod app;
mod helpers;
mod pages;

pub use app::*;
pub use helpers::*;
pub use pages::*;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view).run()
}
