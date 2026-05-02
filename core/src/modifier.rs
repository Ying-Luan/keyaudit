use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModFlags(u32);

impl ModFlags {
    /// alt modifier flag
    const MOD_ALT: Self = Self(0x01);
    /// ctrl modifier flag
    const MOD_CTRL: Self = Self(0x02);
    /// shift modifier flag
    const MOD_SHIFT: Self = Self(0x04);
    /// win modifier flag
    const MOD_WIN: Self = Self(0x08);

    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl std::ops::BitOr for ModFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl From<ModFlags> for Vec<String> {
    fn from(flags: ModFlags) -> Self {
        let mut mods = Vec::new();

        if flags.contains(ModFlags::MOD_ALT) {
            mods.push("Alt".into());
        }
        if flags.contains(ModFlags::MOD_CTRL) {
            mods.push("Ctrl".into());
        }
        if flags.contains(ModFlags::MOD_SHIFT) {
            mods.push("Shift".into());
        }
        if flags.contains(ModFlags::MOD_WIN) {
            mods.push("Win".into());
        }

        mods
    }
}

/// A list of common modifier combinations and their string representations.
pub const MOD_COMBOS: &[ModFlags] = &[
    ModFlags::MOD_ALT,
    ModFlags::MOD_CTRL,
    ModFlags::MOD_SHIFT,
    ModFlags::MOD_WIN,
    ModFlags(ModFlags::MOD_CTRL.0 | ModFlags::MOD_ALT.0),
    ModFlags(ModFlags::MOD_CTRL.0 | ModFlags::MOD_SHIFT.0),
    ModFlags(ModFlags::MOD_ALT.0 | ModFlags::MOD_SHIFT.0),
    ModFlags(ModFlags::MOD_WIN.0 | ModFlags::MOD_CTRL.0),
    ModFlags(ModFlags::MOD_CTRL.0 | ModFlags::MOD_ALT.0 | ModFlags::MOD_SHIFT.0),
];
