use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
    time::Instant,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::*, prelude::*, widgets::*};

use anyhow::{Context, Result};
use schema::App;

mod event_poller;
pub mod schema;

const DEFAULT_POLLING_INTERVAL: Duration = Duration::new(30, 0);

/// Setup the terminal. This is where we would enable raw mode, enter the alternate screen, and
/// hide the cursor. This functions does not handle errors yet. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where we disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
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
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    let mut last_update = Instant::now();
    let poll_on_interval = DEFAULT_POLLING_INTERVAL;

    app.submit_message("Initialized, running program ...");

    app.submit_message("Executing first poll");

    match event_poller::update_events(&mut app) {
        // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
        Ok(()) => {
            last_update = Instant::now();
            app.submit_message("Events updated");
        }
        Err(e) => eprintln!("Error with updating events: {}", e),
    }    
    // Running main loop
    'mainloop: loop {
        terminal.draw(|f| ui(f, &app))?;

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // User pressed ESC or 'q', breaking the main loop
                        break 'mainloop;
                    }
                    KeyCode::Char('p') => {
                        // User pressed 'p', forcing an update of the events
                        app.submit_message("User forced polling of all events");

                        match event_poller::update_events(&mut app) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(()) => {
                                last_update = Instant::now();
                                app.submit_message("Events updated");
                            }
                            Err(e) => eprintln!("Error with updating events: {}", e),
                        }
                        // println!("Data dump: {:?}", app.events); // temp: data dump
                    }
                    _ => {}
                }
            }
        }

        if Instant::now().duration_since(last_update) > poll_on_interval {
            // Update interval exceeded
            app.submit_message("Interval triggered polling of all events");

            match event_poller::update_events(&mut app) {
                // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                Ok(()) => {
                    last_update = Instant::now();
                    app.submit_message("All events updated")
                }
                Err(e) => eprintln!("Error: {}", e),
            }
            // println!("Data dump: {:?}", app.events); // temp: data dump
        }
    }

    Ok(())
}

// function ui
// renders the main window, taking the current state of App
//
// arguments
// f (frame) and App
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    if app.events.len() > 0 {
        let event_items: Vec<ListItem> = app
            .events
            .iter()
            .enumerate()
            .map(|(_i,e)| {
                let line_item = Line::from(Span::raw(format!("{} ({}) has {} offers (last polled at {} ({}))",e.name,e.id, e.num_offers, e.last_updated.format("%H:%M:%S"),e.last_update_status_code)));
                ListItem::new(line_item)
            })
            .collect();

            let list = List::new(event_items)
            .block(Block::default().title("Offers").borders(Borders::ALL))
            .style(Style::default().fg(Color::Blue))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_widget(list, chunks[0]);       
    } else {
       //TODO write a nice message when there are no events to be polled yet
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .rev()
        .enumerate()
        .map(|(_i, m)| {
            let content = Line::from(Span::raw(format!("{} | {}",m.datetime_sent.format("%d/%m %H:%M:%S"),m.content)));
            ListItem::new(content)
        })
        .collect();

    let messages =
    List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[1]);

}