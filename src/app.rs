use ratatui::{
    buffer::Buffer,
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::constants::{
    Card, ALERT, CONTENT, DEFAULT_LOL_DIALOG_COUNT, HELLO, LOL_BUTTON_LABEL, LOL_CONFIRM_TEXT,
    LOL_MESSAGES, LOL_TITLES, MAX_LOL_DIALOG_COUNT, MUTED, PRIMARY, PRIMARY_ACTIVE, SECONDARY,
    SURFACE, TEXT,
};

const LOL_DIALOG_BATCH_LIFETIME_MS: u64 = 5_000;

#[derive(Clone, Copy)]
pub struct LolDialog {
    pub rect: Rect,
    pub title: &'static str,
    pub message: &'static str,
    pub confirm: &'static str,
}

pub struct AppState {
    pub selected_button: usize,
    pub dialogs: Vec<LolDialog>,
    dialogs_expire_at_ms: Option<u64>,
}

pub enum ActivateResult<'a> {
    OpenUrl(&'a str),
    SpawnLol,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_button: 0,
            dialogs: Vec::new(),
            dialogs_expire_at_ms: None,
        }
    }

    pub fn next_button(&mut self, card: &Card<'_>) {
        self.selected_button = (self.selected_button + 1) % button_count(card);
    }

    pub fn previous_button(&mut self, card: &Card<'_>) {
        self.selected_button = if self.selected_button == 0 {
            button_count(card) - 1
        } else {
            self.selected_button - 1
        };
    }

    pub fn activate_selected<'a>(&self, card: &'a Card<'a>) -> ActivateResult<'a> {
        if self.selected_button < card.links.len() {
            ActivateResult::OpenUrl(card.links[self.selected_button].url)
        } else {
            ActivateResult::SpawnLol
        }
    }

    pub fn spawn_lol_dialogs(&mut self, area: Rect, count: usize, now_ms: u64) {
        let mut rng = SimpleRng::from_seed(now_ms ^ 0xa5a5_5a5a_d3c0_b33f);
        self.dialogs = (0..count)
            .map(|_| random_lol_dialog(area, &mut rng))
            .collect();
        self.dialogs_expire_at_ms = Some(now_ms + LOL_DIALOG_BATCH_LIFETIME_MS);
    }

    pub fn clear_expired_dialogs(&mut self, now_ms: u64) {
        if self
            .dialogs_expire_at_ms
            .is_some_and(|expires_at| now_ms >= expires_at)
        {
            self.dialogs.clear();
            self.dialogs_expire_at_ms = None;
        }
    }
}

pub fn button_count(card: &Card<'_>) -> usize {
    card.links.len() + 1
}

pub fn lol_dialog_count() -> usize {
    std::env::var("LOL_DIALOG_COUNT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .map(|value| value.clamp(1, MAX_LOL_DIALOG_COUNT))
        .unwrap_or(DEFAULT_LOL_DIALOG_COUNT)
}

pub fn now_ms() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0)
    }
}

pub fn render_ansi(app: &AppState, card: &Card<'_>, width: u16, height: u16) -> String {
    let area = Rect {
        x: 0,
        y: 0,
        width,
        height,
    };
    let mut buffer = Buffer::empty(area);

    render_frame(&mut buffer, app, card);
    buffer_to_ansi(&buffer)
}

fn render_frame(buffer: &mut Buffer, app: &AppState, card: &Card<'_>) {
    let area = *buffer.area();
    let card_lines = build_card_lines(card, app.selected_button);
    let popup = popup_rect(&card_lines, area);

    Clear.render(popup, buffer);

    let block = Block::default()
        .title(format!(" {} ", card.handle))
        .title_alignment(ratatui::prelude::Alignment::Center)
        .borders(Borders::ALL)
        .title_style(Style::default().fg(TEXT).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(PRIMARY))
        .style(Style::default().bg(SURFACE));

    let inner = block.inner(popup);
    block.render(popup, buffer);

    let paragraph = Paragraph::new(card_lines)
        .alignment(ratatui::prelude::Alignment::Left)
        .wrap(Wrap { trim: false });
    paragraph.render(inner, buffer);

    for dialog in &app.dialogs {
        draw_lol_dialog(buffer, dialog, area);
    }
}

fn draw_lol_dialog(buffer: &mut Buffer, dialog: &LolDialog, area: Rect) {
    let rect = clip_rect(dialog.rect, area);
    if rect.width < 6 || rect.height < 4 {
        return;
    }

    Clear.render(rect, buffer);

    let block = Block::default()
        .title(format!(" {} ", dialog.title))
        .borders(Borders::ALL)
        .title_style(Style::default().fg(ALERT).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(ALERT))
        .style(Style::default().bg(SURFACE));
    let inner = block.inner(rect);
    block.render(rect, buffer);

    let lines = vec![
        Line::from(Span::styled(dialog.message, Style::default().fg(TEXT))),
        Line::raw(""),
        Line::from(Span::styled(
            dialog.confirm,
            Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::prelude::Alignment::Center)
        .wrap(Wrap { trim: true });
    paragraph.render(inner, buffer);
}

fn build_card_lines(card: &Card<'_>, selected_button: usize) -> Vec<Line<'static>> {
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
            card.name.to_string(),
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!(" | {}", card.title), Style::default().fg(MUTED)),
    ]));
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

    for (index, link) in card.links.iter().enumerate() {
        lines.push(button_line(link.label, index == selected_button));
    }
    lines.push(button_line(
        LOL_BUTTON_LABEL,
        card.links.len() == selected_button,
    ));

    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("Tab/arrows", Style::default().fg(PRIMARY)),
        Span::styled(" move focus", Style::default().fg(TEXT)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(PRIMARY)),
        Span::styled(" selects  |  ", Style::default().fg(TEXT)),
        Span::styled("q", Style::default().fg(PRIMARY)),
        Span::styled(" quits", Style::default().fg(TEXT)),
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

fn popup_rect(card_lines: &[Line<'_>], area: Rect) -> Rect {
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
    fn from_seed(seed: u64) -> Self {
        Self(seed)
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

fn buffer_to_ansi(buffer: &Buffer) -> String {
    let area = *buffer.area();
    let mut output = String::new();

    for y in area.y..area.bottom() {
        let mut current_style = Style::default();
        for x in area.x..area.right() {
            let cell = &buffer[(x, y)];
            if cell.skip {
                continue;
            }

            let style = cell.style();
            if style != current_style {
                output.push_str("\x1b[0m");
                output.push_str(&ansi_style(&style));
                current_style = style;
            }
            output.push_str(cell.symbol());
        }
        output.push_str("\x1b[0m");
        if y + 1 < area.bottom() {
            output.push('\n');
        }
    }

    output
}

fn ansi_style(style: &Style) -> String {
    let mut out = String::new();

    if let Some(fg) = style.fg {
        out.push_str(&ansi_color_sequence(38, fg));
    }
    if let Some(bg) = style.bg {
        out.push_str(&ansi_color_sequence(48, bg));
    }
    if style.add_modifier.contains(Modifier::BOLD) {
        out.push_str("\x1b[1m");
    }
    if style.add_modifier.contains(Modifier::ITALIC) {
        out.push_str("\x1b[3m");
    }
    if style.add_modifier.contains(Modifier::UNDERLINED) {
        out.push_str("\x1b[4m");
    }

    out
}

fn ansi_color_sequence(prefix: u8, color: Color) -> String {
    match color {
        Color::Rgb(r, g, b) => format!("\x1b[{prefix};2;{r};{g};{b}m"),
        Color::Reset => format!("\x1b[{prefix}m"),
        Color::Black => format!("\x1b[{prefix};5;0m"),
        Color::Red => format!("\x1b[{prefix};5;1m"),
        Color::Green => format!("\x1b[{prefix};5;2m"),
        Color::Yellow => format!("\x1b[{prefix};5;3m"),
        Color::Blue => format!("\x1b[{prefix};5;4m"),
        Color::Magenta => format!("\x1b[{prefix};5;5m"),
        Color::Cyan => format!("\x1b[{prefix};5;6m"),
        Color::Gray => format!("\x1b[{prefix};5;7m"),
        Color::DarkGray => format!("\x1b[{prefix};5;8m"),
        Color::LightRed => format!("\x1b[{prefix};5;9m"),
        Color::LightGreen => format!("\x1b[{prefix};5;10m"),
        Color::LightYellow => format!("\x1b[{prefix};5;11m"),
        Color::LightBlue => format!("\x1b[{prefix};5;12m"),
        Color::LightMagenta => format!("\x1b[{prefix};5;13m"),
        Color::LightCyan => format!("\x1b[{prefix};5;14m"),
        Color::White => format!("\x1b[{prefix};5;15m"),
        Color::Indexed(i) => format!("\x1b[{prefix};5;{i}m"),
    }
}
