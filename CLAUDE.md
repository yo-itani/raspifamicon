# raspifamicon

Rust製NESエミュレータ。Raspberry Pi 4 + SDL2がターゲット。

## 実装方針

- 小さく段階的に進める（学習目的）
- フェーズ: CPU → メモリマップ/ROM読込 → PPU → APU → 入力 → Mapper拡張
- テストROM (nestest.nes) でCPUの正しさを検証してからPPUに進む

## 技術スタック

- 言語: Rust
- 描画: SDL2 (`sdl2` crate)
- ターゲット: Raspberry Pi 4 (クロスコンパイル) / macOS (開発用)

## ドキュメント

- `docs/development-guide.md` — 実装方針、テスト戦略、ブランチ運用、コード規約
- `docs/project-structure.md` — モジュール構成、責務、データフロー
- `docs/nes-architecture.md` — CPU レジスタ、メモリマップ、アドレッシングモード
- `docs/cpu-instructions.md` — 6502全命令オペコード表、サイクル数、実装注意点
- `docs/ppu.md` / `docs/apu.md` / `docs/cartridge.md` / `docs/controller.md` — 各コンポーネント仕様
