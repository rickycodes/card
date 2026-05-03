use std::env;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

mod constants;

use crate::constants::{
    colours, ALERT, BG, CARD, CONTENT, DEFAULT_LOL_DIALOG_COUNT, HELLO, LOL_BUTTON_LABEL,
    LOL_CONFIRM_TEXT, LOL_MESSAGES, LOL_TITLES, MAX_LOL_DIALOG_COUNT, MUTED, PRIMARY,
    PRIMARY_ACTIVE, SECONDARY, SURFACE, TEXT,
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

#[derive(Clone, Copy)]
struct LolDialog {
    rect: Rect,
    title: &'static str,
    message: &'static str,
    confirm: &'static str,
}

struct AppState {
    selected_button: usize,
    dialogs: Vec<LolDialog>,
    dialogs_spawned_at: Option<Instant>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

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
    let mut app = AppState {
        selected_button: 0,
        dialogs: Vec::new(),
        dialogs_spawned_at: None,
    };

    loop {
        clear_expired_dialogs(&mut app);
        terminal.draw(|frame| draw(frame, &app))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Tab | KeyCode::Right | KeyCode::Down => {
                        app.selected_button = (app.selected_button + 1) % button_count();
                    }
                    KeyCode::BackTab | KeyCode::Left | KeyCode::Up => {
                        app.selected_button = if app.selected_button == 0 {
                            button_count() - 1
                        } else {
                            app.selected_button - 1
                        };
                    }
                    KeyCode::Enter => {
                        if app.selected_button < CARD.links.len() {
                            open_in_browser(CARD.links[app.selected_button].url)?;
                        } else {
                            let area = size_to_rect(terminal.size()?);
                            app.dialogs = spawn_lol_dialogs(area, lol_dialog_count());
                            app.dialogs_spawned_at = Some(Instant::now());
                        }
                    }
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
    lines.push(button_line(LOL_BUTTON_LABEL, CARD.links.len() == selected_button));

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
        .title_style(Style::default().fg(ALERT).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(ALERT))
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

fn clear_expired_dialogs(app: &mut AppState) {
    if app
        .dialogs_spawned_at
        .is_some_and(|spawned_at| spawned_at.elapsed() >= Duration::from_secs(5))
    {
        app.dialogs.clear();
        app.dialogs_spawned_at = None;
    }
}

fn spawn_lol_dialogs(area: Rect, count: usize) -> Vec<LolDialog> {
    let mut rng = SimpleRng::from_entropy();
    (0..count)
        .map(|_| random_lol_dialog(area, &mut rng))
        .collect()
}

fn random_lol_dialog(area: Rect, rng: &mut SimpleRng) -> LolDialog {
    let title = rng.pick(&LOL_TITLES);
    let message = rng.pick(&LOL_MESSAGES);
    let confirm = rng.pick(&LOL_CONFIRM_TEXT);
    let width = dialog_width(title, message, confirm).min(area.width.saturating_sub(1).max(1));
    let height = 6u16.min(area.height.saturating_sub(1).max(1));
    let max_x = area
        .x
        .saturating_add(area.width.saturating_sub(width).saturating_sub(1));
    let max_y = area
        .y
        .saturating_add(area.height.saturating_sub(height).saturating_sub(1));
    let x = rng.range_u16(area.x, max_x);
    let y = rng.range_u16(area.y, max_y);

    LolDialog {
        rect: Rect {
            x,
            y,
            width,
            height,
        },
        title,
        message,
        confirm,
    }
}

fn dialog_width(title: &str, message: &str, confirm: &str) -> u16 {
    let button_width = confirm.len().saturating_add(6);
    let width = title
        .len()
        .max(message.len())
        .max(button_width)
        .saturating_add(4);
    width.clamp(22, 42) as u16
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

fn button_count() -> usize {
    CARD.links.len() + 1
}

fn lol_dialog_count() -> usize {
    env::var("LOL_DIALOG_COUNT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .map(|value| value.clamp(1, MAX_LOL_DIALOG_COUNT))
        .unwrap_or(DEFAULT_LOL_DIALOG_COUNT)
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

struct SimpleRng(u64);

impl SimpleRng {
    fn from_entropy() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0x5eed_fade_cafe_beef);
        Self(seed ^ 0xa5a5_5a5a_d3c0_b33f)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }

    fn range_u16(&mut self, start: u16, end: u16) -> u16 {
        if start >= end {
            return start;
        }

        let span = u32::from(end - start + 1);
        start + (self.next_u32() % span) as u16
    }

    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        let index = (self.next_u32() as usize) % items.len();
        items[index]
    }
}
