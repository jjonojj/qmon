use std::{fs::read_to_string, io::{stdout, Write}};

use crossterm::{cursor, event::KeyCode, style::{Color, ResetColor, SetForegroundColor}, terminal::{size, Clear, ClearType}};

const LINE_IND_COLOR : Color = Color::Rgb { r: 44, g: 44, b: 44 };
const EMPTY_COLOR : Color  = Color::Rgb { r: 68, g: 68, b: 255 };

pub struct Location {
    x : u16,
    y : u16,
}

// qmon's window
pub struct Window {
    content : Vec<String>,
    pos : Location
}

impl Window {
    pub fn default() -> Self {
        Window {
            content : vec!["hello!".to_string(), "world.".to_string()],
            pos : Location { x : 2, y : 0 }
        }
    }
    pub fn draw(&self) {
        print!("{}", Clear(ClearType::All));
        print!("{}", cursor::MoveTo(0, 0));
        for (i, ln) in self.content.iter().enumerate() {
            print!("{}", SetForegroundColor(LINE_IND_COLOR));
            print!("{} ", self.get_line_num(self.content.len(), i as u16));
            print!("{}", ResetColor);
            println!("{}\r", ln);
        }
        print!("{}", SetForegroundColor(EMPTY_COLOR));
        for _ in self.content.len() as u16..size().unwrap().1-1 {
            println!("~\r",);
        }
        print!("~");
        print!("{}", cursor::MoveTo(self.pos.x, self.pos.y));
        let _ = stdout().flush();
    }
    pub fn update_pos(&mut self, code : KeyCode) {
        match code {
            KeyCode::Up => {
                if self.pos.y > 0 {
                    self.pos.y -= 1;
                }
            }
            KeyCode::Down => {
                if self.pos.y < self.content.len() as u16 - 1 {
                    self.pos.y += 1;
                }
            },
            KeyCode::Left => {
                if self.pos.x > 2 {
                    self.pos.x -= 1;
                }
            },
            KeyCode::Right => {
                if self.pos.x < self.content[self.pos.y as usize].len() as u16 + 2 {
                    self.pos.x += 1;
                }
                else {
                    self.pos.y += 1;
                    self.pos.x = 2;
                    if self.content.len() <= self.pos.y as usize {
                        self.content.push("".to_string());
                    }
                }
            },
            _ => ()
        }
        self.draw();
    }

    pub fn get_line_num(&self, len : usize, i : u16) -> String {
        let count = (len as u16 - 1).to_string().len();
        let num_len = self.pos.y.to_string().len();
        let mut ln = String::new();

        for _ in 0..(count - num_len) {
            ln.push('0');
        }
        ln.push_str(i.to_string().as_str());

        ln
    }

    pub fn get_indent_pos(&self) -> u16 {
        self.get_line_num(self.content.len(), self.pos.y as u16).len() as u16 + 1
    }

}