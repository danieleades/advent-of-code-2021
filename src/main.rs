#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use cli::App;

mod cli;

fn main() {
    let app = App::from_cli();

    app.run();
}
