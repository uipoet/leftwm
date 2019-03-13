use super::Window;
use super::WindowHandle;
use super::WindowType;

#[derive(Debug, Clone)]
pub struct WindowChange {
    pub handle: WindowHandle,
    pub transient: Option<Option<WindowHandle>>,
    pub name: Option<Option<String>>,
    pub type_: Option<WindowType>,
    pub floating_loc: Option<(i32, i32)>,
    pub floating_size: Option<(i32, i32)>,
}

impl WindowChange {
    pub fn new(h: WindowHandle) -> WindowChange {
        WindowChange {
            handle: h,
            transient: None,
            name: None,
            type_: None,
            floating_loc: None,
            floating_size: None,
        }
    }

    pub fn update(&self, window: &mut Window) {
        if let Some(trans) = &self.transient {
            window.transient = trans.clone();
        }
        if let Some(name) = &self.name {
            window.name = name.clone();
        }
        if let Some(floating_loc) = self.floating_loc {
            window.floating_loc = Some(floating_loc);
        }
        if let Some(floating_size) = self.floating_size {
            window.floating_size = Some(floating_size);
        }
        if let Some(type_) = &self.type_ {
            window.type_ = type_.clone();
            window.margin = 0;
            window.border = 0;
        }
    }
}
