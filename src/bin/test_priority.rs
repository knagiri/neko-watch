use neko_watch::cat::{Cat, CatState};

fn main() {
    println!("=== 状態優先順位テスト ===\n");
    
    let mut cat = Cat::new("優先度テスト猫".to_string());
    
    // 複数条件が同時に満たされた場合の優先順位テスト
    println!("1. 瀕死 vs 他の状態（瀕死が最優先）");
    test_priority(&mut cat, 5, 5, 5, 5, CatState::Dying, "全ステータス危険→瀕死");
    test_priority(&mut cat, 5, 50, 50, 50, CatState::Dying, "空腹度のみ危険→瀕死");
    
    println!("\n2. 病気 vs 空腹・不潔・不機嫌");
    test_priority(&mut cat, 25, 25, 25, 15, CatState::Sick, "複数問題あるが健康度最悪→病気");
    
    println!("\n3. 空腹 vs 不潔・不機嫌");
    test_priority(&mut cat, 25, 25, 25, 50, CatState::Hungry, "空腹・不機嫌・不潔だが空腹優先");
    
    println!("\n4. 不潔 vs 不機嫌");
    test_priority(&mut cat, 50, 25, 25, 50, CatState::Dirty, "不機嫌・不潔だが不潔優先");
    
    println!("\n5. 疲労の条件");
    test_priority(&mut cat, 50, 45, 50, 45, CatState::Tired, "健康度<50かつ幸福度<50→疲労");
    test_priority(&mut cat, 50, 45, 50, 55, CatState::Normal, "健康度>50で幸福度>30なので普通");
    
    println!("\n6. 上機嫌 vs 満腹");
    test_priority(&mut cat, 95, 85, 50, 50, CatState::Happy, "幸福度>80かつ空腹度>90だが餌やりなしなので上機嫌");
    
    // 餌やり後の満腹状態テスト
    cat.set_status_for_test(95, 85, 50, 50);
    cat.feed();
    println!("餌やり後: {:?} (満腹状態が優先されるべき)", cat.get_state());
    
    println!("\n7. 境界値テスト");
    test_priority(&mut cat, 10, 50, 50, 50, CatState::Hungry, "空腹度=10→瀕死ではないが空腹");
    test_priority(&mut cat, 9, 50, 50, 50, CatState::Dying, "空腹度=9→瀕死");
    test_priority(&mut cat, 30, 50, 50, 50, CatState::Normal, "空腹度=30→空腹ではない");
    test_priority(&mut cat, 29, 50, 50, 50, CatState::Hungry, "空腹度=29→空腹");
}

fn test_priority(cat: &mut Cat, hunger: i32, happiness: i32, cleanliness: i32, health: i32, expected: CatState, description: &str) {
    cat.set_status_for_test(hunger, happiness, cleanliness, health);
    let actual = cat.get_state();
    let result = if actual == expected { "✓" } else { "✗" };
    println!("{} {}: {:?} (H:{}/Ha:{}/C:{}/He:{})", 
             result, description, actual, hunger, happiness, cleanliness, health);
    if actual != expected {
        println!("  期待値: {:?}", expected);
    }
}