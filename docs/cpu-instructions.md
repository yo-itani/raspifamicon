# 6502 CPU 命令リファレンス

ref: https://www.nesdev.org/wiki/6502_instructions

公式命令56種、オペコード151パターン。

**記号**: `*` = ページ境界跨ぎで+1サイクル、`**` = 分岐成立で+1(同ページ)/+2(別ページ)

---

## 1. Load / Store / Transfer

### LDA - Load Accumulator | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $A9 | 2 | 2 |
| Zero Page | $A5 | 2 | 3 |
| Zero Page,X | $B5 | 2 | 4 |
| Absolute | $AD | 3 | 4 |
| Absolute,X | $BD | 3 | 4* |
| Absolute,Y | $B9 | 3 | 4* |
| (Indirect,X) | $A1 | 2 | 6 |
| (Indirect),Y | $B1 | 2 | 5* |

### LDX - Load X Register | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $A2 | 2 | 2 |
| Zero Page | $A6 | 2 | 3 |
| Zero Page,Y | $B6 | 2 | 4 |
| Absolute | $AE | 3 | 4 |
| Absolute,Y | $BE | 3 | 4* |

### LDY - Load Y Register | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $A0 | 2 | 2 |
| Zero Page | $A4 | 2 | 3 |
| Zero Page,X | $B4 | 2 | 4 |
| Absolute | $AC | 3 | 4 |
| Absolute,X | $BC | 3 | 4* |

### STA - Store Accumulator | フラグ: なし

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $85 | 2 | 3 |
| Zero Page,X | $95 | 2 | 4 |
| Absolute | $8D | 3 | 4 |
| Absolute,X | $9D | 3 | 5 |
| Absolute,Y | $99 | 3 | 5 |
| (Indirect,X) | $81 | 2 | 6 |
| (Indirect),Y | $91 | 2 | 6 |

### STX - Store X Register | フラグ: なし

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $86 | 2 | 3 |
| Zero Page,Y | $96 | 2 | 4 |
| Absolute | $8E | 3 | 4 |

### STY - Store Y Register | フラグ: なし

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $84 | 2 | 3 |
| Zero Page,X | $94 | 2 | 4 |
| Absolute | $8C | 3 | 4 |

### Transfer命令 (すべて Implied, 1byte, 2cycles)

| 命令 | 動作 | オペコード | フラグ |
|------|------|-----------|--------|
| TAX | A → X | $AA | N, Z |
| TAY | A → Y | $A8 | N, Z |
| TXA | X → A | $8A | N, Z |
| TYA | Y → A | $98 | N, Z |
| TSX | SP → X | $BA | N, Z |
| TXS | X → SP | $9A | なし |

---

## 2. Arithmetic

### ADC - Add with Carry (A = A + M + C) | フラグ: N, V, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $69 | 2 | 2 |
| Zero Page | $65 | 2 | 3 |
| Zero Page,X | $75 | 2 | 4 |
| Absolute | $6D | 3 | 4 |
| Absolute,X | $7D | 3 | 4* |
| Absolute,Y | $79 | 3 | 4* |
| (Indirect,X) | $61 | 2 | 6 |
| (Indirect),Y | $71 | 2 | 5* |

### SBC - Subtract with Carry (A = A - M - (1-C)) | フラグ: N, V, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $E9 | 2 | 2 |
| Zero Page | $E5 | 2 | 3 |
| Zero Page,X | $F5 | 2 | 4 |
| Absolute | $ED | 3 | 4 |
| Absolute,X | $FD | 3 | 4* |
| Absolute,Y | $F9 | 3 | 4* |
| (Indirect,X) | $E1 | 2 | 6 |
| (Indirect),Y | $F1 | 2 | 5* |

### INC - Increment Memory | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $E6 | 2 | 5 |
| Zero Page,X | $F6 | 2 | 6 |
| Absolute | $EE | 3 | 6 |
| Absolute,X | $FE | 3 | 7 |

### DEC - Decrement Memory | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $C6 | 2 | 5 |
| Zero Page,X | $D6 | 2 | 6 |
| Absolute | $CE | 3 | 6 |
| Absolute,X | $DE | 3 | 7 |

### Increment/Decrement レジスタ (すべて Implied, 1byte, 2cycles) | フラグ: N, Z

| 命令 | 動作 | オペコード |
|------|------|-----------|
| INX | X + 1 → X | $E8 |
| INY | Y + 1 → Y | $C8 |
| DEX | X - 1 → X | $CA |
| DEY | Y - 1 → Y | $88 |

---

## 3. Logic

### AND - Bitwise AND | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $29 | 2 | 2 |
| Zero Page | $25 | 2 | 3 |
| Zero Page,X | $35 | 2 | 4 |
| Absolute | $2D | 3 | 4 |
| Absolute,X | $3D | 3 | 4* |
| Absolute,Y | $39 | 3 | 4* |
| (Indirect,X) | $21 | 2 | 6 |
| (Indirect),Y | $31 | 2 | 5* |

### ORA - Bitwise OR | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $09 | 2 | 2 |
| Zero Page | $05 | 2 | 3 |
| Zero Page,X | $15 | 2 | 4 |
| Absolute | $0D | 3 | 4 |
| Absolute,X | $1D | 3 | 4* |
| Absolute,Y | $19 | 3 | 4* |
| (Indirect,X) | $01 | 2 | 6 |
| (Indirect),Y | $11 | 2 | 5* |

### EOR - Bitwise Exclusive OR | フラグ: N, Z

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $49 | 2 | 2 |
| Zero Page | $45 | 2 | 3 |
| Zero Page,X | $55 | 2 | 4 |
| Absolute | $4D | 3 | 4 |
| Absolute,X | $5D | 3 | 4* |
| Absolute,Y | $59 | 3 | 4* |
| (Indirect,X) | $41 | 2 | 6 |
| (Indirect),Y | $51 | 2 | 5* |

### BIT - Bit Test (A AND M) | フラグ: N=M7, V=M6, Z=(A AND M)==0

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Zero Page | $24 | 2 | 3 |
| Absolute | $2C | 3 | 4 |

---

## 4. Shift / Rotate

### ASL - Arithmetic Shift Left (C ← [76543210] ← 0) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Accumulator | $0A | 1 | 2 |
| Zero Page | $06 | 2 | 5 |
| Zero Page,X | $16 | 2 | 6 |
| Absolute | $0E | 3 | 6 |
| Absolute,X | $1E | 3 | 7 |

### LSR - Logical Shift Right (0 → [76543210] → C) | フラグ: N(=0), Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Accumulator | $4A | 1 | 2 |
| Zero Page | $46 | 2 | 5 |
| Zero Page,X | $56 | 2 | 6 |
| Absolute | $4E | 3 | 6 |
| Absolute,X | $5E | 3 | 7 |

### ROL - Rotate Left (C ← [76543210] ← C) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Accumulator | $2A | 1 | 2 |
| Zero Page | $26 | 2 | 5 |
| Zero Page,X | $36 | 2 | 6 |
| Absolute | $2E | 3 | 6 |
| Absolute,X | $3E | 3 | 7 |

### ROR - Rotate Right (C → [76543210] → C) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Accumulator | $6A | 1 | 2 |
| Zero Page | $66 | 2 | 5 |
| Zero Page,X | $76 | 2 | 6 |
| Absolute | $6E | 3 | 6 |
| Absolute,X | $7E | 3 | 7 |

---

## 5. Branch (すべて Relative, 2bytes, 2cycles**)

| 命令 | 条件 | オペコード |
|------|------|-----------|
| BCC | C = 0 | $90 |
| BCS | C = 1 | $B0 |
| BEQ | Z = 1 | $F0 |
| BNE | Z = 0 | $D0 |
| BMI | N = 1 | $30 |
| BPL | N = 0 | $10 |
| BVS | V = 1 | $70 |
| BVC | V = 0 | $50 |

フラグ変更: なし

---

## 6. Jump / Subroutine

### JMP - Jump | フラグ: なし

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Absolute | $4C | 3 | 3 |
| Indirect | $6C | 3 | 5 |

**注意**: JMP Indirectにはハードウェアバグあり。ポインタのローバイトが$FFの場合、
ハイバイトは次のページ($xx+1, $00)ではなく同ページ($xx, $00)から読まれる。

### その他

| 命令 | 動作 | オペコード | Bytes | Cycles | フラグ |
|------|------|-----------|-------|--------|--------|
| JSR | サブルーチンへジャンプ (戻りアドレス-1をpush) | $20 | 3 | 6 | なし |
| RTS | サブルーチンから復帰 (アドレスをpull, +1) | $60 | 1 | 6 | なし |
| RTI | 割り込みから復帰 (Pをpull, PCをpull) | $40 | 1 | 6 | 全て(スタックから復元) |

---

## 7. Stack

| 命令 | 動作 | オペコード | Bytes | Cycles | フラグ |
|------|------|-----------|-------|--------|--------|
| PHA | Aをスタックにpush | $48 | 1 | 3 | なし |
| PHP | Pをスタックにpush (Bフラグ=1) | $08 | 1 | 3 | なし |
| PLA | スタックからAをpull | $68 | 1 | 4 | N, Z |
| PLP | スタックからPをpull | $28 | 1 | 4 | 全て |

---

## 8. Flag (すべて Implied, 1byte, 2cycles)

| 命令 | 動作 | オペコード |
|------|------|-----------|
| CLC | C ← 0 | $18 |
| SEC | C ← 1 | $38 |
| CLD | D ← 0 | $D8 |
| SED | D ← 1 | $F8 |
| CLI | I ← 0 | $58 |
| SEI | I ← 1 | $78 |
| CLV | V ← 0 | $B8 |

---

## 9. Compare

### CMP - Compare A (A - M) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $C9 | 2 | 2 |
| Zero Page | $C5 | 2 | 3 |
| Zero Page,X | $D5 | 2 | 4 |
| Absolute | $CD | 3 | 4 |
| Absolute,X | $DD | 3 | 4* |
| Absolute,Y | $D9 | 3 | 4* |
| (Indirect,X) | $C1 | 2 | 6 |
| (Indirect),Y | $D1 | 2 | 5* |

### CPX - Compare X (X - M) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $E0 | 2 | 2 |
| Zero Page | $E4 | 2 | 3 |
| Absolute | $EC | 3 | 4 |

### CPY - Compare Y (Y - M) | フラグ: N, Z, C

| モード | オペコード | Bytes | Cycles |
|--------|-----------|-------|--------|
| Immediate | $C0 | 2 | 2 |
| Zero Page | $C4 | 2 | 3 |
| Absolute | $CC | 3 | 4 |

---

## 10. System

| 命令 | 動作 | オペコード | Bytes | Cycles | フラグ |
|------|------|-----------|-------|--------|--------|
| BRK | ソフトウェア割り込み | $00 | 1* | 7 | I=1, B=1(push時) |
| NOP | 何もしない | $EA | 1 | 2 | なし |

*BRKは1バイト命令だがPCは+2される (パディングバイトあり)

---

## 実装時の注意点

1. **ページ境界ペナルティ**: Absolute,X / Absolute,Y / (Indirect),Y の読込命令で、アドレス計算がページ($100)を跨ぐと+1サイクル。書込命令は常にペナルティあり
2. **JMP Indirectバグ**: ポインタが$xxFFの時、ハイバイトは$xx00から読む (ページ跨ぎしない)
3. **BRKのパディング**: PC+2がスタックにpushされるため、BRKの次の1バイトはスキップされる
4. **PHP/BRKのBフラグ**: pushされるステータスのbit4は常に1。IRQ/NMIでは0
5. **NESではDecimal Modeは無効**: SED命令はDフラグを立てるが、ADC/SBCの動作には影響しない
