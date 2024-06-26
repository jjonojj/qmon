
use crossterm::{cursor, event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers}, style::ResetColor, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};
use crate::window::*;

// qmon's editor
pub struct Editor {
    should_quit : bool
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit : false }
    }

    pub fn run(&mut self) {
        let mut win = Window::default();
        if let Err(err) = self.repl(&mut win) {
            panic!("{err:#?}");
        }

        print!("{}", Clear(ClearType::All));
        print!("{}", ResetColor);
        print!("{}", cursor::MoveTo(0, 0));
        println!("exit signal recieved, quit.");
    }

    pub fn repl(&mut self, window : &mut Window) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        window.draw();
        loop {
            if let Key( KeyEvent {
                code, modifiers, kind, state
            } ) = read()? {
                window.draw();
                // println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"); // debug info
                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    },
                    KeyCode::Up |
                    KeyCode::Down |
                    KeyCode::Left |
                    KeyCode::Right => window.update_pos( code ),
                    _ => ()
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}