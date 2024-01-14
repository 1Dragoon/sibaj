use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Function {
    pub(crate) button: MouseButton,
    pub(crate) action: Action,
}

/// May be possible to do both mouse and keyboard functions at the same time? I'd need to see how the layout would work.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Action {
    Mouse(ButtonConfig),
    Keyboard(KeyPress),
    Sensitivity(SensitivityFunction),
    Hypershift,
    Disable,
}

impl Function {
    pub(crate) fn generate_string(&self) -> [u8; 9] {
        let mut string = [0u8; 9];
        string[0] = self.button as _;

        match &self.action {
            Action::Disable => {
                // Leave it blank :)
            }
            Action::Hypershift => {
                string[1] = 0x01;
                string[2] = 0x0c;
                string[3] = 0x01;
                string[4] = 0x01;
            }
            Action::Mouse(emulate_button) => {
                string[2] = 0x01;
                string[3] = 0x01;
                string[4] = emulate_button.button as _;
                if emulate_button.interval_ms > 0 {
                    string[2] = 0x0e;
                    string[3] = 0x03;
                    let data = emulate_button.interval_ms.to_be_bytes();
                    string[5] = data[0];
                    string[6] = data[1];
                }
            }
            Action::Keyboard(keyboard_function) => {
                string[2] = 0x02;
                string[3] = 0x02;
                string[4] = keyboard_function
                    .modifiers
                    .iter()
                    .fold(0x00, |acc, m| acc | *m as u8);
                string[5] = keyboard_function.key as _;
                if keyboard_function.interval_ms > 0 {
                    string[2] = 0x0d;
                    string[3] = 0x04;
                    let data = keyboard_function.interval_ms.to_be_bytes();
                    string[6] = data[0];
                    string[7] = data[1];
                }
            }
            Action::Sensitivity(s_func) => {
                string[2] = 0x06;
                match s_func {
                    SensitivityFunction::Clutch(sc) => {
                        let x_val = sc.x.to_be_bytes();
                        let y_val = sc.y.to_be_bytes();
                        string[3] = 0x05;
                        string[4] = 0x05;
                        string[5] = x_val[0];
                        string[6] = x_val[1];
                        string[7] = y_val[0];
                        string[8] = y_val[1];
                    }
                    SensitivityFunction::CycleUpStage => {
                        string[3] = 0x01;
                        string[4] = 0x06;
                    }
                    SensitivityFunction::CycleDownStage => {
                        string[3] = 0x01;
                        string[4] = 0x07;
                    }
                    SensitivityFunction::StageUp => {
                        string[3] = 0x01;
                        string[4] = 0x01;
                    }
                    SensitivityFunction::StageDown => {
                        string[3] = 0x01;
                        string[4] = 0x02;
                    }
                }
            }
        }
        string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ButtonConfig {
    button: MouseButton,
    /// Repeat this action every N milliseconds. Basically turbo, though with a more accurate description of what the mouse actually does.
    /// Examples: 50 repeats 20 times per second, 1000 repeats once every second, 1 repeats 1000 times per second. Max value here is 65535...if you wanted to for some reason...
    #[serde(default, skip_serializing_if = "u16::is_default")]
    interval_ms: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum MouseButton {
    LClick = 0x01,
    RClick = 0x02,
    MClick = 0x03,
    Mouse4 = 0x04,
    Mouse5 = 0x05,
    UScroll = 0x09,
    DScroll = 0x0a,
    SenStageUp = 0x0b,
    SenStageDown = 0x0c,
    LScroll = 0x34,
    RScroll = 0x35,
    Side1 = 0x40,
    Side2 = 0x41,
    Side3 = 0x42,
    Side4 = 0x43,
    Side5 = 0x44,
    Side6 = 0x45,
    Side7 = 0x46,
    Side8 = 0x47,
    Side9 = 0x48,
    Side10 = 0x49,
    Side11 = 0x4a,
    Side12 = 0x4b,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) struct SensitivityClutch {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) enum SensitivityFunction {
    Clutch(SensitivityClutch), // Set specific sensitivity X, Y axis DPI values. Synapse allows 100 to 30000.
    CycleUpStage,
    CycleDownStage,
    StageUp,
    StageDown,
}

trait IsDefault {
    fn is_default(&self) -> bool;
}

impl IsDefault for u16 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KeyPress {
    pub(crate) key: UsbKbScanCode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) modifiers: Vec<KeyMod>,
    /// Repeat this action every N milliseconds. Basically turbo, though with a more accurate description of what the mouse actually does.
    /// Examples: 50 repeats 20 times per second, 1000 repeats once every second, 1 repeats 1000 times per second. Max value here is 65535...if you wanted to for some reason...
    #[serde(default, skip_serializing_if = "u16::is_default")]
    pub(crate) interval_ms: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum KeyMod {
    LControl = 0x01,
    RControl = 0x10,
    LShift = 0x02,
    RShift = 0x20,
    LAlt = 0x04,
    RAlt = 0x40,
    LGui = 0x08,
    RGui = 0x80,
}

// Source: https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/scancode.doc
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum UsbKbScanCode {
    Disabled = 0x00, // Note 9; This is what synapse sets when you disable a mouse button.
    KeyboardErrorRollOver = 0x01, // Note 9
    KbPOSTFail = 0x02, // Note 9
    KbErrorUndefined = 0x03, // Note 9
    KbA = 0x04,      // and a Note 4
    KbB = 0x05,      // and b
    KbC = 0x06,      // and c Note 4
    KbD = 0x07,      // and d
    KbE = 0x08,      // and e
    KbF = 0x09,      // and f
    KbG = 0x0A,      // and g
    KbH = 0x0B,      // and h
    KbI = 0x0C,      // and i
    KbJ = 0x0D,      // and j
    KbK = 0x0E,      // and k
    KbL = 0x0F,      // and l
    KbM = 0x10,      // and m Note 4
    KbN = 0x11,      // and n
    KbO = 0x12,      // and o Note 4
    KbP = 0x13,      // and p Note 4
    KbQ = 0x14,      // and q Note 4
    KbR = 0x15,      // and r
    KbS = 0x16,      // and s Note 4
    KbT = 0x17,      // and t
    KbU = 0x18,      // and u
    KbV = 0x19,      // and v
    KbW = 0x1A,      // and w Note 4
    KbX = 0x1B,      // and x Note 4
    KbY = 0x1C,      // and y Note 4
    KbZ = 0x1D,      // and z Note 4
    Kb1 = 0x1E,      // and ! Note 4
    Kb2 = 0x1F,      // and @ Note 4
    Kb3 = 0x20,      // and # Note 4
    Kb4 = 0x21,      // and $ Note 4
    Kb5 = 0x22,      // and % Note 4
    Kb6 = 0x23,      // and ^ Note 4
    Kb7 = 0x24,      // and & Note 4
    Kb8 = 0x25,      // and * Note 4
    Kb9 = 0x26,      // and ( Note 4
    Kb0 = 0x27,      // and ) Note 4
    KbEnter = 0x28,  // Note 5
    KbEscape = 0x29,
    KbBackspace = 0x2A, // Note 13
    KbTab = 0x2B,
    KbSpacebar = 0x2C,
    KbMinus = 0x2D,      // and _ Note 4
    KbEquals = 0x2E,     // and + Note 4
    KbLbracket = 0x2F,   // [ and { Note 4
    KbRbracket = 0x30,   // ] and } Note 4
    KbBackslash = 0x31,  // and |
    KbNonUSTilde = 0x32, // and # Note 2
    Keybard = 0x33,      // Note 4
    KbQuote = 0x34,      // and " Note 4
    KbGrave = 0x35,      // and Tilde Note 4
    KbComma = 0x36,      // and < Note 4
    KbDot = 0x37,        // and > Note 4
    KbSlash = 0x38,      // and ? Note 4
    KbCapsLock = 0x39,   // Note 11
    KbF1 = 0x3A,
    KbF2 = 0x3B,
    KbF3 = 0x3C,
    KbF4 = 0x3D,
    KbF5 = 0x3E,
    KbF6 = 0x3F,
    KbF7 = 0x40,
    KbF8 = 0x41,
    KbF9 = 0x42,
    KbF10 = 0x43,
    KbF11 = 0x44,
    KbF12 = 0x45,
    KbPrintScreen = 0x46, // Note 1
    KbScrollLock = 0x47,  // Note 11
    KbPause = 0x48,       // Note 1
    KbInsert = 0x49,      // Note 1
    KbHome = 0x4A,        // Note 1
    KbPageUp = 0x4B,      // Note 1
    KbDelete = 0x4C,      // Note 1
    KbEnd = 0x4D,         // Note 1
    KbPageDown = 0x4E,    // Note 1
    KbRightArrow = 0x4F,  // Note 1
    KbLeftArrow = 0x50,   // Note 1
    KbDownArrow = 0x51,   // Note 1
    KbUpArrow = 0x52,     // Note 1
    KeypadNumLock = 0x53, // and Clear, Note 11
    KeypadSlash = 0x54,   // Note 1
    KeypadAsterisk = 0x55,
    KeypadMinus = 0x56,
    KeypadPlus = 0x57,
    KeypadEnter = 0x58, // Note 5
    Keypad1 = 0x59,     // and End
    Keypad2 = 0x5A,     // and Down Arrow
    Keypad3 = 0x5B,     // and PageDn
    Keypad4 = 0x5C,     // and Left Arrow
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,          // and Right Arrow
    Keypad7 = 0x5F,          // and Home
    Keypad8 = 0x60,          // and Up Arrow
    Keypad9 = 0x61,          // and PageUp
    Keypad0 = 0x62,          // and Insert
    KeypadDot = 0x63,        // and Delete
    KbNonUSBackslash = 0x64, // Non-US \ and | Note 3 and 6
    KbApplication = 0x65, // The key left of the right control key and opens a context menu in windows, aka "compose", Note 10
    KbPower = 0x66,       // Note 9
    KeypadEquals = 0x67,
    KbF13 = 0x68,
    KbF14 = 0x69,
    KbF15 = 0x6A,
    KbF16 = 0x6B,
    KbF17 = 0x6C,
    KbF18 = 0x6D,
    KbF19 = 0x6E,
    KbF20 = 0x6F,
    KbF21 = 0x70,
    KbF22 = 0x71,
    KbF23 = 0x72,
    KbF24 = 0x73,
    KbExecute = 0x74,
    KbHelp = 0x75,
    KbMenu = 0x76,
    KbSelect = 0x77,
    KbStop = 0x78,
    KbAgain = 0x79,
    KbUndo = 0x7A,
    KbCut = 0x7B,
    KbCopy = 0x7C,
    KbPaste = 0x7D,
    KbFind = 0x7E,
    KbMute = 0x7F,
    KbVolumeUp = 0x80,
    KbVolumeDown = 0x81,
    KbLockingCapsLock12 = 0x82, // Note 12
    KbLockingNumLock12 = 0x83,  // Note 12
    KbLockingScrollLock = 0x84, // Note 12
    KeypadComma = 0x85,
    KeypadEqualSign = 0x86,
    KbKanji1 = 0x87,         // Note 15
    KbKanji2 = 0x88,         // Note 16
    KbKanji3 = 0x89,         // Note 17
    KbKanji4 = 0x8A,         // Note 18
    KbKanji5 = 0x8B,         // Note 19
    KbKanji6 = 0x8C,         // Note 20
    KbKanji7 = 0x8D,         // Note 21
    KbKanji8 = 0x8E,         // Note 22
    KbKanji9 = 0x8F,         // Note 22
    KbLANG1 = 0x90,          // Note 8
    KbLANG2 = 0x91,          // Note 8
    KbLANG3 = 0x92,          // Note 8
    KbLANG4 = 0x93,          // Note 8
    KbLANG5 = 0x94,          // Note 8
    KbLANG6 = 0x95,          // Note 8
    KbLANG7 = 0x96,          // Note 8
    KbLANG8 = 0x97,          // Note 8
    KbLANG9 = 0x98,          // Note 8
    KbAlternateErase = 0x99, // Note 7
    KbSysReqAttenti = 0x9A,  // Note 1
    KbCancel = 0x9B,
    KbClear = 0x9C,
    KbPrior = 0x9D,
    KbReturn = 0x9E, // Note: NOT the same as enter! https://www.howtogeek.com/808178/whats-the-difference-between-the-enter-and-return-keys/
    KbSeparator = 0x9F,
    KbOut = 0xA0,
    KbOper = 0xA1,
    KbClearAgain = 0xA2,
    KbCrSelProps = 0xA3,
    KbExSel = 0xA4,
    // 165-223	A5-DF	Reserved
    /// These (should) behave as if just hitting the modifier key by itself
    KbLControl = 0xE0,
    KbLShift = 0xE1,
    KbLAlt = 0xE2,
    KbLGUI = 0xE3, // Note 10 and 23
    KbRControl = 0xE4,
    KbRShift = 0xE5,
    KbRAlt = 0xE6,
    KbRGUI = 0xE7, // Note 10 and 24
                   // 232-255	E8-FF	Reserved
}

// 1.	Usage of keys is not modified by the state of the Control, Alt, Shift or Num Lock keys. That is, a key does not send extra codes to compensate for the state of any Control, Alt, Shift or Num Lock keys.
// 2.	Typical language mappings: US: \| Belg: µ`£ FrCa: <}> Dan:’* Dutch: <> Fren:*µ Ger: #’ Ital: ù§ LatAm: }`] Nor:,* Span:}Ç Swed: ,* Swiss: $£ UK: #~.
// 3.	Typical language mappings: Belg:<\> FrCa:«°» Dan:<\> Dutch:]|[ Fren:<> Ger:<|> Ital:<> LatAm:<> Nor:<> Span:<> Swed:<|> Swiss:<\> UK:\| Brazil: \|.
// 4.	Typically remapped for other languages in the host system.
// 5.	Keyboard Enter and Keypad Enter generate different Usage codes.
// 6.	Typically near the Left-Shift key in AT-102 implementations.
// 7.	Example, Erase-Eaze™ key.
// 8.	Reserved for language-specific functions, such as Front End Processors and Input Method Editors.
// 9.	Reserved for typical keyboard status or keyboard errors. Sent as a member of the keyboard array. Not a physical key.
// 10.	Microsoft Windows key for Microsoft Windows 95 and “Compose.”
// 11.	Implemented as a non-locking key; sent as member of an array.
// 12.	Implemented as a locking key; sent as a toggle button. Available for legacy support; however, most systems should use the non-locking version of this key.
// 13.	Backs up the cursor one position, deleting a character as it goes.
// 14.	Deletes one character without changing position.
// 15.	See page 35 of the reference document
// 16.	See page 35 of the reference document
// 17.	See page 35 of the reference document
// 18.	See page 35 of the reference document
// 19.	See page 35 of the reference document
// 20.	See page 35 of the reference document
// 21.	Toggle Double-Byte/Single-Byte mode.
// 22.	Undefined, available for other Front End Language Processors.
// 23.	Windowing environment key, examples are Microsoft Left Win key, Macintosh Left Apple key, Sun Left Meta key
// 24.	Windowing environment key, examples are Microsoft Right Win key, Macintosh Right Apple key, Sun Right Meta key.

pub(crate) fn generate_message(func: &Function) -> [u8; 91] {
    let mut message = [0u8; 91];
    // Byte number: Comment
    // 0: Report ID: Needed strictly for the API, doesn't actually get sent to the mouse in this position, so everything else is basically off by one.
    message[0] = 0;
    // 1: I think either this or 3 might be an extended checksum
    message[1] = 0;
    // 2: Seems to be part of the checksum calculation. Basically this and byte 89 (byte 88 in the actual usb message) need to xor to zero or else the message is (I think) considered corrupted and is dropped.
    message[2] = 0x1f;
    // 3: Same as byte 1
    message[3] = 0;
    // 4-9: No idea what any of this is. Internal packet header or packet magic possibly? None of the numbers make any sense to me.
    let dunno = [0x00, 0x00, 0x0a, 0x02, 0x0c, 0x01];
    message[4..=9].clone_from_slice(&dunno);
    let func_bytes = func.generate_string();
    // 10-18: Pretty much the meat of the payload. Basically tells the mouse what button to bind to what action.
    message[10..=18].clone_from_slice(&func_bytes);
    // iterate through everything after the checksum seed and xor all bytes together
    message[89] = message[3..].iter().fold(0, |acc, x| acc ^ x);
    message
}

#[cfg(test)]
mod test {
    use super::{
        generate_message, Action, ButtonConfig, Function, KeyMod, KeyPress, MouseButton,
        SensitivityClutch, SensitivityFunction, UsbKbScanCode,
    };
    use hex_literal::hex;

    #[test]
    fn validate_message_function() {
        let control = hex!("00001f0000000a020c010c0006010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(SensitivityFunction::StageUp),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(SensitivityFunction::StageDown),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(SensitivityFunction::CycleUpStage),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000900");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(SensitivityFunction::CycleDownStage),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050575300064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002e00");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(SensitivityFunction::Clutch(SensitivityClutch {
                x: 30000,
                y: 100,
            })),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050575307530000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(super::SensitivityFunction::Clutch(SensitivityClutch {
                x: 30000,
                y: 30000,
            })),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050503200320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function {
            button: MouseButton::SenStageDown,
            action: Action::Sensitivity(super::SensitivityFunction::Clutch(SensitivityClutch {
                x: 800,
                y: 800,
            })),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002022235000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005900");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbGrave,
                modifiers: vec![KeyMod::LShift, KeyMod::RShift],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002020035000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007b00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbGrave,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202702e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::RShift, KeyMod::RAlt, KeyMod::RControl],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202ff2e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009f00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![
                    KeyMod::RShift,
                    KeyMod::RAlt,
                    KeyMod::RControl,
                    KeyMod::RGui,
                    KeyMod::LShift,
                    KeyMod::LAlt,
                    KeyMod::LControl,
                    KeyMod::LGui,
                ],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202402e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::RAlt],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202042e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006400");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::LAlt],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202102e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007000");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::RControl],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202202e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::RShift],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202022e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006200");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![KeyMod::LShift],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202002e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006000");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbEquals,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004e00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Disable,
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01400002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004100");
        let test = Function {
            button: MouseButton::Side1,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014a0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
        let test = Function {
            button: MouseButton::Side11,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014a000d040004003200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007000");
        let test = Function {
            button: MouseButton::Side11,
            action: Action::Keyboard(KeyPress {
                interval_ms: 50,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004a00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202003a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007400");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbF1,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002020045000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbF12,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002020068000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002600");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbF13,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002020073000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003d00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbF24,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01020002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01340002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003500");
        let test = Function {
            button: MouseButton::LScroll,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01350002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003400");
        let test = Function {
            button: MouseButton::RScroll,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01090002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800");
        let test = Function {
            button: MouseButton::UScroll,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010a0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b00");
        let test = Function {
            button: MouseButton::DScroll,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01030002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200");
        let test = Function {
            button: MouseButton::MClick,
            action: Action::Keyboard(KeyPress {
                interval_ms: 0,
                key: UsbKbScanCode::KbA,
                modifiers: vec![],
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014a0001010400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
        let test = Function {
            button: MouseButton::Side11,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::Mouse4,
                interval_ms: 0,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0001010500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::Mouse5,
                interval_ms: 0,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000e030503e8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ad00");
        let test = Function {
            button: MouseButton::Side12,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::Mouse5,
                interval_ms: 1000,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01020001010200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000500");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 0,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0102000e030203e8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e300");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 1000,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0102000e030201f4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000fd00");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 500,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0102000e0302014d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004400");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 333,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0102000e030200fa000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f200");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 250,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0102000e030200320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003a00");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::RClick,
                interval_ms: 50,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c01020001010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600");
        let test = Function {
            button: MouseButton::RClick,
            action: Action::Mouse(ButtonConfig {
                button: MouseButton::LClick,
                interval_ms: 0,
            }),
        };
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c0140010c010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004800");
        let test = Function {
            button: MouseButton::Side1,
            action: Action::Hypershift,
        };
        assert_eq!(generate_message(&test), control);
    }
}
