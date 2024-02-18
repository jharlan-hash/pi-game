use std::{time};
use ratatui::widgets::Paragraph;
use ratatui::prelude::*;

use crossterm::{terminal::{
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
}, event::KeyCode::Char};

use anyhow::Result;
use crossterm::event::Event;
use ratatui::widgets::{Block, Borders};
use tui_big_text::{BigTextBuilder, PixelSize};

fn main() -> Result<()>{
    let welcomescreen = r" 
  ____  _             _     _____            _             
 / ___|| |_ __ _ _ __| |_  |_   _|   _ _ __ (_)_ __   __ _ 
 \___ \| __/ _` | '__| __|   | || | | | '_ \| | '_ \ / _` |
  ___) | || (_| | |  | |_    | || |_| | |_) | | | | | (_| |
 |____/ \__\__,_|_|   \__|   |_| \__, | .__/|_|_| |_|\__, |
                                 |___/|_|            |___/ 
";
    let mut hint_display: Vec<char> = Vec::new();
    let _duration = time::Duration::from_millis(333);
    let mut counter = 0;
    let mut limit = 1;
    let mut typed_characters = String::new();
    enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let sequence = "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914564856692346034861045432664821339360726024914127372458700660631558817488152092096282925409171536436789259036001133053054882046652138414695194151160943305727036575959195309218611738193261179310511854807446237996274956735188575272489122793818301194912983367336244065664308602139494639522473719070217986094370277053921717629317675238467481846766940513200056812714526356082778577134275778960917363717872146844090122495343014654958537105079227968925892354201995611212902196086403441815981362977477130996051870721134999999837297804995105973173281609631859502445945534690830264252230825334468503526193118817101000313783875288658753320838142061717766914730359825349042875546873115956286388235378759375195778185778053217122680661300192787661119590921642019893809525720106548586327886593615338182796823030195203530185296899577362259941389124972177528347913151557485724245415069595082953311686172785588907509838175463746493931925506040092770167113900984882401285836160356370766010471018194295559619894676783744944825537977472684710404753464620804668425906949129331367702898915210475216205696602405803815019351125338243003558764024749647326391419927260426992279678235478163600934172164121992458631503028618297455570674983850549";
    let pi: Vec<char> = sequence.chars().collect();

    terminal.draw(|f| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ])
            .split(f.size());

        f.render_widget(
            Paragraph::new(welcomescreen)
            .block(Block::default().title((counter+1).to_string()).borders(Borders::ALL)),
            layout[0]);
        f.render_widget(
            Paragraph::new(typed_characters.to_string())
            .block(Block::default().title("Pi memorized to 0 digits".to_string()).borders(Borders::ALL)),
            layout[1]);
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

                    typed_characters.push(a);
                    if pi[counter] != typed_characters.chars().nth(counter).unwrap() {
                        break
                    }
                    
                    let collected_vec: String = hint_display.clone().into_iter().collect();

                    while limit <= typed_characters.len()+2 {
                        hint_display.push(pi[limit-1]);
                        limit += 1;
                    }


                    let big_text = BigTextBuilder::default()
                        .pixel_size(PixelSize::Full)
                        .style(Style::new().blue())
                        .lines(vec![
                            collected_vec.red().into(),
                        ]) 
                        .build()?;

                    terminal.draw(|f| {
                        let layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(vec![
                                Constraint::Percentage(30),
                                Constraint::Percentage(70),
                            ])
                            .split(f.size());
/*          
                        f.render_widget(
                            Paragraph::new(format!("{:?}", collected_vec.to_string()))
                                .block(Block::default().title(("Get Memorizing!").to_string()).borders(Borders::ALL)),
                            layout[0]);
*/
                        f.render_widget(big_text, layout[0]);

                        f.render_widget(
                            Paragraph::new(typed_characters.to_string())
                                .block(Block::default().title((format!("Pi memorized to {} digits", counter+1)).to_string()).borders(Borders::ALL)),
                            layout[1]);
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
