use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Cat {
    pub name: String,
    pub hunger: i32,      // 0-100 (100 = 満腹)
    pub happiness: i32,   // 0-100 (100 = 最高に幸せ)
    pub cleanliness: i32, // 0-100 (100 = 清潔)
    pub health: i32,      // 0-100 (100 = 健康)
    animation_frame: usize,
    frame_counter: u32,
    last_fed: Option<Instant>,
    sleep_until: Option<Instant>,
    // 浮動小数点で累積計算
    hunger_f: f64,
    happiness_f: f64,
    cleanliness_f: f64,
    health_f: f64,
}

impl Cat {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hunger: 80,
            happiness: 80,
            cleanliness: 80,
            health: 80,
            animation_frame: 0,
            frame_counter: 0,
            last_fed: None,
            sleep_until: None,
            hunger_f: 80.0,
            happiness_f: 80.0,
            cleanliness_f: 80.0,
            health_f: 80.0,
        }
    }

    pub fn update(&mut self, delta_seconds: f64) {
        let delta = delta_seconds / 60.0; // 分単位に変換
        
        // 浮動小数点で累積計算
        self.hunger_f = (self.hunger_f - delta * 2.5).max(0.0);
        self.happiness_f = (self.happiness_f - delta * 2.0).max(0.0);
        self.cleanliness_f = (self.cleanliness_f - delta * 1.0).max(0.0);
        
        // 健康度は他のステータスが30未満だと2倍の速度で減る
        let health_penalty = if self.hunger < 30 || self.happiness < 30 || self.cleanliness < 30 {
            4.0  // -4/分
        } else {
            2.0  // -2/分
        };
        self.health_f = (self.health_f - delta * health_penalty).max(0.0);
        
        // 整数値を更新
        self.hunger = self.hunger_f as i32;
        self.happiness = self.happiness_f as i32;
        self.cleanliness = self.cleanliness_f as i32;
        self.health = self.health_f as i32;
        
        // アニメーションフレームの更新
        self.frame_counter += 1;
        if self.frame_counter % 10 == 0 {  // 10フレームごとに更新
            self.animation_frame = (self.animation_frame + 1) % 4;
        }
    }

    pub fn feed(&mut self) {
        self.hunger_f = (self.hunger_f + 40.0).min(100.0);
        self.happiness_f = (self.happiness_f + 10.0).min(100.0);
        self.cleanliness_f = (self.cleanliness_f - 10.0).max(0.0);
        self.hunger = self.hunger_f as i32;
        self.happiness = self.happiness_f as i32;
        self.cleanliness = self.cleanliness_f as i32;
        self.last_fed = Some(Instant::now());
    }

    pub fn play(&mut self) {
        self.happiness_f = (self.happiness_f + 25.0).min(100.0);
        self.hunger_f = (self.hunger_f - 15.0).max(0.0);
        self.cleanliness_f = (self.cleanliness_f - 5.0).max(0.0);
        self.happiness = self.happiness_f as i32;
        self.hunger = self.hunger_f as i32;
        self.cleanliness = self.cleanliness_f as i32;
    }

    pub fn bathe(&mut self) {
        self.cleanliness_f = (self.cleanliness_f + 60.0).min(100.0);
        self.happiness_f = (self.happiness_f - 30.0).max(0.0);
        self.health_f = (self.health_f - 10.0).max(0.0);
        self.cleanliness = self.cleanliness_f as i32;
        self.happiness = self.happiness_f as i32;
        self.health = self.health_f as i32;
    }

    pub fn sleep(&mut self) {
        let recovery = (self.hunger_f + self.happiness_f + self.cleanliness_f) / 3.0;
        self.health_f = recovery;
        self.health = self.health_f as i32;
        self.sleep_until = Some(Instant::now() + Duration::from_secs(600)); // 10分間
    }

    pub fn get_mood(&self) -> CatMood {
        if self.health < 20 {
            CatMood::Sick
        } else if self.hunger < 20 || self.happiness < 20 {
            CatMood::Sad
        } else if self.health < 50 && self.happiness < 50 {
            CatMood::Sleepy
        } else if self.happiness > 80 && self.hunger > 70 {
            CatMood::Happy
        } else {
            CatMood::Normal
        }
    }

    pub fn get_ascii_art(&self) -> String {
        let frame = self.animation_frame;
        
        // 睡眠中は専用の表示を優先
        if self.is_sleeping() {
            return self.get_sleeping_animation(frame);
        }
        
        let state = self.get_state();
        match state {
            CatState::Dying => self.get_dying_animation(frame),
            CatState::Sick => self.get_sick_animation(frame),
            CatState::Hungry => self.get_hungry_animation(frame),
            CatState::Dirty => self.get_dirty_animation(frame),
            CatState::Unhappy => self.get_unhappy_animation(frame),
            CatState::Tired => self.get_tired_animation(frame),
            CatState::Happy => self.get_happy_animation(frame),
            CatState::Full => self.get_full_animation(frame),
            CatState::Normal => self.get_normal_animation(frame),
        }
    }
    
    pub fn get_state(&self) -> CatState {
        // 優先順位順にチェック
        if self.hunger < 10 || self.happiness < 10 || self.cleanliness < 10 || self.health < 10 {
            CatState::Dying
        } else if self.health < 20 {
            CatState::Sick
        } else if self.hunger < 30 {
            CatState::Hungry
        } else if self.cleanliness < 30 {
            CatState::Dirty
        } else if self.happiness < 30 {
            CatState::Unhappy
        } else if self.health < 50 && self.happiness < 50 {
            CatState::Tired
        } else if self.hunger > 90 && self.is_recently_fed() {
            CatState::Full
        } else if self.happiness > 80 {
            CatState::Happy
        } else {
            CatState::Normal
        }
    }
    
    fn get_normal_animation(&self, frame: usize) -> String {
        match frame % 4 {
            0 => "   /\\_/\\          \n  ( o.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n                  ".to_string(),
            1 => "   /\\_/\\          \n  ( -.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n                  ".to_string(),
            2 => "   /\\_/\\          \n  ( o.- )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n                  ".to_string(),
            _ => "   /\\_/\\          \n  ( o.o )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n                  ".to_string(),
        }
    }

    fn get_dying_animation(&self, _frame: usize) -> String {
        "   /\\_/\\          \n  ( x.x )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n       ...        ".to_string()
    }
    
    fn get_hungry_animation(&self, frame: usize) -> String {
        match frame % 4 {
            0 | 1 => "   /\\_/\\          \n  ( >.< )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \nおなかすいた...   ".to_string(),
            _ => "   /\\_/\\          \n  ( >.< )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n    グゥ～        ".to_string(),
        }
    }
    
    fn get_happy_animation(&self, frame: usize) -> String {
        match frame % 4 {
            0 | 2 => "   /\\_/\\          \n  ( ^.^ )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n       ♪         ".to_string(),
            _ => "   /\\_/\\          \n  ( ^.^ )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n      ♪♪         ".to_string(),
        }
    }
    
    fn get_dirty_animation(&self, frame: usize) -> String {
        let sweat = match frame % 4 {
            0 => ";;           ",
            1 => ";;;          ",
            2 => ";;;;         ",
            _ => ";;;;;;       ",
        };
        format!("   /\\_/\\          \n  ( >.< )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n    {}", sweat)
    }
    
    fn get_tired_animation(&self, frame: usize) -> String {
        let (eyes, sleep_effect) = match frame % 4 {
            0 => ("( -.- )", "Zzz          "),
            1 => ("( _._ )", "zZz          "),
            2 => ("( -.- )", "zzZ          "),
            _ => ("( _._ )", "ZzZ          "),
        };
        format!("   /\\_/\\          \n  {}         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n    {}", eyes, sleep_effect)
    }
    
    fn get_unhappy_animation(&self, _frame: usize) -> String {
        "   /\\_/\\          \n  ( -.~ )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n    ムスッ       ".to_string()
    }

    fn get_full_animation(&self, _frame: usize) -> String {
        "   /\\_/\\          \n  ( ^ω^ )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \nごちそうさま♪    ".to_string()
    }

    fn get_sick_animation(&self, frame: usize) -> String {
        match frame % 2 {
            0 => "   /\\_/\\          \n  ( @.@ )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n   ぐったり      ".to_string(),
            _ => "   /\\_/\\          \n  ( x.x )         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n   ぐったり      ".to_string(),
        }
    }
    
    fn get_sleeping_animation(&self, frame: usize) -> String {
        let (eyes, sleep_effect) = match frame % 4 {
            0 => ("( -.z )", "Zzz...       "),
            1 => ("( z.- )", "zzZ...       "),
            2 => ("( z.z )", "ZZZ...       "),
            _ => ("( -.z )", "zzz...       "),
        };
        format!("   /\\_/\\          \n  {}         \n   > ^ <     /)   \n  /|   |\\   / /   \n ( |___| ) /_/    \n   {}", eyes, sleep_effect)
    }
    
    fn is_recently_fed(&self) -> bool {
        if let Some(last_fed) = self.last_fed {
            last_fed.elapsed() < Duration::from_secs(30)
        } else {
            false
        }
    }
    
    pub fn can_perform_action(&self) -> bool {
        if let Some(sleep_until) = self.sleep_until {
            Instant::now() >= sleep_until
        } else {
            true
        }
    }
    
    pub fn is_sleeping(&self) -> bool {
        if let Some(sleep_until) = self.sleep_until {
            Instant::now() < sleep_until
        } else {
            false
        }
    }
    
    // テスト用関数: ステータスを直接設定
    #[cfg(debug_assertions)]
    pub fn set_status_for_test(&mut self, hunger: i32, happiness: i32, cleanliness: i32, health: i32) {
        self.hunger = hunger.clamp(0, 100);
        self.happiness = happiness.clamp(0, 100);
        self.cleanliness = cleanliness.clamp(0, 100);
        self.health = health.clamp(0, 100);
        self.hunger_f = self.hunger as f64;
        self.happiness_f = self.happiness as f64;
        self.cleanliness_f = self.cleanliness as f64;
        self.health_f = self.health as f64;
    }
    
    // テスト用関数: 現在の状態と全ステータスを表示
    #[cfg(debug_assertions)]
    pub fn debug_status(&self) -> String {
        let sleep_status = if let Some(sleep_until) = self.sleep_until {
            if Instant::now() < sleep_until {
                let remaining = sleep_until.duration_since(Instant::now());
                format!("睡眠中 (残り: {}秒)", remaining.as_secs())
            } else {
                "睡眠終了".to_string()
            }
        } else {
            "起きている".to_string()
        };
        
        let fed_status = if self.is_recently_fed() {
            if let Some(last_fed) = self.last_fed {
                let elapsed = last_fed.elapsed();
                format!("餌やり後 ({}秒経過)", elapsed.as_secs())
            } else {
                "満腹".to_string()
            }
        } else {
            "通常".to_string()
        };
        
        format!(
            "状態: {:?}\n空腹度: {}/100 ({:.2})\n幸福度: {}/100 ({:.2})\n清潔度: {}/100 ({:.2})\n健康度: {}/100 ({:.2})\nアニメーションフレーム: {}\n睡眠状態: {}\n餌やり状態: {}",
            self.get_state(),
            self.hunger, self.hunger_f,
            self.happiness, self.happiness_f,
            self.cleanliness, self.cleanliness_f,
            self.health, self.health_f,
            self.animation_frame,
            sleep_status,
            fed_status
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatMood {
    Normal,
    Happy,
    Sad,
    Sleepy,
    Sick,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatState {
    Dying,      // 瀕死
    Sick,       // 病気
    Hungry,     // 空腹
    Dirty,      // 不潔
    Unhappy,    // 不機嫌
    Tired,      // 疲労
    Happy,      // 上機嫌
    Full,       // 満腹
    Normal,     // 普通
}
