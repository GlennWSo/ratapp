use std::io;

use ratapp::App;

fn main() -> io::Result<()> {
    ratatui::run(|term| App::default().run(term))
}
