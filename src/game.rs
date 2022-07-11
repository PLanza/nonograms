use std::io::stdout;
use std::path::Path;

use crate::board::{Board, CellState};
use crate::constraints::Constraints;
use crate::cursor::Cursor;
use crate::parsing;

use crossterm::{cursor, event, execute, terminal};

pub struct Game {
    board: Board,
    v_constraints: Constraints,
    h_constraints: Constraints,
    dimensions: (usize, usize),
    cursor: Cursor,
}

impl Game {
    pub fn from_file(path: &Path) -> Result<Game, String> {
        let constraints = parsing::parse_file(path)?;
        let dimensions = (constraints.0.get_size(), constraints.1.get_size());

        Ok(Game {
            board: Board::new(dimensions.0, dimensions.1),
            v_constraints: constraints.0,
            h_constraints: constraints.1,
            dimensions,
            cursor: Cursor::new(dimensions),
        })
    }

    pub fn init(&self) -> Result<(), String> {
        terminal::enable_raw_mode().map_err(|e| e.to_string())?;

        let draw_width = self.h_constraints.get_max_digits().0
            + self.h_constraints.get_max_len().0
            + 1
            + 3 * self.dimensions.0
            + 1;
        let draw_height = self.v_constraints.get_max_len().0 + 1 + 2 * self.dimensions.1 + 1;

        execute!(stdout(), cursor::Hide, cursor::MoveTo(0, 0)).map_err(|e| e.to_string())?;

        let old_term_size = terminal::size().map_err(|e| e.to_string())?;
        if old_term_size == (draw_width as u16, draw_height as u16) {
            return Ok(());
        }

        execute!(
            stdout(),
            terminal::SetSize(draw_width as u16, draw_height as u16)
        )
        .map_err(|e| e.to_string())?;

        // Wait until resize happens
        loop {
            match event::read().map_err(|e| e.to_string())? {
                event::Event::Resize(..) => break,
                _ => (),
            }
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<(), String> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All),)
            .map_err(|e| e.to_string())?;

        // Horizontal offset until board begins
        let h_cs_len = self.h_constraints.get_max_digits().0 + self.h_constraints.get_max_len().0;

        // Vertical offset until board begins
        let v_max_len = self.v_constraints.get_max_len().0;

        // Draw the vertical constraints
        for i in 0..v_max_len {
            print!("{:1$}│", "", h_cs_len);
            for c_i in 0..self.dimensions.0 {
                let curr_constraints = self.v_constraints.get(c_i)?;

                match (curr_constraints.len() + i).checked_sub(v_max_len) {
                    None => print!("  "),
                    Some(index) => print!("{:2}", curr_constraints[index]),
                }
                print!("│");
            }
            print!("\r\n");
        }

        // Top Board Row
        print!("{:─<1$}", "", h_cs_len);
        print!("╆");
        for j in 0..(self.dimensions.0 - 1) {
            if (j + 1) % 5 == 0 {
                print!("━━╈");
            } else {
                print!("━━┿");
            }
        }
        print!("━━┪\r\n");

        for i in 0..self.dimensions.1 {
            // Draw horizontal constraints
            let mut constraint_string = String::new();
            for c in self.h_constraints.get(i)? {
                constraint_string.push_str(format!(" {}", c).as_str());
            }
            print!("{:>1$}", constraint_string, h_cs_len);

            print!("┃");
            for j in 0..self.dimensions.0 {
                match self.board.get(j, i) {
                    CellState::BLANK => print!("  "),
                    CellState::FILLED => print!("██"),
                    CellState::CROSSED => print!(""),
                };

                if j == self.dimensions.0 - 1 {
                    print!("┃\r\n");
                } else if (j + 1) % 5 == 0 {
                    print!("┇");
                } else {
                    print!("│");
                }
            }

            if i == self.dimensions.1 - 1 {
                continue;
            }

            print!("{:─<1$}", "", h_cs_len);
            if (i + 1) % 5 == 0 {
                print!("╊");
                for j in 0..(self.dimensions.0 - 1) {
                    if (j + 1) % 5 == 0 {
                        print!("┅┅╋");
                    } else {
                        print!("┅┅┿");
                    }
                }
                print!("┅┅┫\r\n");
            } else {
                print!("╂");
                for j in 0..(self.dimensions.0 - 1) {
                    if (j + 1) % 5 == 0 {
                        print!("──╂");
                    } else {
                        print!("──┼");
                    }
                }
                print!("──┨\r\n");
            }
        }

        // Bottom Board Row
        print!("{:─<1$}┺", "", h_cs_len);
        for j in 0..(self.dimensions.0 - 1) {
            if (j + 1) % 5 == 0 {
                print!("━━┻");
            } else {
                print!("━━┷");
            }
        }
        print!("━━┛\r\n");

        let term_size = terminal::size().map_err(|e| e.to_string())?;
        let board_size = (
            h_cs_len + 1 + 3 * self.board.cols,
            v_max_len + 1 + 2 * self.board.rows,
        );

        // Draw cursor
        let cursor_x = (h_cs_len + 3 * self.cursor.col) as u16;
        let cursor_y =
            (term_size.1 - board_size.1 as u16) + (v_max_len - 1 + 2 * self.cursor.row) as u16;
        self.draw_cursor(cursor_x, cursor_y)?;

        Ok(())
    }

    pub fn draw_cursor(&self, cursor_x: u16, cursor_y: u16) -> Result<(), String> {
        execute!(
            stdout(),
            cursor::SavePosition,
            cursor::MoveTo(cursor_x, cursor_y),
        )
        .map_err(|e| e.to_string())?;
        print!("╔══╗");
        execute!(stdout(), cursor::MoveTo(cursor_x, cursor_y + 1),).map_err(|e| e.to_string())?;
        print!("║");
        execute!(stdout(), cursor::MoveTo(cursor_x + 3, cursor_y + 1),)
            .map_err(|e| e.to_string())?;
        print!("║");
        execute!(stdout(), cursor::MoveTo(cursor_x, cursor_y + 2),).map_err(|e| e.to_string())?;
        print!("╚══╝");

        execute!(stdout(), cursor::RestorePosition,).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        use crossterm::event::{read, Event::Key, KeyCode, KeyEvent};

        self.draw()?;
        loop {
            match read().map_err(|e| e.to_string())? {
                Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => self.cursor.move_up(),
                    KeyCode::Left => self.cursor.move_left(),
                    KeyCode::Down => self.cursor.move_down(),
                    KeyCode::Right => self.cursor.move_right(),
                    KeyCode::Char('z') => self.board.toggle_fill_at(self.cursor.position()),
                    KeyCode::Char('x') => self.board.toggle_cross_at(self.cursor.position()),
                    _ => continue,
                },
                _ => continue,
            }
            self.draw()?;
        }

        Ok(())
    }

    pub fn quit(&self) -> Result<(), String> {
        terminal::disable_raw_mode().map_err(|e| e.to_string())?;

        execute!(stdout(), cursor::Show,).map_err(|e| e.to_string())?;

        Ok(())
    }
}
