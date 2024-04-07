use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rust_space_invaders::{
    frame::{self, new_frame, Drawable},
    invader::Invaders,
    player::Player,
    render,
    util::load_audio,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Sounds are:
    // { explode, lose, move, pew, startup, win }
    let mut sounds = load_audio().unwrap();
    sounds.play("startup");

    //Setting up the terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let _ = stdout.execute(Hide);

    let (render_s, render_r) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_r.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game main loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            sounds.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        sounds.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        player.update(delta);
        if invaders.update(delta) {
            sounds.play("move");
        }
        if player.detect_hits(&mut invaders) {
            sounds.play("explode");
        }

        let drawable: Vec<&dyn Drawable> = vec![&player, &invaders];
        for d in drawable {
            d.draw(&mut curr_frame);
        }
        let _ = render_s.send(curr_frame).unwrap();
        thread::sleep(Duration::from_millis(1));

        if invaders.all_dead() {
            sounds.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            sounds.play("lose");
            break 'gameloop;
        }
    }

    //Cleanup
    render_handle.join().unwrap();
    sounds.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    let _ = terminal::disable_raw_mode();

    Ok(())
}
