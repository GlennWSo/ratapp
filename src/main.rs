use std::{cell::RefCell, io, rc::Rc};

use ratzilla::{DomBackend, WebRenderer, ratatui::Terminal};
use rudoku::App;

fn main() -> io::Result<()> {
    // color_eyre::install()?;
    // let terminal = ratatui::init();
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let app = Rc::new(RefCell::new(App::default()));

    terminal.on_key_event({
        let app = app.clone();
        move |key_event| {
            app.borrow_mut().handle_key(key_event);
        }
    });
    terminal.draw_web(move |frame| app.borrow_mut().draw(frame));

    Ok(())
    // ratatui::restore();
    // app_result
    //
}
