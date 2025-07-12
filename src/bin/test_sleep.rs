use neko_watch::cat::Cat;

fn main() {
    println!("=== 睡眠アニメーションテスト ===\n");
    
    let mut cat = Cat::new("睡眠テスト猫".to_string());
    
    println!("通常状態:");
    println!("{}", cat.get_ascii_art());
    println!("睡眠中: {}", cat.is_sleeping());
    
    println!("\n[4]睡眠実行後:");
    cat.sleep();
    println!("睡眠中: {}", cat.is_sleeping());
    
    println!("\n睡眠中のアニメーション（4フレーム）:");
    for frame in 0..4 {
        println!("フレーム{}:", frame);
        // 内部フレームを手動設定してテスト
        let art = get_sleep_animation_test(frame);
        println!("{}", art);
        println!("---");
    }
    
    println!("\n実際の睡眠アニメーション:");
    println!("{}", cat.get_ascii_art());
}

fn get_sleep_animation_test(frame: usize) -> String {
    let (eyes, sleep_effect) = match frame % 4 {
        0 => ("( -.z )", "Zzz...       "),
        1 => ("( z.- )", "zzZ...       "),
        2 => ("( z.z )", "ZZZ...       "),
        _ => ("( -.z )", "zzz...       "),
    };
    format!("   /\\_/\\          \n  {}         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n   {}", eyes, sleep_effect)
}