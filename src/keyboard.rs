use bevy::prelude::*;

pub const BINDINGS: usize = 7;

#[derive(Component)]
pub struct KeyboardBindings(pub [KeyCode; BINDINGS]);

impl Default for KeyboardBindings {
    fn default() -> KeyboardBindings {
        KeyboardBindings([
            KeyCode::O,
            KeyCode::L,
            KeyCode::K,
            KeyCode::M,
            KeyCode::R,
            KeyCode::Space,
            KeyCode::P,
        ])
    }
}

pub fn spawn_bindings(mut commands: Commands) {
    commands.spawn(KeyboardBindings::default());
}

impl KeyboardBindings {
    // fn get(&self) -> &[KeyCode] {
    //     &self.0
    // }

    pub fn accelerate(&self) -> KeyCode {
        self.0[0]
    }

    pub fn decelerate(&self) -> KeyCode {
        self.0[1]
    }

    pub fn rotate_left(&self) -> KeyCode {
        self.0[2]
    }

    pub fn rotate_right(&self) -> KeyCode {
        self.0[3]
    }

    pub fn fire(&self) -> KeyCode {
        self.0[4]
    }

    pub fn camera(&self) -> KeyCode {
        self.0[5]
    }

    pub fn pause(&self) -> KeyCode {
        self.0[6]
    }
}

pub const KeyCodeString: [&str; 163] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "Escape",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
    "Snapshot",
    "Scroll",
    "Pause",
    "Insert",
    "Home",
    "Delete",
    "End",
    "PageDown",
    "PageUp",
    "Left",
    "Up",
    "Right",
    "Down",
    "Back",
    "Return",
    "Space",
    "Compose",
    "Caret",
    "Numlock",
    "Numpad0",
    "Numpad1",
    "Numpad2",
    "Numpad3",
    "Numpad4",
    "Numpad5",
    "Numpad6",
    "Numpad7",
    "Numpad8",
    "Numpad9",
    "AbntC1",
    "AbntC2",
    "NumpadAdd",
    "Apostrophe",
    "Apps",
    "Asterisk",
    "Plus",
    "At",
    "Ax",
    "Backslash",
    "Calculator",
    "Capital",
    "Colon",
    "Comma",
    "Convert",
    "NumpadDecimal",
    "NumpadDivide",
    "Equals",
    "Grave",
    "Kana",
    "Kanji",
    "LAlt",
    "LBracket",
    "LControl",
    "LShift",
    "LWin",
    "Mail",
    "MediaSelect",
    "MediaStop",
    "Minus",
    "NumpadMultiply",
    "Mute",
    "MyComputer",
    "NavigateForward",
    "NavigateBackward",
    "NextTrack",
    "NoConvert",
    "NumpadComma",
    "NumpadEnter",
    "NumpadEquals",
    "Oem102",
    "Period",
    "PlayPause",
    "Power",
    "PrevTrack",
    "RAlt",
    "RBracket",
    "RControl",
    "RShift",
    "RWin",
    "Semicolon",
    "Slash",
    "Sleep",
    "Stop",
    "NumpadSubtract",
    "Sysrq",
    "Tab",
    "Underline",
    "Unlabeled",
    "VolumeDown",
    "VolumeUp",
    "Wake",
    "WebBack",
    "WebFavorites",
    "WebForward",
    "WebHome",
    "WebRefresh",
    "WebSearch",
    "WebStop",
    "Yen",
    "Copy",
    "Paste",
    "Cut",
];
