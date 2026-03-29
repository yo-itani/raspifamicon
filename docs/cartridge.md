# カートリッジ / Mapper

ref: https://www.nesdev.org/wiki/Mapper

## 概要

Mapperはカートリッジ上のハードウェアで、バンク切り替えを行う。
Mapperなしでは PRG-ROM 32KB + CHR-ROM 8KB が上限。

## 主要Mapper

| iNES # | 名前 | 特徴 |
|--------|------|------|
| 0 | NROM | バンク切替なし。16/32KB PRG, 8KB CHR。最初に実装する |
| 1 | MMC1 | 16/32KB PRG切替、4/8KB CHR切替、ミラリング制御、バッテリーRAM |
| 2 | UxROM | 16KB PRG切替、上位バンク固定、CHR-RAM |
| 3 | CNROM | PRG固定、8KB CHR切替 |
| 4 | MMC3 | 8KB PRG、1/2KB CHR、スキャンラインIRQ、ミラリング制御 |
| 7 | AxROM | 32KB PRG切替、シングルスクリーンミラリング |

## iNESフォーマット (ヘッダ16バイト)

```
bytes 0-3:   "NES\x1A" (マジックナンバー)
byte 4:      PRG-ROMサイズ (16KB単位)
byte 5:      CHR-ROMサイズ (8KB単位, 0=CHR-RAM)
byte 6:      フラグ6 (Mapper下位4bit、ミラリング、バッテリー、トレーナー)
byte 7:      フラグ7 (Mapper上位4bit、フォーマット)
bytes 8-15:  その他 (多くのROMでは0)
```

Mapper番号 = (flag7 & 0xF0) | (flag6 >> 4)
