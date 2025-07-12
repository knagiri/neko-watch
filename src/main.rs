mod app;
mod cat;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ターミナルの初期化
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリケーションの実行
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // ターミナルのリセット
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // UIの描画
        terminal.draw(|f| ui::draw(f, app))?;

        // 入力の処理
        app.handle_input()?;

        // ゲームの更新
        app.tick();

        // 終了フラグのチェック
        if app.should_quit {
            break;
        }

        // フレームレートの制御
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}