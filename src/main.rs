#![windows_subsystem = "windows"]

#[cfg(target_os = "windows")]
mod app {
    use std::ptr::null_mut;

    use winapi::um::winuser::{IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_YESNO, MessageBoxW};

    const TEXT: [u16; 4] = [1053, 1077, 1090, 0];  // "Нет\0"

    pub fn main() {
        unsafe {
            loop {
                let result = MessageBoxW(
                    null_mut(),
                    TEXT.as_ptr(),
                    TEXT.as_ptr(),
                    MB_ICONINFORMATION | MB_YESNO,
                );

                if result == IDYES {
                    MessageBoxW(null_mut(), TEXT.as_ptr(), TEXT.as_ptr(), MB_ICONERROR | MB_OK);
                } else {
                    break
                }
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod app {
    pub fn main() {
        println!("Only windows supported");
    }
}

fn main() {
    app::main();
}
