use neko_watch::cat::{Cat, CatState};

fn main() {
    println!("=== ASCII アート行数チェック ===\n");
    
    let mut cat = Cat::new("アートテスト猫".to_string());
    
    // 各状態のASCIIアートをチェック
    test_ascii_lines(&mut cat, "Normal", 50, 50, 50, 50);
    test_ascii_lines(&mut cat, "Dying", 5, 5, 5, 5);
    test_ascii_lines(&mut cat, "Sick", 50, 50, 50, 15);
    test_ascii_lines(&mut cat, "Hungry", 25, 50, 50, 50);
    test_ascii_lines(&mut cat, "Dirty", 50, 50, 25, 50);
    test_ascii_lines(&mut cat, "Unhappy", 50, 25, 50, 50);
    test_ascii_lines(&mut cat, "Tired", 50, 40, 50, 40);
    test_ascii_lines(&mut cat, "Happy", 50, 85, 50, 50);
    
    // 満腹状態
    cat.set_status_for_test(95, 50, 50, 50);
    cat.feed();
    test_ascii_lines(&mut cat, "Full", 95, 50, 50, 50);
    
    println!("\n=== Normal状態のアニメーション確認 ===");
    cat.set_status_for_test(50, 50, 50, 50);
    for frame in 0..4 {
        println!("フレーム{}:", frame);
        let art = get_animation_frame(&cat, frame);
        println!("{}", art);
        println!("---");
    }
}

fn test_ascii_lines(cat: &mut Cat, state_name: &str, hunger: i32, happiness: i32, cleanliness: i32, health: i32) {
    cat.set_status_for_test(hunger, happiness, cleanliness, health);
    let art = cat.get_ascii_art();
    let lines: Vec<&str> = art.split('\n').collect();
    let line_count = lines.len();
    
    println!("{}: {} 行 {:?}", 
             state_name, 
             line_count, 
             cat.get_state());
    
    if line_count != 6 {
        println!("  ⚠️  期待値: 6行, 実際: {}行", line_count);
        for (i, line) in lines.iter().enumerate() {
            println!("  行{}: '{}'", i+1, line);
        }
    }
}

// プライベート関数のテスト用
fn get_animation_frame(cat: &Cat, frame: usize) -> String {
    // Normal状態のアニメーションを手動で再現
    match frame % 4 {
        0 => "   /\\_/\\          \n  ( o.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    ".to_string(),
        1 => "   /\\_/\\          \n  ( -.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    ".to_string(),
        2 => "   /\\_/\\          \n  ( o.- )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    ".to_string(),
        _ => "   /\\_/\\          \n  ( o.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    ".to_string(),
    }
}