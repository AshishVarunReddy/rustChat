use std::io::{Write, stdout};
use crossterm::{cursor, event::KeyModifiers, QueueableCommand};
use std::thread;
use crossterm::terminal::{self, ClearType};
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use str;

fn main(){
    let mut stdout = stdout();
    let _ = terminal::enable_raw_mode();
    let  (mut _width, mut height) = terminal::size().unwrap();
    stdout.queue(terminal::Clear(ClearType::FromCursorUp)).unwrap();
    let mut bar = str::repeat("⎯", _width as usize);
    let mut prompt = String::new();
    let mut quit = false;
    let mut chat = Vec::new();
    while !quit {
	while poll(Duration::ZERO).unwrap(){
	    
	    match read().unwrap() {
		Event::Resize(nw, nh) => {
		    _width = nw;
		    height = nh;
		    bar = str::repeat("⎯", _width as usize);
		},
		Event::Key(event) => {
		    match event.code {
			KeyCode::Char(x) => {
			    if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL){
				quit = true;
			    }else{
				prompt.push(x);
			    }
			},

			KeyCode::Enter => {
			    let clearline = str::repeat(" ", _width as usize);
			    chat.push(prompt.clone());
			    prompt.clear();
			    stdout.queue(cursor::MoveTo(0, height-1)).unwrap();
			    stdout.write(clearline.as_bytes()).unwrap();
			}
			KeyCode::Backspace => {
			    prompt.pop();
			    stdout.queue(cursor::MoveTo(0, height-1)).unwrap();

			    stdout.write(prompt.as_bytes()).unwrap();
			}
			_ => {},
		    }
		},
		_ => {},
	    }
	}
	//render_frame();
	for (row, line) in chat.iter().enumerate() {
	    stdout.queue(cursor::MoveTo(0,row as u16)).unwrap();
	    stdout.write(line.as_bytes()).unwrap();
	}
	stdout.queue(cursor::MoveTo(0, height-2)).unwrap();
	stdout.write(bar.as_bytes()).unwrap();
	stdout.queue(cursor::MoveTo(0, height-1)).unwrap();
	stdout.write(prompt.as_bytes()).unwrap();
	stdout.flush().unwrap();
	thread::sleep(Duration::from_millis(33));
    }

    
    
}
