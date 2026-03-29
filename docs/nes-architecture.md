# NES アーキテクチャ概要

ref: https://www.nesdev.org/wiki/NES_reference_guide

## システム構成

- **CPU**: Ricoh 2A03 (MOS 6502改、Decimal Mode無効) @ 1.789773 MHz (NTSC)
- **PPU**: 256x240ピクセル、3PPUサイクル = 1CPUサイクル
- **APU**: CPU内蔵、5チャンネル (矩形波x2, 三角波, ノイズ, DMC)
- **RAM**: 2KB内蔵 + カートリッジRAM (最大8KB)

## CPUレジスタ

| レジスタ | サイズ | 用途 |
|---------|--------|------|
| A | 8bit | アキュムレータ |
| X | 8bit | インデックス |
| Y | 8bit | インデックス |
| SP | 8bit | スタックポインタ ($0100-$01FFを指す) |
| PC | 16bit | プログラムカウンタ |
| P | 8bit | ステータス (NV-BDIZC) |

### ステータスフラグ (P)

```
bit 7: N (Negative)    - 結果のbit7が1なら1
bit 6: V (Overflow)    - 符号付きオーバーフロー
bit 5: -               - 常に1
bit 4: B (Break)       - BRK/PHP時は1、割り込み時は0
bit 3: D (Decimal)     - NESでは無効
bit 2: I (IRQ Disable) - 1でIRQ禁止
bit 1: Z (Zero)        - 結果が0なら1
bit 0: C (Carry)       - キャリー/ボロー
```

## CPUメモリマップ

```
$0000-$07FF  2KB 内蔵RAM (ZeroPage: $00-$FF, Stack: $0100-$01FF)
$0800-$1FFF  ↑のミラー (3回繰り返し)
$2000-$2007  PPUレジスタ (8バイト)
$2008-$3FFF  ↑のミラー (8バイト毎に繰り返し)
$4000-$4017  APU・I/Oレジスタ
$4018-$401F  テストモード用 (通常無効)
$4020-$5FFF  カートリッジ拡張領域
$6000-$7FFF  カートリッジRAM (バッテリーバックアップ)
$8000-$FFFF  カートリッジPRG-ROM (32KB, Mapper制御)
```

### 割り込みベクタ (カートリッジが提供)

```
$FFFA-$FFFB  NMI
$FFFC-$FFFD  RESET
$FFFE-$FFFF  IRQ/BRK
```

## アドレッシングモード

| 記法 | モード | バイト数 | サイクル |
|------|--------|---------|---------|
| (暗黙) | Implicit | 1 | 可変 |
| A | Accumulator | 1 | 2 |
| #v | Immediate | 2 | 2 |
| d | Zero Page | 2 | 3 |
| d,x | Zero Page,X | 2 | 4 |
| d,y | Zero Page,Y | 2 | 4 |
| a | Absolute | 3 | 4 |
| a,x | Absolute,X | 3 | 4+ |
| a,y | Absolute,Y | 3 | 4+ |
| (a) | Indirect (JMP専用) | 3 | 5 |
| (d,x) | Indexed Indirect | 2 | 6 |
| (d),y | Indirect Indexed | 2 | 5+ |
| label | Relative (分岐) | 2 | 2+/3+ |

"+" = ページ境界跨ぎで+1サイクル
