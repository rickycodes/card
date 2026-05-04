use std::io;
use std::process::Command;
use std::time::Duration;

use card::app::{lol_dialog_count, now_ms, ActivateResult, AppState, LolDialog};
use card::constants::{
    colours, BG, CARD, CONTENT, HELLO, LOL_BUTTON_LABEL, MUTED, PRIMARY, PRIMARY_ACTIVE, SECONDARY,
    SURFACE, TEXT,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_app().unwrap_or_else(report_terminal_error),
        2 => {
            let arg = &args[1];
            match arg.as_str() {
                "colours" | "colors" => colours(),
                _ => println!(
                    "This is not a valid argument. Please use none or `colours`: {:?}",
                    arg.as_str()
                ),
            }
        }
        _ => run_app().unwrap_or_else(report_terminal_error),
    }
}

fn report_terminal_error(error: io::Error) {
    eprintln!("failed to run terminal UI: {error}");
    std::process::exit(1);
}

fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_event_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run_event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = AppState::new();

    loop {
        app.clear_expired_dialogs(now_ms());
        terminal.draw(|frame| draw(frame, &app))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Tab | KeyCode::Right | KeyCode::Down => {
                        app.next_button(&CARD);
                    }
                    KeyCode::BackTab | KeyCode::Left | KeyCode::Up => {
                        app.previous_button(&CARD);
                    }
                    KeyCode::Enter => match app.activate_selected(&CARD) {
                        ActivateResult::OpenUrl(url) => open_in_browser(url)?,
                        ActivateResult::SpawnLol => {
                            let area = size_to_rect(terminal.size()?);
                            app.spawn_lol_dialogs(area, lol_dialog_count(), now_ms());
                        }
                    },
                    _ => {}
                },
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, app: &AppState) {
    let area = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(BG)), area);

    let card_lines = build_card_lines(app.selected_button);
    let popup = popup_rect(&card_lines, area);

    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(format!(" {} ", CARD.handle))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .title_style(Style::default().fg(TEXT).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(PRIMARY))
        .style(Style::default().bg(SURFACE));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let paragraph = Paragraph::new(card_lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner);

    for dialog in &app.dialogs {
        draw_lol_dialog(frame, dialog, area);
    }
}

fn build_card_lines(selected_button: usize) -> Vec<Line<'static>> {
    let mut lines = vec![Line::raw("")];
    lines.extend(HELLO.iter().map(|line| {
        Line::from(Span::styled(
            (*line).to_string(),
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ))
    }));

    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(
            CARD.name.to_string(),
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!(" | {}", CARD.title), Style::default().fg(MUTED)),
    ]));
    // lines.push(Line::from(vec![
    //     Span::raw("  "),
    //     Span::styled(CARD.company.to_string(), Style::default().fg(PRIMARY)),
    // ]));
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("  {}", CONTENT[0]),
        Style::default().fg(TEXT),
    )));
    lines.push(Line::from(Span::styled(
        format!("  {}", CONTENT[1]),
        Style::default().fg(TEXT),
    )));
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(CONTENT[2].to_string(), Style::default().fg(SECONDARY)),
    ]));
    lines.push(Line::raw(""));

    for (index, link) in CARD.links.iter().enumerate() {
        lines.push(button_line(link.label, index == selected_button));
    }
    lines.push(button_line(
        LOL_BUTTON_LABEL,
        CARD.links.len() == selected_button,
    ));

    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("Tab/arrows", Style::default().fg(PRIMARY)),
        Span::styled(" move focus", Style::default().fg(MUTED)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(PRIMARY)),
        Span::styled(" selects  |  ", Style::default().fg(MUTED)),
        Span::styled("q", Style::default().fg(PRIMARY)),
        Span::styled(" quits", Style::default().fg(MUTED)),
    ]));

    lines
}

fn button_line(label: &str, selected: bool) -> Line<'static> {
    let style = if selected {
        Style::default()
            .fg(SECONDARY)
            .bg(PRIMARY_ACTIVE)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(TEXT)
            .bg(SURFACE)
            .add_modifier(Modifier::BOLD)
    };

    Line::from(vec![
        Span::raw("  "),
        Span::styled(label.to_string(), style),
    ])
}

fn popup_rect(card_lines: &[Line], area: Rect) -> Rect {
    let card_width = card_lines
        .iter()
        .map(Line::width)
        .max()
        .unwrap_or(60)
        .saturating_add(4)
        .min(usize::from(area.width.saturating_sub(2).max(1))) as u16;
    let card_height = (card_lines.len() as u16 + 4).min(area.height.saturating_sub(2).max(1));
    centered_rect(card_width, card_height, area)
}

fn draw_lol_dialog(frame: &mut Frame, dialog: &LolDialog, area: Rect) {
    let rect = clip_rect(dialog.rect, area);
    if rect.width < 6 || rect.height < 4 {
        return;
    }

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(format!(" {} ", dialog.title))
        .borders(Borders::ALL)
        .title_style(
            Style::default()
                .fg(PRIMARY_ACTIVE)
                .add_modifier(Modifier::BOLD),
        )
        .border_style(Style::default().fg(PRIMARY))
        .style(Style::default().bg(SURFACE));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let lines = vec![
        Line::from(Span::styled(dialog.message, Style::default().fg(TEXT))),
        Line::raw(""),
        Line::from(Span::styled(
            dialog.confirm,
            Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, inner);
}

fn clip_rect(rect: Rect, bounds: Rect) -> Rect {
    let x = rect.x.max(bounds.x);
    let y = rect.y.max(bounds.y);
    let right = rect
        .x
        .saturating_add(rect.width)
        .min(bounds.x.saturating_add(bounds.width));
    let bottom = rect
        .y
        .saturating_add(rect.height)
        .min(bounds.y.saturating_add(bounds.height));

    Rect {
        x,
        y,
        width: right.saturating_sub(x),
        height: bottom.saturating_sub(y),
    }
}

fn open_in_browser(url: &str) -> io::Result<()> {
    let mut command = if cfg!(target_os = "macos") {
        let mut command = Command::new("open");
        command.arg(url);
        command
    } else if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", url]);
        command
    } else {
        let mut command = Command::new("xdg-open");
        command.arg(url);
        command
    };

    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());
    command.spawn()?;
    Ok(())
}

fn size_to_rect(size: Size) -> Rect {
    Rect {
        x: 0,
        y: 0,
        width: size.width,
        height: size.height,
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let horizontal_margin = area.width.saturating_sub(width) / 2;
    let vertical_margin = area.height.saturating_sub(height) / 2;

    Rect {
        x: area.x + horizontal_margin,
        y: area.y + vertical_margin,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
