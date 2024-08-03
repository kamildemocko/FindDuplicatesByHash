mod internal;

use internal::{app, arguments};


fn main() {
    let root_folder = arguments::get_argument();
    let mut app = app::App::new(&root_folder);
    app.run();
}
