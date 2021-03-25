use acute_input::{Key, MouseButton, State};
use winit::event::{ElementState, MouseButton as WinitMouseButton, VirtualKeyCode};

pub fn convert_key(key: VirtualKeyCode) -> Key {
    match key {
        VirtualKeyCode::Key1 => Key::Key1,
        VirtualKeyCode::Key2 => Key::Key2,
        VirtualKeyCode::Key3 => Key::Key3,
        VirtualKeyCode::Key4 => Key::Key4,
        VirtualKeyCode::Key5 => Key::Key5,
        VirtualKeyCode::Key6 => Key::Key6,
        VirtualKeyCode::Key7 => Key::Key7,
        VirtualKeyCode::Key8 => Key::Key8,
        VirtualKeyCode::Key9 => Key::Key9,
        VirtualKeyCode::Key0 => Key::Key0,
        VirtualKeyCode::A => Key::A,
        VirtualKeyCode::B => Key::B,
        VirtualKeyCode::C => Key::C,
        VirtualKeyCode::D => Key::D,
        VirtualKeyCode::E => Key::E,
        VirtualKeyCode::F => Key::F,
        VirtualKeyCode::G => Key::G,
        VirtualKeyCode::H => Key::H,
        VirtualKeyCode::I => Key::I,
        VirtualKeyCode::J => Key::J,
        VirtualKeyCode::K => Key::K,
        VirtualKeyCode::L => Key::L,
        VirtualKeyCode::M => Key::M,
        VirtualKeyCode::N => Key::N,
        VirtualKeyCode::O => Key::O,
        VirtualKeyCode::P => Key::P,
        VirtualKeyCode::Q => Key::Q,
        VirtualKeyCode::R => Key::R,
        VirtualKeyCode::S => Key::S,
        VirtualKeyCode::T => Key::T,
        VirtualKeyCode::U => Key::U,
        VirtualKeyCode::V => Key::V,
        VirtualKeyCode::W => Key::W,
        VirtualKeyCode::X => Key::X,
        VirtualKeyCode::Y => Key::Y,
        VirtualKeyCode::Z => Key::Z,
        VirtualKeyCode::Escape => Key::Escape,
        VirtualKeyCode::F1 => Key::F1,
        VirtualKeyCode::F2 => Key::F2,
        VirtualKeyCode::F3 => Key::F3,
        VirtualKeyCode::F4 => Key::F4,
        VirtualKeyCode::F5 => Key::F5,
        VirtualKeyCode::F6 => Key::F6,
        VirtualKeyCode::F7 => Key::F7,
        VirtualKeyCode::F8 => Key::F8,
        VirtualKeyCode::F9 => Key::F9,
        VirtualKeyCode::F10 => Key::F10,
        VirtualKeyCode::F11 => Key::F11,
        VirtualKeyCode::F12 => Key::F12,
        VirtualKeyCode::F13 => Key::F13,
        VirtualKeyCode::F14 => Key::F14,
        VirtualKeyCode::F15 => Key::F15,
        VirtualKeyCode::F16 => Key::F16,
        VirtualKeyCode::F17 => Key::F17,
        VirtualKeyCode::F18 => Key::F18,
        VirtualKeyCode::F19 => Key::F19,
        VirtualKeyCode::F20 => Key::F20,
        VirtualKeyCode::F21 => Key::F21,
        VirtualKeyCode::F22 => Key::F22,
        VirtualKeyCode::F23 => Key::F23,
        VirtualKeyCode::F24 => Key::F24,
        VirtualKeyCode::Snapshot => Key::Print,
        VirtualKeyCode::Scroll => Key::Scroll,
        VirtualKeyCode::Pause => Key::Pause,
        VirtualKeyCode::Insert => Key::Insert,
        VirtualKeyCode::Home => Key::Home,
        VirtualKeyCode::Delete => Key::Delete,
        VirtualKeyCode::End => Key::End,
        VirtualKeyCode::PageDown => Key::PageDown,
        VirtualKeyCode::PageUp => Key::PageUp,
        VirtualKeyCode::Left => Key::Left,
        VirtualKeyCode::Up => Key::Up,
        VirtualKeyCode::Right => Key::Right,
        VirtualKeyCode::Down => Key::Down,
        VirtualKeyCode::Back => Key::Back,
        VirtualKeyCode::Return => Key::Return,
        VirtualKeyCode::Space => Key::Space,
        VirtualKeyCode::Compose => Key::Compose,
        VirtualKeyCode::Caret => Key::Caret,
        VirtualKeyCode::Numlock => Key::Numlock,
        VirtualKeyCode::Numpad0 => Key::Numpad0,
        VirtualKeyCode::Numpad1 => Key::Numpad1,
        VirtualKeyCode::Numpad2 => Key::Numpad2,
        VirtualKeyCode::Numpad3 => Key::Numpad3,
        VirtualKeyCode::Numpad4 => Key::Numpad4,
        VirtualKeyCode::Numpad5 => Key::Numpad5,
        VirtualKeyCode::Numpad6 => Key::Numpad6,
        VirtualKeyCode::Numpad7 => Key::Numpad7,
        VirtualKeyCode::Numpad8 => Key::Numpad8,
        VirtualKeyCode::Numpad9 => Key::Numpad9,
        VirtualKeyCode::NumpadAdd => Key::NumpadDivide,
        VirtualKeyCode::NumpadDivide => Key::NumpadDivide,
        VirtualKeyCode::NumpadDecimal => Key::NumpadDecimal,
        VirtualKeyCode::NumpadComma => Key::NumpadComma,
        VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
        VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
        VirtualKeyCode::NumpadMultiply => Key::NumpadMultiply,
        VirtualKeyCode::NumpadSubtract => Key::NumpadSubtract,
        VirtualKeyCode::AbntC1 => Key::AbntC1,
        VirtualKeyCode::AbntC2 => Key::AbntC2,
        VirtualKeyCode::Apostrophe => Key::Apostrophe,
        VirtualKeyCode::Apps => Key::Apps,
        VirtualKeyCode::Asterisk => Key::Asterisk,
        VirtualKeyCode::At => Key::At,
        VirtualKeyCode::Ax => Key::Ax,
        VirtualKeyCode::Backslash => Key::Backslash,
        VirtualKeyCode::Calculator => Key::Calculator,
        VirtualKeyCode::Capital => Key::Capital,
        VirtualKeyCode::Colon => Key::Colon,
        VirtualKeyCode::Comma => Key::Comma,
        VirtualKeyCode::Convert => Key::Convert,
        VirtualKeyCode::Equals => Key::Equals,
        VirtualKeyCode::Grave => Key::Grave,
        VirtualKeyCode::Kana => Key::Kana,
        VirtualKeyCode::Kanji => Key::Kanji,
        VirtualKeyCode::LAlt => Key::LAlt,
        VirtualKeyCode::LBracket => Key::LBracket,
        VirtualKeyCode::LControl => Key::LControl,
        VirtualKeyCode::LShift => Key::LShift,
        VirtualKeyCode::LWin => Key::LWin,
        VirtualKeyCode::Mail => Key::Mail,
        VirtualKeyCode::MediaSelect => Key::MediaSelect,
        VirtualKeyCode::MediaStop => Key::MediaStop,
        VirtualKeyCode::Minus => Key::Minus,
        VirtualKeyCode::Mute => Key::Mute,
        VirtualKeyCode::MyComputer => Key::MyComputer,
        VirtualKeyCode::NavigateForward => Key::NavigateForward,
        VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
        VirtualKeyCode::NextTrack => Key::NextTrack,
        VirtualKeyCode::NoConvert => Key::NoConvert,
        VirtualKeyCode::OEM102 => Key::OEM102,
        VirtualKeyCode::Period => Key::Period,
        VirtualKeyCode::PlayPause => Key::PlayPause,
        VirtualKeyCode::Plus => Key::Plus,
        VirtualKeyCode::Power => Key::Power,
        VirtualKeyCode::PrevTrack => Key::PrevTrack,
        VirtualKeyCode::RAlt => Key::RAlt,
        VirtualKeyCode::RBracket => Key::RBracket,
        VirtualKeyCode::RControl => Key::RControl,
        VirtualKeyCode::RShift => Key::RShift,
        VirtualKeyCode::RWin => Key::RWin,
        VirtualKeyCode::Semicolon => Key::Semicolon,
        VirtualKeyCode::Slash => Key::Slash,
        VirtualKeyCode::Sleep => Key::Sleep,
        VirtualKeyCode::Stop => Key::Stop,
        VirtualKeyCode::Sysrq => Key::Sysrq,
        VirtualKeyCode::Tab => Key::Tab,
        VirtualKeyCode::Underline => Key::Underline,
        VirtualKeyCode::Unlabeled => Key::Unlabeled,
        VirtualKeyCode::VolumeDown => Key::VolumeDown,
        VirtualKeyCode::VolumeUp => Key::VolumeUp,
        VirtualKeyCode::Wake => Key::Wake,
        VirtualKeyCode::WebBack => Key::WebBack,
        VirtualKeyCode::WebFavorites => Key::WebFavorites,
        VirtualKeyCode::WebForward => Key::WebForward,
        VirtualKeyCode::WebHome => Key::WebHome,
        VirtualKeyCode::WebRefresh => Key::WebRefresh,
        VirtualKeyCode::WebSearch => Key::WebSearch,
        VirtualKeyCode::WebStop => Key::WebStop,
        VirtualKeyCode::Yen => Key::Yen,
        VirtualKeyCode::Copy => Key::Copy,
        VirtualKeyCode::Paste => Key::Paste,
        VirtualKeyCode::Cut => Key::Cut,
    }
}

pub fn convert_mouse(button: WinitMouseButton) -> MouseButton {
    match button {
        WinitMouseButton::Left => MouseButton::Left,
        WinitMouseButton::Right => MouseButton::Right,
        WinitMouseButton::Middle => MouseButton::Middle,
        WinitMouseButton::Other(other) => MouseButton::Other(other),
    }
}

pub fn convert_state(state: ElementState) -> State {
    match state {
        ElementState::Pressed => State::Pressed,
        ElementState::Released => State::Released,
    }
}
