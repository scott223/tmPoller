use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::*, prelude::*, widgets::*};
use std::{
    error::Error,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};
//use anyhow::{Context, Result};

use crate::schema::{App, Messages};

/// Setup the terminal. This is where we would enable raw mode, enter the alternate screen, and
/// hide the cursor. This functions does not handle errors yet. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error + Send + Sync>>
{
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    Ok(terminal)
} // fn setup_terminal

/// Restore the terminal. This is where we disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
} // fn restore_terminal

// function ui
// renders the main window, taking the current state of App
//
// arguments
// f (frame) and App
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &Arc<Mutex<App>>, messages: &Messages) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let unlocked_app = app.lock().unwrap();

    if unlocked_app.events.len() > 0 {
        let event_items: Vec<ListItem> = unlocked_app
            .events
            .iter()
            .enumerate()
            .map(|(_i, e)| {
                let line_item = Line::from(Span::raw(format!(
                    "{} ({}) has {} offers (last polled at {} (-))",
                    e.name,
                    e.id,
                    e.num_offers,
                    e.last_updated
                        .expect("cannot unwrap to datetime")
                        .format("%H:%M:%S")
                )));
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

    let messages: Vec<ListItem> = messages
        .messages
        .iter()
        .rev()
        .enumerate()
        .map(|(_i, m)| {
            let content = Line::from(Span::raw(format!(
                "{} | {}",
                m.datetime_sent.expect("dt error").format("%d/%m %H:%M:%S"),
                m.content
            )));
            ListItem::new(content)
        })
        .collect();

    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[1]);
} // fn ui
