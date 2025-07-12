use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{cat::Cat, app::App};

pub fn draw(frame: &mut Frame, app: &App) {
    #[cfg(debug_assertions)]
    let constraints = if app.show_debug {
        vec![
            Constraint::Length(3),   // タイトル
            Constraint::Length(14),  // 猫の表示（6行対応に拡大）
            Constraint::Length(3),   // コマンド
            Constraint::Min(5),      // デバッグ情報
        ]
    } else {
        vec![
            Constraint::Length(3),   // タイトル
            Constraint::Length(14),  // 猫の表示（6行対応に拡大）
            Constraint::Length(3),   // コマンド
        ]
    };
    
    #[cfg(not(debug_assertions))]
    let constraints = vec![
        Constraint::Length(3),   // タイトル
        Constraint::Length(14),  // 猫の表示（6行対応に拡大）
        Constraint::Length(3),   // コマンド
    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(frame.size());

    draw_title(frame, chunks[0]);
    draw_cat(frame, chunks[1], &app.cat);
    draw_commands(frame, chunks[2]);
    
    #[cfg(debug_assertions)]
    if app.show_debug && chunks.len() > 3 {
        let debug_text = app.cat.debug_status();
        draw_debug_info(frame, chunks[3], &debug_text);
    }
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("neko-watch")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );
    frame.render_widget(title, area);
}

fn draw_cat(frame: &mut Frame, area: Rect, cat: &Cat) {
    let color = get_cat_color(cat);
    let cat_art = Paragraph::new(cat.get_ascii_art())
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(format!("名前: {}", cat.name))
                .borders(Borders::ALL),
        );
    frame.render_widget(cat_art, area);
}

fn get_cat_color(cat: &Cat) -> Color {
    // 睡眠中は専用の色
    if cat.is_sleeping() {
        return Color::Blue;
    }
    
    let state = cat.get_state();
    match state {
        crate::cat::CatState::Dying => Color::Red,
        crate::cat::CatState::Sick => Color::LightRed,
        crate::cat::CatState::Hungry => Color::LightYellow,
        crate::cat::CatState::Dirty => Color::Yellow,
        crate::cat::CatState::Unhappy => Color::Gray,
        crate::cat::CatState::Tired => Color::DarkGray,
        crate::cat::CatState::Happy => Color::LightGreen,
        crate::cat::CatState::Full => Color::Green,
        crate::cat::CatState::Normal => Color::Yellow,
    }
}

fn draw_commands(frame: &mut Frame, area: Rect) {
    #[cfg(debug_assertions)]
    let commands = vec![
        Span::raw("[1] 餌をあげる  "),
        Span::raw("[2] 遊ぶ  "),
        Span::raw("[3] お風呂  "),
        Span::raw("[4] 寝かせる  "),
        Span::styled("[q] 終了", Style::default().fg(Color::Red)),
        Span::raw("  "),
        Span::styled("[d] デバッグ [h] 非表示 [0] 瀕死 [8] 病気 [9] 空腹", Style::default().fg(Color::Gray)),
    ];
    
    #[cfg(not(debug_assertions))]
    let commands = vec![
        Span::raw("[1] 餌をあげる  "),
        Span::raw("[2] 遊ぶ  "),
        Span::raw("[3] お風呂  "),
        Span::raw("[4] 寝かせる  "),
        Span::styled("[q] 終了", Style::default().fg(Color::Red)),
    ];
    
    let commands_paragraph = Paragraph::new(Line::from(commands))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(commands_paragraph, area);
}

#[cfg(debug_assertions)]
fn draw_debug_info(frame: &mut Frame, area: Rect, debug_text: &str) {
    let debug_paragraph = Paragraph::new(debug_text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("デバッグ情報")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        );
    frame.render_widget(debug_paragraph, area);
}