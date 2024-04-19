// import necessary modules
use std::{time};
use std::cmp::max;
use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use ratatui::widgets::Paragraph;
use ratatui::prelude::*;

use crossterm::{terminal::{
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
}, event::KeyCode::Char};

use anyhow::Result;
use crossterm::event::{Event};
use ratatui::widgets::{Block, Borders};
use tui_big_text::{BigTextBuilder, PixelSize}; // allows big text

fn main() -> Result<()>{
    let mut hint_display: Vec<char> = Vec::new(); // this is the big text hint display
    let _duration = time::Duration::from_millis(333);
    let mut counter = 0;
    let mut limit = 1;
    let mut typed_characters = String::new(); // input buffer
    enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let sequence = "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914564856692346034861045432664821339360726024914127372458700660631558817488152092096282925409171536436789259036001133053054882046652138414695194151160943305727036575959195309218611738193261179310511854807446237996274956735188575272489122793818301194912983367336244065664308602139494639522473719070217986094370277053921717629317675238467481846766940513200056812714526356082778577134275778960917363717872146844090122495343014654958537105079227968925892354201995611212902196086403441815981362977477130996051870721134999999837297804995105973173281609631859502445945534690830264252230825334468503526193118817101000313783875288658753320838142061717766914730359825349042875546873115956286388235378759375195778185778053217122680661300192787661119590921642019893809525720106548586327886593615338182796823030195203530185296899577362259941389124972177528347913151557485724245415069595082953311686172785588907509838175463746493931925506040092770167113900984882401285836160356370766010471018194295559619894676783744944825537977472684710404753464620804668425906949129331367702898915210475216205696602405803815019351125338243003558764024749647326391419927260426992279678235478163600934172164121992458631503028618297455570674983850549";
    let pi: Vec<char> = sequence.chars().collect(); // Creating a vector of characters of pi to an arbitrary length

    // This code is using the Crossterm library to draw to the terminal
    // and the Ratatui library to create a layout and a paragraph widget.
    // This is only run the first time to initialize the terminal and draw the paragraph widget.

    terminal.draw(|f| {
        // Create a layout with two vertical sections
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                // The first section has a minimum height of 7 lines
                Constraint::Min(7),
                // The second section takes up the remaining vertical space
                Constraint::Percentage(100),
            ])
            .split(f.size());

        // Render a paragraph widget in the second section of the layout
        f.render_widget(
            // Create a new paragraph widget with the contents of the `typed_characters` variable
            Paragraph::new(typed_characters.to_string())
                // Add a block around the paragraph with a title and borders
                .block(Block::default().title("Pi memorized to 0 digits".to_string()).borders(Borders::ALL)),
            // Render the paragraph widget in the second section of the layout
            layout[1]);
    })?;

    // Main event loop

    loop {
        // Check for user input every 250 milliseconds
        if crossterm::event::poll(time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let Event::Key(key) = crossterm::event::read()? {
                // If the key kind is a press event, match the key code.
                if key.kind == crossterm::event::KeyEventKind::Press {
                    // matched_key is a char that matches the key pressed
                    let matched_key = match key.code {
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


                    typed_characters.push(matched_key); // add the matched key to the input buffer

                    // if the added character is not equal to the pi sequence, break
                    if pi[counter] != matched_key {
                        break
                    }

                    // Collecting the hint display into a string for bigtext formatting
                    let collected_vec: String = hint_display.clone().into_iter().collect();

                    // limit is set to 1 at the start of the program
                    // This loop will add the next character to the hint display
                    // The 2 is added to the length of the typed characters to account for the 3. at the start of the pi sequence
                    while limit <= typed_characters.len()+2 {
                        hint_display.push(pi[limit-1]); // the one is subtracted to account for the 0 index
                        limit += 1;
                    }

                    // This function will animate the hint display
                    for _i in 0..(counter + 1) {
                        let value = animate_letters(collected_vec.clone(), counter);
                        log(&value.clone().expect("something went wrong logging the value"));

                        let big_text = BigTextBuilder::default()
                            .pixel_size(PixelSize::Full)
                            .style(Style::new())
                            .lines(vec![
                                value.expect("something went wrong animating letters").clone().white().into(), // the hint display is white and shown
                            ])
                            .build()?;

                        terminal.draw(|f| {
                            let layout = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints(vec![
                                    Constraint::Min(7),
                                    Constraint::Percentage(100),
                                ])
                                .split(f.size());

                            f.render_widget(big_text, layout[0]);

                            f.render_widget(
                                Paragraph::new(typed_characters.to_string())
                                    .block(Block::default().title(format!("Pi memorized to {} digits", (max(counter, 1) - 1)).to_string()).borders(Borders::ALL)),
                                layout[1]);
                        })?;
                    }
                    
                    counter += 1
                }
            }
        }
    }

    // shutdown: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("You memorized {} digits of pi! The last digit was {}, not {}.", max(counter, 1) - 1, pi[counter], typed_characters.chars().last().unwrap());
    // assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
    Ok(())
}

fn log(variable: &str) -> std::io::Result<()> {
    let mut file = File::create("log.txt")?;
    file.write_all(variable.as_bytes())?;
    Ok(())
}

fn animate_letters(collected_vec: String, counter: usize) -> Option<String> {
    for i in 0..(counter + 1){
        let letter = &collected_vec[..i];
        sleep(time::Duration::from_millis(6));
        return Some(letter.to_string());
    }
    None
}