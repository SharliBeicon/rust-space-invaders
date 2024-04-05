mod util;

use std::{error::Error, io, time::Duration};
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use util::load_audio;

fn main() -> Result<(),Box<dyn Error>>{
    // Sounds are:
    // { explode, lose, move, pew, startup, win }
    let mut sounds = load_audio().unwrap();
    sounds.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let _ = stdout.execute(Hide);
    
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        sounds.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }
    }

    sounds.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    let _ = terminal::disable_raw_mode();
    
    Ok(())
}
