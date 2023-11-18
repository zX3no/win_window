#![allow(non_snake_case)]
pub mod bitmap;
pub mod constants;
pub use bitmap::*;
pub use constants::*;

pub type HWND = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type WORD = u16;
pub type DWORD = u32;
pub type BOOL = i32;
pub type UINT = u32;
pub type LONG = i32;
pub type LPCSTR = *const i8;
pub type LPCWSTR = *const u16;

pub enum VOID {}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.left.abs() + self.right.abs()
    }
    pub fn height(&self) -> i32 {
        self.top.abs() + self.bottom.abs()
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct MSG {
    pub hwnd: isize,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: Point,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct WindowInfo {
    pub size: u32,
    pub window: Rect,
    pub client: Rect,
    pub style: u32,
    pub ex_style: u32,
    pub window_status: u32,
    pub window_borders_x: u32,
    pub window_borders_y: u32,
    pub window_type: u16,
    pub creator_version: u16,
}

#[link(name = "user32")]
extern "system" {
    pub fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;

    ///Return value
    ///
    ///Type: `HWND`
    ///
    ///If the function succeeds, the return value is a handle to the new window.
    ///
    ///If the function fails, the return value is `NULL`. To get extended error information, call GetLastError.
    ///
    ///This function typically fails for one of the following reasons:
    ///
    ///- an invalid parameter value
    ///- the system class was registered by a different module
    ///- The WH_CBT hook is installed and returns a failure code
    ///- if one of the controls in the dialog template is not registered, or its window window procedure fails WM_CREATE or WM_NCCREATE
    pub fn CreateWindowExA(
        dwexstyle: u32,
        lpclassname: *const u8,
        lpwindowname: *const u8,
        dwstyle: u32,
        x: i32,
        y: i32,
        nwidth: i32,
        nheight: i32,
        hwndparent: isize,
        hmenu: isize,
        hinstance: isize,
        lpparam: *const std::ffi::c_void,
    ) -> isize;

    pub fn AdjustWindowRectEx(lpRect: *mut Rect, dwStyle: u32, bMenu: i32, dwExStyle: u32) -> i32;

    pub fn DestroyWindow(hWnd: isize) -> i32;

    /// Indicates to the system that a thread has made a request to terminate (quit).
    /// It is typically used in response to a WM_DESTROY message.
    pub fn PostQuitMessage(nExitCode: i32);

    pub fn PeekMessageA(
        lpmsg: *mut MSG,
        hwnd: isize,
        wmsgfiltermin: u32,
        wmsgfiltermax: u32,
        wremovemsg: u32,
    ) -> i32;

    pub fn DefWindowProcA(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize;

    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;

    pub fn GetMessageA(
        lpMsg: *const MSG,
        hWnd: isize,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;

    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;

    pub fn GetLastError() -> u32;

    pub fn GetWindow(hwnd: isize, uCmd: u32) -> isize;

    pub fn GetForegroundWindow() -> isize;

    pub fn GetProcAddress(hModule: *mut VOID, lpProcName: *const i8) -> *mut VOID;

    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut VOID;

    pub fn GetWindowLongPtrA(hwnd: isize, nIndex: i32) -> isize;

    pub fn ValidateRect(hwnd: isize, lpRect: *const Rect) -> i32;

    pub fn GetWindowRect(hwnd: isize, lpRect: *mut Rect) -> i32;

    pub fn GetWindowInfo(hwnd: isize, pwi: *mut WindowInfo) -> i32;

    pub fn GetDC(hwnd: isize) -> *mut VOID;

    // pub fn wglGetProcAddress(lpszProc: *const i8) -> *mut VOID__;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WNDCLASSA {
    pub style: u32,
    pub wnd_proc: Option<
        unsafe extern "system" fn(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize,
    >,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: isize,
    pub icon: isize,
    pub cursor: isize,
    pub background: isize,
    pub menu_name: *const u8,
    pub class_name: *const u8,
}

impl Default for WNDCLASSA {
    fn default() -> Self {
        Self {
            style: Default::default(),
            wnd_proc: Default::default(),
            cls_extra: Default::default(),
            wnd_extra: Default::default(),
            instance: Default::default(),
            icon: Default::default(),
            cursor: Default::default(),
            background: Default::default(),
            menu_name: unsafe { std::mem::zeroed() },
            class_name: unsafe { std::mem::zeroed() },
        }
    }
}

pub struct Window {
    pub hwnd: isize,
    pub hinstance: isize,
}

//TODO: https://github.com/makepad/makepad/blob/master/libs/windows-core/src/hresult.rs#L29
impl Window {
    pub fn proc_addr(&self) -> isize {
        unsafe { GetWindowLongPtrA(self.hwnd, GWLP_WNDPROC) }
    }
    pub fn area(&self) -> Rect {
        let mut rect = Rect::default();
        //GetWindowRect is virtualized for DPI.
        unsafe { GetWindowRect(self.hwnd, &mut rect) };
        rect
    }
}

// pub unsafe fn wgl_proc(name: &str) -> *mut VOID__ {
//     let name = format!("{name}\0");
//     dbg!(&name);
//     let proc = wglGetProcAddress(name.as_ptr() as *const i8);
//     dbg!(GetLastError());
//     assert!(!proc.is_null());
//     proc
// }

//TODO: https://devblogs.microsoft.com/oldnewthing/20100412-00/?p=14353

/// Create a window.
/// ```rs
///    let window = create_window("window name", 1280, 960);
///
///    loop {
///        match event() {
///            Event::Quit => break,
///            _ => {}
///        }
///    }
/// ```
pub fn create_window(title: &str, width: i32, height: i32) -> Window {
    //Title must be null terminated.
    let title = std::ffi::CString::new(title).unwrap();
    let wnd_class = WNDCLASSA {
        // wnd_proc: Some(DefWindowProcA),
        wnd_proc: Some(test_proc),
        class_name: title.as_ptr() as *const u8,
        // style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        style: CS_HREDRAW | CS_VREDRAW,
        background: 0,
        ..Default::default()
    };

    let options = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

    let _result = unsafe { RegisterClassA(&wnd_class) };
    let hinstance = get_instance_handle();
    let rect = adjust_window_size(width, height, options);

    let hwnd = unsafe {
        CreateWindowExA(
            0,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            options,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            //TODO:
            //Note: Width and height include the border.
            rect.width(),
            rect.height(),
            0,
            0,
            hinstance,
            std::ptr::null(),
        )
    };

    assert_ne!(hwnd, 0);

    Window { hwnd, hinstance }
}

pub fn adjust_window_size(width: i32, height: i32, options: u32) -> Rect {
    let mut rect = Rect {
        left: 0,
        top: 0,
        right: width,
        bottom: height,
    };
    let result = unsafe { AdjustWindowRectEx(&mut rect as *mut Rect, options, 0, 0) };
    if result == 0 {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Error with `AdjustWindowRectEx`, error code: {}",
            last_error
        );
    }
    rect
}

pub enum Event {
    Quit,
    Resize(u32, u32),
}

static mut MSG: MSG = MSG {
    hwnd: 0,
    message: 0,
    w_param: 0,
    l_param: 0,
    time: 0,
    pt: Point { x: 0, y: 0 },
};

static mut QUIT: bool = false;

unsafe extern "system" fn test_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    match msg {
        WM_DESTROY | WM_CLOSE => {
            QUIT = true;
            // PostQuitMessage(0);
            return 0;
        }
        // WM_CLOSE => {
        // DestroyWindow(hwnd);
        // }
        WM_CREATE => {
            if !set_dark_mode(hwnd) {
                println!("Failed to set dark mode!");
            }
            return 0;
        }
        WM_ERASEBKGND => {
            return 1;
        }
        WM_PAINT => {
            ValidateRect(hwnd, std::ptr::null());
            return 0;
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

pub fn event() -> Option<Event> {
    unsafe {
        if QUIT {
            return Some(Event::Quit);
        }

        let result = PeekMessageA(&mut MSG, 0, 0, 0, PM_REMOVE);
        match result {
            0 => None,
            _ => match MSG.message {
                WM_SIZE => {
                    let width = (MSG.l_param as u32) & 0xffff;
                    let height = ((MSG.l_param as u32) >> 16) & 0xffff;
                    Some(Event::Resize(width, height))
                }
                _ => {
                    TranslateMessage(&mut MSG);
                    DispatchMessageA(&mut MSG);
                    None
                }
            },
        }
    }
}

pub fn event_blocking() -> Option<Event> {
    let message_result = unsafe { GetMessageA(&mut MSG, 0, 0, 0) };

    match message_result {
        -1 => {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        }
        0 => Some(Event::Quit),
        _ => {
            //Handle message here.
            unsafe {
                TranslateMessage(&mut MSG);
                DispatchMessageA(&mut MSG);
            }
            None
        }
    }
}

pub fn get_instance_handle() -> isize {
    // Gets the instance handle by taking the address of the
    // pseudo-variable created by the microsoft linker:
    // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483

    // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
    // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance
    #[repr(C, packed(2))]
    pub struct ImageDosHeader {
        pub e_magic: u16,
        pub e_cblp: u16,
        pub e_cp: u16,
        pub e_crlc: u16,
        pub e_cparhdr: u16,
        pub e_minalloc: u16,
        pub e_maxalloc: u16,
        pub e_ss: u16,
        pub e_sp: u16,
        pub e_csum: u16,
        pub e_ip: u16,
        pub e_cs: u16,
        pub e_lfarlc: u16,
        pub e_ovno: u16,
        pub e_res: [u16; 4],
        pub e_oemid: u16,
        pub e_oeminfo: u16,
        pub e_res2: [u16; 10],
        pub e_lfanew: i32,
    }

    extern "C" {
        static __ImageBase: ImageDosHeader;
    }

    unsafe { &__ImageBase as *const _ as _ }
}

//TODO: Find a way to toggle this at runtime?
///Only works on Windows 17763 and above.
pub unsafe fn set_dark_mode(hwnd: isize) -> bool {
    const WCA_USEDARKMODECOLORS: u32 = 26;

    #[repr(C)]
    struct WINDOWCOMPOSITIONATTRIBDATA {
        attrib: u32,
        data: *mut std::ffi::c_void,
        size: usize,
    }

    #[repr(C)]
    struct OSVERSIONINFOW {
        pub dw_osversion_info_size: u32,
        pub dw_major_version: u32,
        pub dw_minor_version: u32,
        pub dw_build_number: u32,
        pub dw_platform_id: u32,
        pub sz_csdversion: [u16; 128],
    }

    //Check if this version of windows supports `SetWindowCompositionAttribute`.
    let nt = LoadLibraryA("ntdll.dll\0".as_ptr() as *const i8);
    let func = GetProcAddress(nt, "RtlGetVersion\0".as_ptr() as *const i8);
    let get_version: fn(*mut OSVERSIONINFOW) -> i32 = std::mem::transmute(func);
    let mut v: OSVERSIONINFOW = OSVERSIONINFOW {
        dw_osversion_info_size: 0,
        dw_major_version: 0,
        dw_minor_version: 0,
        dw_build_number: 0,
        dw_platform_id: 0,
        sz_csdversion: [0; 128],
    };
    let status = get_version(&mut v);

    if v.dw_build_number < 17763 || status < 0 {
        return false;
    }

    let user32 = LoadLibraryA("user32.dll\0".as_ptr() as *const i8);
    let func = GetProcAddress(
        user32,
        "SetWindowCompositionAttribute\0".as_ptr() as *const i8,
    );
    let set_window: fn(isize, *mut WINDOWCOMPOSITIONATTRIBDATA) -> i32 = std::mem::transmute(func);
    let mut dark_mode: i32 = 1;
    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WCA_USEDARKMODECOLORS,
        data: &mut dark_mode as *mut _ as _,
        size: 4,
    };

    set_window(hwnd, &mut data) != 0
}
