use neko_watch::cat::{Cat, CatState};

fn main() {
    println!("=== 猫の状態遷移テスト ===\n");
    
    let mut cat = Cat::new("テスト猫".to_string());
    
    // 各状態をテスト
    test_state(&mut cat, "瀕死", 5, 5, 5, 5, CatState::Dying);
    test_state(&mut cat, "病気", 50, 50, 50, 15, CatState::Sick);
    test_state(&mut cat, "空腹", 25, 50, 50, 50, CatState::Hungry);
    test_state(&mut cat, "不潔", 50, 50, 25, 50, CatState::Dirty);
    test_state(&mut cat, "不機嫌", 50, 25, 50, 50, CatState::Unhappy);
    test_state(&mut cat, "疲労", 50, 40, 50, 40, CatState::Tired);
    test_state(&mut cat, "上機嫌", 50, 85, 50, 50, CatState::Happy);
    test_state(&mut cat, "普通", 50, 50, 50, 50, CatState::Normal);
    
    // 満腹状態のテスト
    println!("=== 満腹状態テスト ===");
    cat.set_status_for_test(95, 50, 50, 50);
    println!("餌やり前: {:?}", cat.get_state());
    cat.feed();
    println!("餌やり直後: {:?}", cat.get_state());
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("2秒後: {:?}", cat.get_state());
    
    // アクション効果のテスト
    println!("\n=== アクション効果テスト ===");
    cat.set_status_for_test(50, 50, 50, 50);
    println!("初期状態: {}", cat.debug_status());
    
    cat.feed();
    println!("餌やり後: {}", cat.debug_status());
    
    cat.play();
    println!("遊び後: {}", cat.debug_status());
    
    cat.bathe();
    println!("お風呂後: {}", cat.debug_status());
    
    cat.sleep();
    println!("睡眠後: {}", cat.debug_status());
}

fn test_state(cat: &mut Cat, name: &str, hunger: i32, happiness: i32, cleanliness: i32, health: i32, expected: CatState) {
    cat.set_status_for_test(hunger, happiness, cleanliness, health);
    let actual = cat.get_state();
    let result = if actual == expected { "✓" } else { "✗" };
    println!("{} {}: 期待値={:?}, 実際={:?}", result, name, expected, actual);
}