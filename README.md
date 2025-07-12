# neko-watch 🐱

ターミナルで動作する猫育成ゲーム

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![TUI](https://img.shields.io/badge/TUI-Terminal-blue?style=for-the-badge)

## 概要

neko-watchは、ターミナル上でASCIIアートの猫を育てるシンプルなペット育成ゲームです。
猫らしい行動と表情で癒しの体験を提供します。

## 特徴

- 🎨 **表情豊かなASCIIアート**: 9つの状態と睡眠モードで様々な表情
- ⏰ **リアルタイム育成**: 時間経過でステータスが変化
- 🎭 **猫らしい行動**: お風呂嫌い、餌やり後の満足感など
- 🌙 **睡眠システム**: 寝かせると10分間行動不可
- 🎵 **アニメーション**: まばたき、音符、睡眠効果など
- 🐾 **30分サイクル**: 適度な難易度でお世話が必要

## インストール

### 必要要件

- Rust 1.70+
- ターミナル（WSLでは動作しない可能性があります）

### ビルド

```bash
git clone https://github.com/YOUR_USERNAME/neko-watch.git
cd neko-watch
cargo build --release
```

## 使い方

### 基本操作

```bash
cargo run
```

### ゲーム操作

- **[1]** 餌をあげる - 空腹度+40、幸福度+10、清潔度-10
- **[2]** 遊ぶ - 幸福度+25、空腹度-15、清潔度-5  
- **[3]** お風呂 - 清潔度+60、幸福度-30、健康度-10（猫は水嫌い！）
- **[4]** 寝かせる - 健康度回復、10分間行動不可
- **[q]** 終了

### デバッグモード（開発ビルドのみ）

- **[d]** デバッグ情報表示（リアルタイム更新）
- **[h]** デバッグ情報非表示
- **[0]** 瀕死状態に設定
- **[8]** 病気状態に設定
- **[9]** 空腹状態に設定

## 猫の状態

### 状態一覧（優先順位順）

1. **瀕死** - いずれかのステータス < 10
2. **病気** - 健康度 < 20
3. **空腹** - 空腹度 < 30
4. **不潔** - 清潔度 < 30
5. **不機嫌** - 幸福度 < 30
6. **疲労** - 健康度 < 50 かつ 幸福度 < 50
7. **満腹** - 空腹度 > 90 かつ 餌やり直後30秒
8. **上機嫌** - 幸福度 > 80
9. **普通** - 上記以外
10. **睡眠中** - 寝かせるアクション後10分間（最優先）

### ステータス変化

- **空腹度**: -2.5/分
- **幸福度**: -2.0/分
- **清潔度**: -1.0/分
- **健康度**: -2.0/分（通常）、-4.0/分（他ステータス < 30時）

## 技術詳細

### 使用技術

- **言語**: Rust
- **TUI**: ratatui + crossterm
- **時間管理**: chrono

### アーキテクチャ

- `Cat`: ゲームロジックとステータス管理
- `App`: アプリケーション状態と入力処理
- `UI`: ターミナルUI描画
- 6行×18文字の固定レイアウト

## テスト

```bash
# 状態遷移テスト
cargo run --bin test_states

# 優先順位テスト  
cargo run --bin test_priority

# ASCIIアート行数テスト
cargo run --bin test_ascii

# 睡眠アニメーションテスト
cargo run --bin test_sleep
```

## 開発

### プロジェクト構造

```
src/
├── main.rs          # エントリーポイント
├── lib.rs           # ライブラリ設定
├── app.rs           # アプリケーション状態
├── cat.rs           # 猫のロジック
├── ui.rs            # UI描画
└── bin/             # テストプログラム
docs/
└── requirements.md  # 詳細仕様書
```

### コントリビューション

1. Forkしてください
2. FeatureBranchを作成してください (`git checkout -b feature/amazing-feature`)
3. 変更をcommitしてください (`git commit -m 'Add amazing feature'`)
4. BranchにPushしてください (`git push origin feature/amazing-feature`)
5. Pull Requestを開いてください

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 作者

- [nyagi](https://github.com/nyagi) - 初期開発

## 謝辞

- Rustコミュニティ
- ratatuiライブラリ開発者