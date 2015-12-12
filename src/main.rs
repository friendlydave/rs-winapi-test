extern crate user32;
extern crate winapi;
extern crate wio;
extern crate kernel32;

use user32::*;
use kernel32::*;
use winapi::*;
use wio::wide::ToWide;

use std::mem::zeroed;
use std::ptr;

fn str_to_wchar(s:&str) -> *const u16 {
    s.to_wide_null().as_ptr()
}

fn main() {
    unsafe {
        main2()
    }
}

// This was translated from some C++ code on MSDN.
// TODO: link to the original code on MSDN
unsafe fn main2() {
    println!("App Start");

    let CLASS_NAME = str_to_wchar("Sample Window Class");
    let window_text = str_to_wchar("Learn to Program Windows");

    let hInstance = GetModuleHandleW(0 as *const u16);

    let mut wc = zeroed::<WNDCLASSW>();
    wc = WNDCLASSW {
        lpfnWndProc: Some(WindowProc),
        hInstance: hInstance,
        lpszClassName: CLASS_NAME,
        hCursor: LoadCursorW(0 as HINSTANCE, IDC_ARROW),
        .. wc
    };

    RegisterClassW(&wc);

    let hwnd: HWND = CreateWindowExW(
        0,
        CLASS_NAME,
        //"a".to_wide_null().as_ptr(),
        ptr::null_mut(),
        WS_OVERLAPPEDWINDOW,

        // Size and position
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,

        ptr::null_mut(),
        ptr::null_mut(),
        hInstance,
        //ptr::null_mut(),
        ptr::null_mut()
    );
    println!("hwnd: {:?}",hwnd);
    if hwnd == 0 as HWND {
        let e = GetLastError();
        println!("err {}",e);
        return;
    }
    ShowWindow(hwnd, 5);

    let mut msg = zeroed::<MSG>();
    while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    println!("App End");
    return;
}

unsafe extern "system" fn WindowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_DESTROY => {
            println!("Destroying window");
            PostQuitMessage(0);
            return 0;
        }

        WM_PAINT => {
            let mut ps = zeroed::<PAINTSTRUCT>();
            let hdc = BeginPaint(hwnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW+1) as HBRUSH);
            EndPaint(hwnd, &ps);
            return 0;
        }

        _ => return DefWindowProcW(hwnd, uMsg, wParam, lParam)
    }
}
