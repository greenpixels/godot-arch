pub fn enable_ansi_support() {
    #[cfg(windows)]
    {
        use std::ptr::null_mut;
        use winapi::um::consoleapi::GetConsoleMode;
        use winapi::um::consoleapi::SetConsoleMode;
        use winapi::um::processenv::GetStdHandle;
        use winapi::um::winbase::STD_OUTPUT_HANDLE;
        use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle != null_mut() {
                let mut mode = 0;
                if GetConsoleMode(handle, &mut mode) != 0 {
                    SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
                }
            }
        }
    }
}
