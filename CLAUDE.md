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

## コード規約

- 各コンポーネント (cpu, ppu, apu, bus, cartridge) はモジュール分離
- NES固有の技術情報は `docs/` 配下を参照
