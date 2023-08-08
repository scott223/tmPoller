use std::{error::Error, time::Duration, time::Instant, io::{self, Stdout}};

use crossterm::{
    event::{self, Event, KeyCode, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::*, prelude::*, widgets::*};

use anyhow::{Context, Result};

mod event_poller;
pub mod schema;

use crate::schema::TMEvent;

const DEFAULT_POLLING_INTERVAL: Duration = Duration::new(60,0);

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}

// Function run
//
// creates an "infitine" loop that checks for keystrokes, and executes the correspondig functions
//
// Arguments
//
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, tm_events: &mut Vec<TMEvent>) -> Result<(), Box<dyn Error>> {
    let mut last_update = Instant::now();
    let mut poll_on_interval = DEFAULT_POLLING_INTERVAL;

    // Running main loop
    'mainloop: loop {
        terminal.draw(|f| ui(f, tm_events))?;

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // User pressed ESC or 'q', breaking the main loop
                        break 'mainloop;
                    }
                    KeyCode::Char('p') => {
                        // User pressed 'p', forcing an update of the events
                        println!("User forced polling of all events");

                        match event_poller::update_events(tm_events) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(()) => {
                                last_update = Instant::now();
                                println!("Events updated");
                            }
                            Err(e) => println!("Error with updating events: {}", e),
                        }
                        println!("Data dump: {:?}", tm_events); // temp: data dump
                    }
                    _ => {}
                }
            }
        }

        if Instant::now().duration_since(last_update) > poll_on_interval {
            // Update interval exceeded
            println!("Interval triggered polling of all events");

            match event_poller::update_events(tm_events) {
                // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                Ok(()) => {
                    last_update = Instant::now();
                    println!("All events updated")
                }
                Err(e) => println!("Error: {}", e),
            }
            println!("Data dump: {:?}", tm_events); // temp: data dump
        }
    }

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, tm_events: &mut Vec<TMEvent>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let block = Block::default()
    .title("Events polled")
    .borders(Borders::ALL);

    let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_widget(list, chunks[0]);

}
