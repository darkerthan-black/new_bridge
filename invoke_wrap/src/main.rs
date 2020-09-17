use invoke_wrap::*;
use winapi::um::oaidl::{VARIANT, *};
use winapi::um::oleauto::*;
// use winapi::_core::ptr::null_mut;


// invoke_wrap!(CommConnect, p_inter, 0x1, DISPATCH_METHOD, void,0);
// invoke_wrap!(CommConnect, p_inter,0x1, DISPATCH_METHOD, (), 0 );
// invoke_wrap!(GetLoginInfo,p_inter,0x4, DISPATCH_METHOD,BSTR, 1, (BSTR, ACCOUNT_CNT) );
// invoke_wrap!();
// invoke_wrap!(foo, u32, f64,);
// invoke_wrap!(bar, String, u32, f64);
// make_function!(error, String, u32,, f64);  // 컴파일 에러
//make_function!(todo, &str); // 지원 안됨.
use wio::com::ComPtr;
use wio::handle::Handle;
use winapi::um::oaidl::{IDispatch,* };
use winapi::um::oleauto::*;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::LONG;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{HMODULE, UINT, WPARAM, LPARAM, LRESULT, LOWORD};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winuser::{CreateDialogParamW, MAKEINTRESOURCEW, DefWindowProcW, WM_INITDIALOG, DefDlgProcW, DestroyWindow, PostQuitMessage, IDCANCEL, EndDialog};
use wio::bstr::BStr;
use winapi::_core::mem::zeroed;
use winapi::shared::winerror::{SUCCEEDED, FAILED};
use winapi::shared::guiddef::{LPIID, GUID, IID_NULL};
use winapi::um::combaseapi::IIDFromString;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::*;
use winapi::um::errhandlingapi::GetLastError;
use std::sync::Mutex;
use winapi::shared::wtypes::*;

fn main() {

    let arg_atr = "ACCOUNT_CNT";
    // invoke_wrap!( self.comp_kiwoom, 0x1, DISPATCH_METHOD, VT_I4, 0 );
    invoke_wrap!(self.comp_kiwoom,0x4, DISPATCH_METHOD,VT_BSTR, 1,  sTag, VT_BSTR   );
    // invoke_wrap!(self.comp_kiwoom, 0x3, DISPATCH_METHOD, VT_I4, 4,
    //                 sRQName, VT_BSTR ,
    //                sTrCode, VT_BSTR ,
    //                nPrevNext, VT_BSTR ,
    //                sScreenNo,  VT_BSTR  )
    // generated_function();
    // foo(1, 1.23);
    // bar(String::from("test"), 1, 1.23);
}

// invoke!(p_inter, disp_id, DISPATCH_METHOD, var_num, v(type, val), ...  )
//
// let mut dp = DISPPARAMS { rgvarg: null_mut(), rgdispidNamedArgs: null_mut(), cArgs: 0, cNamedArgs: 0 };
// let p_dp: *mut DISPPARAMS = &mut dp;
// let mut var: VARIANT = VARIANT::default();
// let p_var: *mut VARIANT = &mut var;
// VariantInit(p_var);
//
// let mut retval: u32 = 0;
// let p_val: *mut u32 = &mut retval;
//
//
//
// (**p_kh_interface).Invoke(
// // <*const p_kh_interface>::as_ref().Invoke(
// 0x1,
// &IID_NULL,
// LOCALE_USER_DEFAULT,
// DISPATCH_METHOD,
// p_dp,
// p_var,
// null_mut(),
// p_val
// );