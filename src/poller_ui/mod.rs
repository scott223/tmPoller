use std::{
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::*, prelude::*, widgets::*};
use anyhow::{Context, Result};

use crate::schema::App;

/// Setup the terminal. This is where we would enable raw mode, enter the alternate screen, and
/// hide the cursor. This functions does not handle errors yet. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
} // fn setup_terminal

/// Restore the terminal. This is where we disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
} // fn restore_terminal

// function ui
// renders the main window, taking the current state of App
//
// arguments
// f (frame) and App
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
} // fn ui