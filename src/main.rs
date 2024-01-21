use ratatui::{prelude::{CrosstermBackend, Terminal}, widgets::Paragraph};

use crossterm::{terminal::{
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
}, event::KeyCode::Char};

use anyhow::Result;
use crossterm::event::Event;
use ratatui::widgets::{Block, Borders};

fn main() -> Result<()>{
    let mut counter = 0;
    let mut list = String::new();
    enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let pi = std::f64::consts::PI;

    terminal.draw(|f| {
        f.render_widget(
            Paragraph::new("Hello World!")
                .block(Block::default().title("Greeting").borders(Borders::ALL)),
            f.size())
    })?;
    loop {
        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    let a = match key.code {
                        Char('q') => break,
                        Char('0') => '0',
                        Char('1') => '1',
                        Char('2') => '2',
                        Char('3') => '3',
                        Char('4') => '4',
                        Char('5') => '5',
                        Char('6') => '6',
                        Char('7') => '7',
                        Char('8') => '8',
                        Char('9') => '9',
                        Char('.') => '.',
                        _ => continue,
                    };
                    list.push(a);
                    if pi.to_string().chars().nth(counter) != list.chars().nth(counter) {
                        break
                    }
                    terminal.draw(|f| {
                        f.render_widget(
                            Paragraph::new(list.clone())
                                .block(Block::default().title("Pi").borders(Borders::ALL)),
                            f.size())
                    })?;
                    counter += 1
                }
            }
        }
    }

    // shutdown: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}