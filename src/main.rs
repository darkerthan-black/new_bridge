#![windows_subsystem = "windows"]
#![allow(unused_assignments)]
//#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

//
// mod lib;

extern crate winapi;

// mod kiwoom_ctrl;
use new_bridge::kiwoom_ctrl::*;



use dlopen::wrapper::{Container, WrapperApi};
use dlopen::raw::Library;
use dlopen::symbor::Library as LibSym;

use winapi::_core::iter::once;
use winapi::um::winuser::{LPCREATESTRUCTW, DefWindowProcW, RegisterClassW, CreateWindowExW, GetMessageW, MSG, WNDCLASSEXW, CS_DBLCLKS, LoadIconW, IDI_APPLICATION, LoadCursorW, IDC_ARROW, COLOR_BTNSHADOW, RegisterClassExW, ShowWindow, SW_SHOWNORMAL, TranslateMessage, DispatchMessageW, WS_CHILD, CreateDialogParamW, MAKEINTRESOURCEW, UpdateWindow, WM_INITDIALOG};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::{HWND, HBRUSH, HMENU__, HMENU};
use winapi::shared::ntdef::{NULL,LONG};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HMODULE, HINSTANCE};
use winapi::shared::minwindef::*;
use winapi::um::winuser::{CS_OWNDC,CS_HREDRAW, CS_VREDRAW,WS_OVERLAPPEDWINDOW, WS_VISIBLE, HWND_DESKTOP };
use winapi::um::unknwnbase::*;
use winapi::shared::wtypesbase::*;
use winapi::um::objidlbase::IStream;
use winapi::shared::wtypes::BSTR;
use winapi::um::objbase::{COINIT_APARTMENTTHREADED, CoInitialize};
use winapi::um::winuser::*;
use winapi::shared::guiddef::*;
use winapi::RIDL;
use winapi::um::oaidl::*;
use winapi::shared::ntdef::*;
use winapi::um::combaseapi::*;
use winapi::DEFINE_GUID;
use winapi::um::oleauto::*;
use winapi::um::oleauto::DISPID;
use winapi::shared::wtypes::*;
// use winapi::um::commctrl::*;
use winapi::um::commctrl::{INITCOMMONCONTROLSEX, ICC_WIN95_CLASSES, InitCommonControlsEx} ;




use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::io::Error;
use std::mem::{MaybeUninit, zeroed};


use winapi::_core::mem::size_of;
// use com::sys::{HRESULT, CoInitializeEx, CoUninitialize};
use winapi::shared::winerror::{SUCCEEDED, FAILED};
use winapi::um::errhandlingapi::GetLastError;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::*;
use winapi::_core::mem;
use winapi::shared::minwindef;
use new_bridge::Window;


const IDI_APPICON:u16         =            101;
const IDR_MAINMENU:u16         =           102;
const IDR_ACCELERATOR:u16 =          103;

const ID_FILE_EXIT:u16 =         40001;
const ID_HELP_ABOUT:u16 =        40002;

// const IDC_STATIC :i16         =         -1;


const  ID_CONTAINER : usize = 1250;
// const IDD_ABOUTDIALOG :u16  = 104;
const IDD_KHOPENAPITEST_DLG:u16  =         1000;
// const IDC_KHOPENAPICTRL1:u16     =         1011;


fn main() {


    new_bridge::show_window();

}
