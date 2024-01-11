use serde::{Deserialize, Serialize};

/// May be possible to do both mouse and keyboard functions at the same time? I'd need to see how the layout would work.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Function {
    Mouse(MouseButton, MouseFunction),
    Keyboard(MouseButton, KeyboardFunction),
}

impl Function {
    pub(crate) fn generate_string(&self) -> [u8; 9] {
        let mut string = [0u8; 9];

        match self {
            Function::Mouse(physical_button, emulate_function) => {
                string[0] = *physical_button as _;
                match emulate_function {
                    MouseFunction::Button(emulate_button, interval) => {
                        string[2] = 0x01;
                        string[3] = 0x01;
                        string[4] = *emulate_button as _;
                        if *interval > 0 {
                            string[2] = 0x0e;
                            string[3] = 0x03;
                            let data = interval.to_be_bytes();
                            string[5] = data[0];
                            string[6] = data[1];
                        }
                    },
                    MouseFunction::Sensitivity(s_func) => {
                        string[2] = 0x06;
                        match s_func {
                            SensitivityFunction::Clutch(x, y) => {
                                let x_val = x.to_be_bytes();
                                let y_val = y.to_be_bytes();
                                string[3] = 0x05;
                                string[4] = 0x05;
                                string[5] = x_val[0];
                                string[6] = x_val[1];
                                string[7] = y_val[0];
                                string[8] = y_val[1];
                            },
                            SensitivityFunction::CycleUpStage => {
                                string[3] = 0x01;
                                string[4] = 0x06;
                            },
                            SensitivityFunction::CycleDownStage => {
                                string[3] = 0x01;
                                string[4] = 0x07;
                            },
                            SensitivityFunction::StageUp => {
                                string[3] = 0x01;
                                string[4] = 0x01;
                            },
                            SensitivityFunction::StageDown => {
                                string[3] = 0x01;
                                string[4] = 0x02;
                            },
                        }
                    },
                }
            },
            Function::Keyboard(physical_button, keyboard_function) => {
                string[0] = *physical_button as _;
                string[2] = 0x02;
                string[3] = 0x02;
                string[4] = keyboard_function.modifiers.iter().fold(0x00, |acc, m| acc | *m as u8);
                string[5] = keyboard_function.key as _;
                if keyboard_function.turbo > 0 {
                    string[2] = 0x0d;
                    string[3] = 0x04;
                    let data = keyboard_function.turbo.to_be_bytes();
                    string[6] = data[0];
                    string[7] = data[1];
                }
            },
        }
        string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum MouseFunction {
    Button(MouseButton, u16), // Emulate a regular mouse button. Repeat action every N milliseconds. 0 for no repetition.
    Sensitivity(SensitivityFunction),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum MouseButton {
    LeftClick = 0x01,
    RightClick = 0x02,
    MiddleClick = 0x03,
    Mouse4 = 0x04,
    Mouse5 = 0x05,
    Scrollup = 0x09,
    Scrolldown = 0x0a,
    SensitivityStageUp = 0x0b,
    SensitivityStageDown = 0x0c,
    ScrollLeft = 0x34,
    ScrollRight = 0x35,
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
    Side12 = 0x4b
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) enum SensitivityFunction {
    Clutch(u16, u16), // Set specific sensitivity X, Y axis DPI values. Synapse allows 100 to 30000.
    CycleUpStage,
    CycleDownStage,
    StageUp,
    StageDown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KeyboardFunction {
    pub(crate) turbo: u16,
    pub(crate) key: UsbKbScanCode,
    pub(crate) modifiers: Vec<KeyboardModifier>
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum KeyboardModifier {
    LeftControl = 0x01,
    RightControl = 0x10,
    LeftShift = 0x02,
    RightShift = 0x20,
    LeftAlt = 0x04,
    RightAlt = 0x40,
    LeftGui = 0x08,
    RightGui = 0x80,
}

// Source: https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/scancode.doc
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum UsbKbScanCode {
    Disabled = 0x00, // Note 9; This is what synapse sets when you disable a mouse button.
    KeyboardErrorRollOver = 0x01, // Note 9
    KeyboardPOSTFail = 0x02, // Note 9
    KeyboardErrorUndefined = 0x03, // Note 9
    KeyboardA = 0x04, // and a Note 4
    KeyboardB = 0x05, // and b
    KeyboardC = 0x06, // and c Note 4
    KeyboardD = 0x07, // and d
    KeyboardE = 0x08, // and e
    KeyboardF = 0x09, // and f
    KeyboardG = 0x0A, // and g
    KeyboardH = 0x0B, // and h
    KeyboardI = 0x0C, // and i
    KeyboardJ = 0x0D, // and j
    KeyboardK = 0x0E, // and k
    KeyboardL = 0x0F, // and l
    KeyboardM = 0x10, // and m Note 4
    KeyboardN = 0x11, // and n
    KeyboardO = 0x12, // and o Note 4
    KeyboardP = 0x13, // and p Note 4
    KeyboardQ = 0x14, // and q Note 4
    KeyboardR = 0x15, // and r
    KeyboardS = 0x16, // and s Note 4
    KeyboardT = 0x17, // and t
    KeyboardU = 0x18, // and u
    KeyboardV = 0x19, // and v
    KeyboardW = 0x1A, // and w Note 4
    KeyboardX = 0x1B, // and x Note 4
    KeyboardY = 0x1C, // and y Note 4
    KeyboardZ = 0x1D, // and z Note 4
    Keyboard1 = 0x1E, // and ! Note 4
    Keyboard2 = 0x1F, // and @ Note 4
    Keyboard3 = 0x20, // and # Note 4
    Keyboard4 = 0x21, // and $ Note 4
    Keyboard5 = 0x22, // and % Note 4
    Keyboard6 = 0x23, // and ^ Note 4
    Keyboard7 = 0x24, // and & Note 4
    Keyboard8 = 0x25, // and * Note 4
    Keyboard9 = 0x26, // and ( Note 4
    Keyboard0 = 0x27, // and ) Note 4
    KeyboardEnter = 0x28, // Note 5
    KeyboardEscape = 0x29,
    KeyboardBackspace = 0x2A, // Note 13
    KeyboardTab = 0x2B,
    KeyboardSpacebar = 0x2C,
    KeyboardMinus = 0x2D, // and _ Note 4
    KeyboardEquals = 0x2E, // and + Note 4
    KeyboardLbracket = 0x2F, // [ and { Note 4
    KeyboardRbracket = 0x30, // ] and } Note 4
    KeyboardBackslash = 0x31, // and |
    KeyboardNonUSTilde = 0x32, // and # Note 2
    Keyboard = 0x33, // Note 4
    KeyboardQuote = 0x34, // and " Note 4
    KeyboardGrave = 0x35, // and Tilde Note 4
    KeyboardComma = 0x36, // and < Note 4
    KeyboardDot = 0x37, // and > Note 4
    KeyboardSlash = 0x38, // and ? Note 4
    KeyboardCapsLock = 0x39, // Note 11
    KeyboardF1 = 0x3A,
    KeyboardF2 = 0x3B,
    KeyboardF3 = 0x3C,
    KeyboardF4 = 0x3D,
    KeyboardF5 = 0x3E,
    KeyboardF6 = 0x3F,
    KeyboardF7 = 0x40,
    KeyboardF8 = 0x41,
    KeyboardF9 = 0x42,
    KeyboardF10 = 0x43,
    KeyboardF11 = 0x44,
    KeyboardF12 = 0x45,
    KeyboardPrintScreen = 0x46, // Note 1
    KeyboardScrollLock = 0x47, // Note 11
    KeyboardPause = 0x48, // Note 1
    KeyboardInsert = 0x49, // Note 1
    KeyboardHome = 0x4A, // Note 1
    KeyboardPageUp = 0x4B, // Note 1
    KeyboardDelete = 0x4C, // Note 1
    KeyboardEnd = 0x4D, // Note 1
    KeyboardPageDown = 0x4E, // Note 1
    KeyboardRightArrow = 0x4F, // Note 1
    KeyboardLeftArrow = 0x50, // Note 1
    KeyboardDownArrow = 0x51, // Note 1
    KeyboardUpArrow = 0x52, // Note 1
    KeypadNumLock = 0x53, // and Clear, Note 11
    KeypadSlash = 0x54, // Note 1
    KeypadAsterisk = 0x55,
    KeypadMinus = 0x56,
    KeypadPlus = 0x57,
    KeypadEnter = 0x58, // Note 5
    Keypad1 = 0x59, // and End
    Keypad2 = 0x5A, // and Down Arrow
    Keypad3 = 0x5B, // and PageDn
    Keypad4 = 0x5C, // and Left Arrow
    Keypad5 = 0x5D,
    Keypad6 = 0x5E, // and Right Arrow
    Keypad7 = 0x5F, // and Home
    Keypad8 = 0x60, // and Up Arrow
    Keypad9 = 0x61, // and PageUp
    Keypad0 = 0x62, // and Insert
    KeypadDot = 0x63, // and Delete
    KeyboardNonUSBackslash = 0x64, // Non-US \ and | Note 3 and 6
    KeyboardApplication = 0x65, // Note 10
    KeyboardPower = 0x66, // Note 9
    KeypadEquals = 0x67,
    KeyboardF13 = 0x68,
    KeyboardF14 = 0x69,
    KeyboardF15 = 0x6A,
    KeyboardF16 = 0x6B,
    KeyboardF17 = 0x6C,
    KeyboardF18 = 0x6D,
    KeyboardF19 = 0x6E,
    KeyboardF20 = 0x6F,
    KeyboardF21 = 0x70,
    KeyboardF22 = 0x71,
    KeyboardF23 = 0x72,
    KeyboardF24 = 0x73,
    KeyboardExecute = 0x74,
    KeyboardHelp = 0x75,
    KeyboardMenu = 0x76,
    KeyboardSelect = 0x77,
    KeyboardStop = 0x78,
    KeyboardAgain = 0x79,
    KeyboardUndo = 0x7A,
    KeyboardCut = 0x7B,
    KeyboardCopy = 0x7C,
    KeyboardPaste = 0x7D,
    KeyboardFind = 0x7E,
    KeyboardMute = 0x7F,
    KeyboardVolumeUp = 0x80,
    KeyboardVolumeDown = 0x81,
    KeyboardLockingCapsLock12 = 0x82, // Note 12
    KeyboardLockingNumLock12 = 0x83, // Note 12
    KeyboardLockingScrollLock = 0x84, // Note 12
    KeypadComma = 0x85,
    KeypadEqualSign = 0x86,
    KeyboardKanji1 = 0x87, // Note 15
    KeyboardKanji2 = 0x88, // Note 16
    KeyboardKanji3 = 0x89, // Note 17
    KeyboardKanji4 = 0x8A, // Note 18
    KeyboardKanji5 = 0x8B, // Note 19
    KeyboardKanji6 = 0x8C, // Note 20
    KeyboardKanji7 = 0x8D, // Note 21
    KeyboardKanji8 = 0x8E, // Note 22
    KeyboardKanji9 = 0x8F, // Note 22
    KeyboardLANG1 = 0x90, // Note 8
    KeyboardLANG2 = 0x91, // Note 8
    KeyboardLANG3 = 0x92, // Note 8
    KeyboardLANG4 = 0x93, // Note 8
    KeyboardLANG5 = 0x94, // Note 8
    KeyboardLANG6 = 0x95, // Note 8
    KeyboardLANG7 = 0x96, // Note 8
    KeyboardLANG8 = 0x97, // Note 8
    KeyboardLANG9 = 0x98, // Note 8
    KeyboardAlternateErase = 0x99, // Note 7
    KeyboardSysReqAttenti = 0x9A, // Note 1
    KeyboardCancel = 0x9B,
    KeyboardClear = 0x9C,
    KeyboardPrior = 0x9D,
    KeyboardReturn = 0x9E, // Note: NOT the same as return
    KeyboardSeparator = 0x9F,
    KeyboardOut = 0xA0,
    KeyboardOper = 0xA1,
    KeyboardClearAgain = 0xA2,
    KeyboardCrSelProps = 0xA3,
    KeyboardExSel = 0xA4,
    // 165-223	A5-DF	Reserved
    /// These behave as if just hitting the modifier key by itself
    KeyboardLControl = 0xE0,
    KeyboardLShift = 0xE1,
    KeyboardLAlt = 0xE2,
    KeyboardLGUI10 = 0xE3, // Note 10 and 23
    KeyboardRControl = 0xE4,
    KeyboardRShift = 0xE5,
    KeyboardRAlt = 0xE6,
    KeyboardRGUI10 = 0xE7, // Note 10 and 24
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
    // 1: Part of the checksum calculation. Basically this and byte 89 (byte 88 in the actual usb message) need to xor to zero or else the message (I think) is considered corrupted and is dropped.
    message[2] = 0x1f;
    // 1: Same as byte 1
    message[3] = 0;
    // 4-9: No idea what any of this is. Internal packet header or packet magic possibly? None of the numbers make any sense to me.
    let dunno = [0x00, 0x00, 0x0a, 0x02, 0x0c, 0x01];
    message[4..=9].clone_from_slice(&dunno);
    let func_bytes = func.generate_string();
    message[10..=18].clone_from_slice(&func_bytes);
    // iterate through everything after the checksum seed and xor all bytes together
    message[89] = message[3..].iter().fold(0, |acc, x| acc ^ x);
    message
}


#[cfg(test)]
mod test {
    use hex_literal::hex;

    use crate::model::{generate_message, KeyboardFunction, UsbKbScanCode, KeyboardModifier};

    use super::{Function, MouseButton, MouseFunction};

    #[test]
    fn validate_message_function() {
        let control = hex!("00001f0000000a020c010c0006010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::StageUp));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::StageDown));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::CycleUpStage));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006010700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000900");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::CycleDownStage));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050575300064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002e00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(30000, 100)));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050575307530000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(30000, 30000)));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c010c0006050503200320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b0002022235000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005900");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardGrave, modifiers: vec![KeyboardModifier::LeftShift, KeyboardModifier::RightShift] });
        assert_eq!(generate_message(&test), control);
        // assert_eq!(format!("{:02x?}", generate_message(&test)), format!("{:02x?}", control));

        let control = hex!("00001f0000000a020c014b0002020035000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007b00");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardGrave, modifiers: vec![] });
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202702e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardEquals, modifiers: vec![KeyboardModifier::RightShift, KeyboardModifier::RightAlt, KeyboardModifier::RightControl] });
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202402e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardEquals, modifiers: vec![KeyboardModifier::RightAlt] });
        assert_eq!(generate_message(&test), control);

        let control = hex!("00001f0000000a020c014b000202042e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006400");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardEquals, modifiers: vec![KeyboardModifier::LeftAlt] });
        assert_eq!(generate_message(&test), control);

        let lctrleq12 = hex!("00001f0000000a020c014b000202102e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007000");
        let test = Function::Keyboard(MouseButton::Side12, KeyboardFunction{ turbo: 0, key: UsbKbScanCode::KeyboardEquals, modifiers: vec![KeyboardModifier::LeftControl] });
        assert_eq!(generate_message(&test), control);

        let lctrleq12 = hex!("00001f0000000a020c014b000202012e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006100");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let rshfteq12 = hex!("00001f0000000a020c014b000202202e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let lshfteq12 = hex!("00001f0000000a020c014b000202022e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006200");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let default12 = hex!("00001f0000000a020c014b000202002e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006000");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let disable12 = hex!("00001f0000000a020c014b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004e00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let abytes01 = hex!("00001f0000000a020c01400002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004100");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let abytes11 = hex!("00001f0000000a020c014a0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let abytes11t20 = hex!("00001f0000000a020c014a000d040004003200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007000");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let abytes12 = hex!("00001f0000000a020c014b0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004a00");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let f1bytes12 = hex!("00001f0000000a020c014b000202003a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007400");
        let test = Function::Mouse(MouseButton::SensitivityStageDown, MouseFunction::Sensitivity(super::SensitivityFunction::Clutch(800, 800)));
        assert_eq!(generate_message(&test), control);

        let f12bytes12 = hex!("00001f0000000a020c014b0002020045000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b00");
         let f13bytes12 = hex!("00001f0000000a020c014b0002020068000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002600");
         let f24bytes12 = hex!("00001f0000000a020c014b0002020073000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003d00");
           let abytesrc = hex!("00001f0000000a020c01020002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300");
           let abytessl = hex!("00001f0000000a020c01340002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003500");
           let abytessr = hex!("00001f0000000a020c01350002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003400");
           let abytessu = hex!("00001f0000000a020c01090002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800");
           let abytessd = hex!("00001f0000000a020c010a0002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b00");
           let abytessc = hex!("00001f0000000a020c01030002020004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200");
          let m4bytes11 = hex!("00001f0000000a020c014a0001010400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
          let m5bytes12 = hex!("00001f0000000a020c014b0001010500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b00");
       let m5bytes12t01 = hex!("00001f0000000a020c014b000e030503e8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ad00");
          let rcbytesrc = hex!("00001f0000000a020c01020001010200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000500");
          let rcbyrct01 = hex!("00001f0000000a020c0102000e030203e8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e300");
          let rcbyrct02 = hex!("00001f0000000a020c0102000e030201f4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000fd00");
          let rcbyrct03 = hex!("00001f0000000a020c0102000e0302014d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004400");
          let rcbyrct04 = hex!("00001f0000000a020c0102000e030200fa000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f200");
          let rcbyrct20 = hex!("00001f0000000a020c0102000e030200320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003a00");
           let rcbytelc = hex!("00001f0000000a020c01020001010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600");
           let lcbyterc = hex!("00001f0000000a020c01010001010200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600");
           let lcbytelc = hex!("00001f0000000a020c01010001010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000500");
    
    }
}
