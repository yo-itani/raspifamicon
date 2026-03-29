# プロジェクト構成

## ディレクトリ構造

```
raspifamicon/
├── CLAUDE.md              # エージェント向け最小方針 (docs/へのポインタ)
├── docs/                  # NES技術資料・開発ガイド
│   ├── nes-architecture.md
│   ├── ppu.md
│   ├── apu.md
│   ├── cartridge.md
│   ├── controller.md
│   ├── project-structure.md   # ← このファイル
│   └── development-guide.md
├── src/
│   ├── main.rs            # エントリポイント・エミュレーションループ
│   ├── cpu.rs             # 6502 CPU (レジスタ、命令デコード・実行)
│   ├── bus.rs             # メモリバス (CPUメモリマップの実装、各デバイスへのルーティング)
│   ├── ppu.rs             # PPU (レンダリング、VBlank、スプライト)
│   ├── apu.rs             # APU (音声チャンネル、ミキサー)
│   ├── cartridge.rs       # ROMファイル読み込み、Mapperトレイト・実装
│   └── controller.rs      # コントローラ入力 (ストローブ・シリアル読取)
└── tests/                 # 統合テスト (nestest検証など)
```

## モジュール責務

| モジュール | 責務 | 依存先 |
|-----------|------|--------|
| `cpu` | 命令フェッチ・デコード・実行、レジスタ管理、割り込み | `bus` (メモリR/W) |
| `bus` | アドレスに応じたデバイスへのR/Wルーティング | `ppu`, `apu`, `cartridge`, `controller` |
| `ppu` | スキャンラインレンダリング、VRAM管理、NMI発火 | なし (bus経由でアクセスされる) |
| `apu` | 音声波形生成、フレームカウンタ | なし (bus経由でアクセスされる) |
| `cartridge` | iNES解析、PRG/CHR-ROMバンク管理、Mapper実装 | なし |
| `controller` | ボタン状態のラッチ・シリアル読取 | なし |
| `main` | SDL2初期化、エミュレーションループ、入出力統合 | 全モジュール |

## データフロー

```
main (ループ)
  │
  ├─→ cpu.step()
  │     └─→ bus.read() / bus.write()
  │           ├─→ RAM (内蔵2KB)
  │           ├─→ ppu (レジスタR/W)
  │           ├─→ apu (レジスタW)
  │           ├─→ cartridge (PRG-ROM読込)
  │           └─→ controller (ボタン読取)
  │
  ├─→ ppu.step() (CPUサイクルx3)
  │     └─→ フレームバッファ生成 → SDL2描画
  │
  └─→ apu.step()
        └─→ サンプル生成 → SDL2オーディオ
```
