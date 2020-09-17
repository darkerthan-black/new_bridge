#![allow(non_snake_case)]



extern crate dlopen;
extern crate wio;


pub mod kiwoom_ctrl;
pub mod ocidl;
mod event_handle;

// #[macro_use]
// extern crate guid;

#[macro_use]
extern crate dlopen_derive;


#[macro_use]
extern crate lazy_static;


use crate::kiwoom_ctrl::{ Kiwoom, SHARED_KIWOOM};

use dlopen::wrapper::{Container, WrapperApi};
use dlopen::raw::Library;
use dlopen::symbor::Library as LibSym;
use winapi::shared::ntdef::{HRESULT, NULL};
use winapi::shared::wtypesbase::LPCOLESTR;
use winapi::shared::windef::{HWND, HBRUSH, HMENU__};
use winapi::um::objidlbase::IStream;
use winapi::um::unknwnbase::IUnknown;
use winapi::shared::wtypes::BSTR;
use winapi::_core::iter::once;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{HMODULE, DWORD, HINSTANCE, UINT, WPARAM, LPARAM, LRESULT};
use std::io::Error;
use winapi::_core::mem::{zeroed, size_of, MaybeUninit};
use winapi::_core::ptr::null_mut;
use winapi::um::commctrl::{INITCOMMONCONTROLSEX, ICC_WIN95_CLASSES, InitCommonControlsEx};
use winapi::um::objbase::CoInitialize;
use winapi::um::winuser::{WNDCLASSEXW, CS_DBLCLKS, LoadIconW, IDI_APPLICATION, LoadCursorW, IDC_ARROW, COLOR_BTNSHADOW, RegisterClassExW, CreateWindowExW, WS_OVERLAPPEDWINDOW, HWND_DESKTOP, ShowWindow, SW_SHOWNORMAL, UpdateWindow, MSG, GetMessageW, TranslateMessage, DispatchMessageW, CREATESTRUCTW, SetWindowLongPtrW, GWLP_USERDATA, DestroyWindow, PostQuitMessage, DefWindowProcW, WM_CREATE};
use winapi::_core::ffi::c_void;
use winapi::_core::mem;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use winapi::shared::guiddef::REFIID;
use winapi::_core::borrow::BorrowMut;
// use guid::GUID;

//
// #[macro_use]
// extern crate guid;
// use guid::GUID;
// //
// const MY_GUID: GUID = guid!{"3a00a55e-f50d-11ea-adc1-0242ac120002"};//3DED3F5C_22B7_443A_9561_7B54B917ACA4
// const MY_GUID: GUID = guid!{"3DED3F5C-22B7-443A-9561-7B54B917ACA4"};



#[derive(WrapperApi)]
struct AtlApi {
    // example_rust_fun: fn(arg: i32) -> u32,
    // example_c_fun: unsafe extern "C" fn(),
    // example_reference: &'a mut i32,
    AtlAxWinInit: unsafe extern "stdcall" fn()->HRESULT,
    AtlAxCreateControlLic: unsafe extern "stdcall" fn(lpszName: LPCOLESTR, hWnd:HWND, pStream:*mut IStream, ppUnkContainer:*mut *mut IUnknown, bstrLic:BSTR )->HRESULT,
    AtlAxCreateControl:unsafe extern "stdcall" fn(lpszName: LPCOLESTR, hWnd: HWND, pStream:*mut IStream, ppUnkContainer: *mut *mut IUnknown )->HRESULT,
    AtlAxCreateControlEx:unsafe extern "stdcall" fn(lpszName: LPCOLESTR, hWnd: HWND, pStream:*mut IStream, ppUnkContainer: *mut *mut IUnknown,
                         ppControl: *mut *mut IUnknown, iidSink: REFIID, pSink: *mut IUnknown   )->HRESULT,
    AtlAxGetControl:unsafe extern "stdcall" fn(h: HWND, pp: *mut *mut IUnknown)->HRESULT,
}





pub fn rust_to_win_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}


pub struct Window {
    handle: HWND,
}

struct UserData {
    in_window: isize,
}


pub fn create_window(hinstance: HMODULE, name: &str, title: &str) -> Result<Window, Error> {
    let name = crate::rust_to_win_str(name);
    let title = crate::rust_to_win_str(title);

    //heap에 생성한다.
    let in_window = Box::new(UserData {
        in_window: 0,

    });




    let mut handle = null_mut();


    unsafe {



        let icc = INITCOMMONCONTROLSEX {
            dwSize: mem::size_of::<INITCOMMONCONTROLSEX> as DWORD,
            dwICC: ICC_WIN95_CLASSES,
        };
        InitCommonControlsEx(&icc);


        let h_result = CoInitialize(null_mut());

        let wnd_class =
            WNDCLASSEXW {
                cbSize:size_of::<WNDCLASSEXW>() as u32,
                style: CS_DBLCLKS,
                lpfnWndProc: Some(kh_wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: LoadIconW(NULL as HINSTANCE, IDI_APPLICATION),
                hCursor: LoadCursorW(NULL as HINSTANCE , IDC_ARROW),
                hbrBackground: COLOR_BTNSHADOW as HBRUSH,
                lpszMenuName: null_mut(),
                lpszClassName: name.as_ptr(),
                hIconSm: LoadIconW(NULL as HINSTANCE, IDI_APPLICATION),
            };

        RegisterClassExW(&wnd_class);


        handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW ,
            100,
            15,
            1200,
            830,
            HWND_DESKTOP,
            NULL as *mut HMENU__,
            hinstance,
            // NULL, // later
            Box::into_raw(in_window) as *mut UserData as *mut c_void,
        );

        ShowWindow(handle, SW_SHOWNORMAL);
        UpdateWindow(handle);


    };

    if handle.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(Window { handle })
    }

}

pub fn handle_message(window: &mut Window)-> bool {
    unsafe {
        let message = MaybeUninit::<MSG>::uninit();
        if GetMessageW(message.as_ptr() as *mut MSG, window.handle, 0, 0) >0  {
            // if IsDialogMessageW(window.handle, message.as_ptr() as *mut MSG ) == minwindef::FALSE {
                TranslateMessage(message.as_ptr() as *const MSG);
                DispatchMessageW(message.as_ptr() as *const MSG);
            // }

            true
        } else {
            false
        }
    }
}

unsafe extern "system" fn kh_wnd_proc( hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM ) -> LRESULT {


    // println!("프로시져 {}", msg);
    match msg {
        WM_CREATE => {
            let bw = (*(lParam as *mut CREATESTRUCTW)).lpCreateParams as *mut UserData;//lparam 으로 전달된 포인터를 윈도우의 유저데이터로 저장.
            SetWindowLongPtrW(hWnd, GWLP_USERDATA, bw as i32);
            // create_button(wstr("button1"), BtnId::Btn1, 50, 100, 100, 25, hWnd);
            // create_button(wstr("button2"), BtnId::Btn2, 250, 100, 100, 25, hWnd);
            0
        }
        WM_CLOSE => {DestroyWindow(hWnd); 0 }
        WM_DESTROY => {PostQuitMessage(0); 0}
        _ => DefWindowProcW(hWnd, msg, wParam, lParam),
    }



}

pub fn show_window() {

    // let hinstance: HMODULE = unsafe {  GetModuleHandleW(null_mut()) };

    // let mut window = create_window(hinstance, "Form1", "Form1").unwrap();

    // let kiwoom = unsafe{ Kiwoom::new(window.handle).unwrap() };
    //


    let mut window:Window = create_window(unsafe { GetModuleHandleW(null_mut()) }, "Form1", "Form1").unwrap();
    // let kiwoom :Kiwoom = unsafe{  };

    unsafe {

        SHARED_KIWOOM = Some(Kiwoom::new(window.handle).unwrap());

        SHARED_KIWOOM.as_ref().unwrap().comm_connect();
//
// let name =         SHARED_KIWOOM.as_ref().unwrap().GetLoginInfo(rust_to_win_str("USER_NAME").as_mut_ptr().borrow_mut());
//     println!("{}", wio::bstr::BStr::from_raw(name).to_string_lossy());
    }

    // unsafe { SHARED_KIWOOM.as_ref().unwrap(). }

    // unsafe { SHARED_KIWOOM.as_ref().unwrap().GetLoginInfo("ACCOUNT"); }

    // kiwoom.comm_connect();

    // KIWOOM.lock().unwrap().comm_connect();




    // println!( " {:?}", MY_GUID);


    // println!("{:#x}, {:#x}. {:#x}, {:#x?}",973120862, 62733,  4586,  [173, 193, 2, 66, 172, 18, 0, 2] );
    // 0x120B729F,0x65BF,0x11D0,0x9D,0xDC,0x00,0xA0,0xC9,0x03,0x48,0x92


    // println!("{:#x}, {:#x}. {:#x}, {:#x?}",1038958428, 8887,17466, [149, 97, 123, 84, 185, 23, 172, 164]  );
    // 0x3ded3f5c, 0x22b7. 0x443a,0x95,0x61, 0x7b, 0x54, 0xb9, 0x17, 0xac, 0xa4,


    // let c_hwnd = vec![h_container];


    while crate::handle_message(&mut window) {}

}




