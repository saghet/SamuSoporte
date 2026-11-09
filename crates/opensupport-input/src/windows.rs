use crate::error::InputError;
use opensupport_core::protocol::{InputEvent, MouseButton, SpecialKeyCombo};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_SCANCODE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_VIRTUALDESK, MOUSEEVENTF_WHEEL, MOUSEINPUT,
    VIRTUAL_KEY, VK_CONTROL, VK_DELETE, VK_LWIN, VK_MENU, VK_TAB,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};

pub struct WindowsInputInjector;

impl WindowsInputInjector {
    pub fn new() -> Self {
        Self
    }

    pub fn inject(&self, event: &InputEvent) -> Result<(), InputError> {
        match event {
            InputEvent::MouseMove { x, y } => self.inject_mouse_move(*x, *y),
            InputEvent::MouseDown { button } => self.inject_mouse_button(*button, true),
            InputEvent::MouseUp { button } => self.inject_mouse_button(*button, false),
            InputEvent::MouseWheel { delta_x: _, delta_y } => self.inject_mouse_wheel(*delta_y),
            InputEvent::KeyDown { key_code, scan_code } => self.inject_key(*key_code, *scan_code, true),
            InputEvent::KeyUp { key_code, scan_code } => self.inject_key(*key_code, *scan_code, false),
            InputEvent::SpecialCombo(combo) => self.inject_special_combo(*combo),
        }
    }

    fn inject_mouse_move(&self, x: i32, y: i32) -> Result<(), InputError> {
        unsafe {
            let v_x = GetSystemMetrics(SM_XVIRTUALSCREEN);
            let v_y = GetSystemMetrics(SM_YVIRTUALSCREEN);
            let v_w = GetSystemMetrics(SM_CXVIRTUALSCREEN);
            let v_h = GetSystemMetrics(SM_CYVIRTUALSCREEN);

            if v_w <= 0 || v_h <= 0 {
                return Err(InputError::WindowsApi("Virtual screen metrics invalid".into()));
            }

            // Normalización a coordenadas absolutas de Windows (0 a 65535)
            let norm_x = ((x - v_x) * 65535 / v_w) as i32;
            let norm_y = ((y - v_y) * 65535 / v_h) as i32;

            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: norm_x,
                        dy: norm_y,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK | MOUSEEVENTF_MOVE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let sent = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if sent == 0 {
                let err = windows::core::Error::from_win32();
                if err.code() == windows::core::HRESULT(0x80070005u32 as i32) {
                    return Err(InputError::PermissionDenied(
                        "Acceso denegado por UIPI / UI Automation. Se requiere ejecución elevada o servicio SYSTEM para inyectar entrada en este escritorio.".into()
                    ));
                }
                return Err(InputError::WindowsApi(format!("SendInput mouse move failed: {:?}", err)));
            }
            Ok(())
        }
    }

    fn inject_mouse_button(&self, button: MouseButton, is_down: bool) -> Result<(), InputError> {
        let flag = match (button, is_down) {
            (MouseButton::Left, true) => MOUSEEVENTF_LEFTDOWN,
            (MouseButton::Left, false) => MOUSEEVENTF_LEFTUP,
            (MouseButton::Right, true) => MOUSEEVENTF_RIGHTDOWN,
            (MouseButton::Right, false) => MOUSEEVENTF_RIGHTUP,
            (MouseButton::Middle, true) => MOUSEEVENTF_MIDDLEDOWN,
            (MouseButton::Middle, false) => MOUSEEVENTF_MIDDLEUP,
            _ => return Ok(()),
        };

        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: flag,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let sent = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if sent == 0 {
                return Err(InputError::WindowsApi("SendInput mouse click failed".into()));
            }
            Ok(())
        }
    }

    fn inject_mouse_wheel(&self, delta_y: i32) -> Result<(), InputError> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: delta_y as u32,
                        dwFlags: MOUSEEVENTF_WHEEL,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let sent = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if sent == 0 {
                return Err(InputError::WindowsApi("SendInput mouse wheel failed".into()));
            }
            Ok(())
        }
    }

    fn inject_key(&self, key_code: u32, scan_code: u32, is_down: bool) -> Result<(), InputError> {
        unsafe {
            let mut flags = if is_down { 0 } else { KEYEVENTF_KEYUP.0 };
            let (vk, scan) = if scan_code > 0 {
                flags |= KEYEVENTF_SCANCODE.0;
                (0, scan_code as u16)
            } else {
                (key_code as u16, 0)
            };

            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: scan,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(flags),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let sent = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if sent == 0 {
                return Err(InputError::WindowsApi("SendInput keyboard failed".into()));
            }
            Ok(())
        }
    }

    fn inject_special_combo(&self, combo: SpecialKeyCombo) -> Result<(), InputError> {
        match combo {
            SpecialKeyCombo::AltTab => {
                self.inject_key(VK_MENU.0 as u32, 0, true)?;
                self.inject_key(VK_TAB.0 as u32, 0, true)?;
                self.inject_key(VK_TAB.0 as u32, 0, false)?;
                self.inject_key(VK_MENU.0 as u32, 0, false)?;
            }
            SpecialKeyCombo::WinL => {
                self.inject_key(VK_LWIN.0 as u32, 0, true)?;
                self.inject_key('L' as u32, 0, true)?;
                self.inject_key('L' as u32, 0, false)?;
                self.inject_key(VK_LWIN.0 as u32, 0, false)?;
            }
            SpecialKeyCombo::CtrlAltDel => {
                // Nota de seguridad y arquitectura:
                // En Windows, Ctrl+Alt+Del (SAS - Secure Attention Sequence) sólo puede ser simulado
                // por un servicio ejecutándose como LocalSystem llamando a SendSAS().
                // Este método simula la combinación para contextos estándar y se enruta al servicio.
                self.inject_key(VK_CONTROL.0 as u32, 0, true)?;
                self.inject_key(VK_MENU.0 as u32, 0, true)?;
                self.inject_key(VK_DELETE.0 as u32, 0, true)?;
                self.inject_key(VK_DELETE.0 as u32, 0, false)?;
                self.inject_key(VK_MENU.0 as u32, 0, false)?;
                self.inject_key(VK_CONTROL.0 as u32, 0, false)?;
            }
        }
        Ok(())
    }
}

impl Default for WindowsInputInjector {
    fn default() -> Self {
        Self::new()
    }
}
