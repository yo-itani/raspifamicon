// CPU (6502) — Ricoh 2A03 (Decimal Mode 無効)

use bitflags::bitflags;

// ---------------------------------------------------------------------------
// ステータスフラグ (P レジスタ)
// ---------------------------------------------------------------------------
//  bit 7: N (Negative)    — 結果の bit7 が 1 なら 1
//  bit 6: V (Overflow)    — 符号付きオーバーフロー
//  bit 5: - (Unused)      — 常に 1
//  bit 4: B (Break)       — BRK/PHP 時は 1、ハードウェア割り込み時は 0
//  bit 3: D (Decimal)     — NES では無効
//  bit 2: I (IRQ Disable) — 1 で IRQ 禁止
//  bit 1: Z (Zero)        — 結果が 0 なら 1
//  bit 0: C (Carry)       — キャリー/ボロー
// ---------------------------------------------------------------------------
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        const C      = 0b0000_0001; // Carry
        const Z      = 0b0000_0010; // Zero
        const I      = 0b0000_0100; // IRQ Disable
        const D      = 0b0000_1000; // Decimal (NES では無効)
        const B      = 0b0001_0000; // Break
        const UNUSED = 0b0010_0000; // 常に 1
        const V      = 0b0100_0000; // Overflow
        const N      = 0b1000_0000; // Negative
    }
}

impl Default for StatusFlags {
    /// 電源投入時: I=1, bit5=1 → 0x24
    fn default() -> Self {
        Self::I | Self::UNUSED
    }
}

// ---------------------------------------------------------------------------
// CPU 構造体
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Cpu {
    /// アキュムレータ
    pub a: u8,
    /// インデックスレジスタ X
    pub x: u8,
    /// インデックスレジスタ Y
    pub y: u8,
    /// スタックポインタ ($0100-$01FF を指す)
    pub sp: u8,
    /// プログラムカウンタ
    pub pc: u16,
    /// ステータスレジスタ (NV-BDIZC)
    pub status: StatusFlags,
}

impl Cpu {
    /// 電源投入時の初期状態を返す。
    ///
    /// - A, X, Y = 0
    /// - SP = 0xFD (RESET 時にスタックから 3 バイト消費)
    /// - PC = 0x0000 (後で RESET ベクタ $FFFC-$FFFD から設定)
    /// - Status = 0x24 (I=1, bit5=1)
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0x0000,
            status: StatusFlags::default(),
        }
    }
}

// ===========================================================================
// テスト
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Cpu::new() の初期値
    // -----------------------------------------------------------------------
    #[test]
    fn test_cpu_initial_state() {
        let cpu = Cpu::new();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.pc, 0x0000);
        assert_eq!(cpu.status.bits(), 0x24);
    }

    // -----------------------------------------------------------------------
    // StatusFlags のビット操作
    // -----------------------------------------------------------------------
    #[test]
    fn test_status_default() {
        let s = StatusFlags::default();
        assert!(s.contains(StatusFlags::I));
        assert!(s.contains(StatusFlags::UNUSED));
        assert!(!s.contains(StatusFlags::N));
        assert!(!s.contains(StatusFlags::Z));
        assert!(!s.contains(StatusFlags::C));
        assert!(!s.contains(StatusFlags::V));
        assert_eq!(s.bits(), 0x24);
    }

    #[test]
    fn test_status_set_and_clear() {
        let mut s = StatusFlags::default();

        // フラグを立てる
        s.insert(StatusFlags::N);
        s.insert(StatusFlags::C);
        assert!(s.contains(StatusFlags::N));
        assert!(s.contains(StatusFlags::C));

        // フラグを下ろす
        s.remove(StatusFlags::N);
        assert!(!s.contains(StatusFlags::N));
        assert!(s.contains(StatusFlags::C)); // C はそのまま
    }

    #[test]
    fn test_status_from_bits() {
        // 全フラグON (0xFF) → bit5 含め全ビット立つ
        let s = StatusFlags::from_bits_truncate(0xFF);
        assert!(s.contains(StatusFlags::N));
        assert!(s.contains(StatusFlags::V));
        assert!(s.contains(StatusFlags::B));
        assert!(s.contains(StatusFlags::D));
        assert!(s.contains(StatusFlags::I));
        assert!(s.contains(StatusFlags::Z));
        assert!(s.contains(StatusFlags::C));
        assert!(s.contains(StatusFlags::UNUSED));
    }

    #[test]
    fn test_status_individual_bits() {
        assert_eq!(StatusFlags::C.bits(), 0x01);
        assert_eq!(StatusFlags::Z.bits(), 0x02);
        assert_eq!(StatusFlags::I.bits(), 0x04);
        assert_eq!(StatusFlags::D.bits(), 0x08);
        assert_eq!(StatusFlags::B.bits(), 0x10);
        assert_eq!(StatusFlags::UNUSED.bits(), 0x20);
        assert_eq!(StatusFlags::V.bits(), 0x40);
        assert_eq!(StatusFlags::N.bits(), 0x80);
    }
}
