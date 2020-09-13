
extern crate invoke_wrap;

use invoke_wrap::*;
use dlopen::wrapper::{Container};
// use dlopen::raw::Library;
use crate::{rust_to_win_str, create_window};
use crate::Window;
use crate::AtlApi;
use wio::com::ComPtr;
use wio::handle::Handle;
use winapi::um::oaidl::{IDispatch,* };
use winapi::um::oleauto::*;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::LONG;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{HMODULE, UINT, WPARAM, LPARAM, LRESULT, LOWORD, DWORD};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winuser::{CreateDialogParamW, MAKEINTRESOURCEW, DefWindowProcW, WM_INITDIALOG, DefDlgProcW, DestroyWindow, PostQuitMessage, IDCANCEL, EndDialog};
use wio::bstr::BStr;
use winapi::_core::mem::zeroed;
use winapi::shared::winerror::{SUCCEEDED, FAILED};
use winapi::shared::guiddef::{LPIID, GUID, IID_NULL, REFIID, IID};
use winapi::um::combaseapi::{IIDFromString, CLSCTX_SERVER, CoCreateInstance, CoInitializeEx};
use winapi::ctypes::c_void;
use winapi::shared::ntdef::*;
use winapi::um::errhandlingapi::GetLastError;
use std::sync::Mutex;
use winapi::shared::wtypes::*;
use winapi::RIDL;
use crate::ocidl::{IConnectionPoint, IConnectionPointContainer};//  AtlAxCreateControlEx  사용시 불필요

// use com::sys::CoCreateInstance;
use winapi::shared::wtypesbase::{CLSCTX_LOCAL_SERVER, CLSCTX_INPROC_SERVER};
use winapi::um::objbase::{CoInitialize, COINIT_APARTMENTTHREADED};
use crate::event_handle::EventHandle;
// use guid::GUID;



// use std::option::Option::None;

// const IID_ICONNECTIONPOINT: guid::GUID = guid! {"B196B286-BAB4-101A-B69C-00AA00341D07"};
// const IID_ICONNECTIONPOINT_CONTAINER: guid::GUID = guid! {"B196B284-BAB4-101A-B69C-00AA00341D07"};
// const IID_IENUMCONNECTIONS: guid::GUID = guid! {"B196B285-BAB4-101A-B69C-00AA00341D07"};

//pub static IID_IUNKNOWN: &'static str = "00000000-0000-0000-C000-000000000046";
// const IID_IUNKNOWN:GUID = GUID { Data1:0x00000000, Data2: 0x0000, Data3:0x0000, Data4:[0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]};


const IDD_KHOPENAPITEST_DLG:u16  =         1000;

pub static mut SHARED_KIWOOM:Option<Kiwoom> = None;

pub struct Kiwoom {
    comp_kiwoom:ComPtr<IDispatch>,
    com_control:ComPtr<IUnknown>,
    com_container:ComPtr<IUnknown>,
    com_dkh_ev:ComPtr<IUnknown>,
    hwnd: HWND,
    con:Container<AtlApi>,
    com_dw_cookie: *mut DWORD,
}

impl Kiwoom {
    //다이얼로그를 생성하고, 키움 컨트롤을 얹는다.
    pub unsafe fn new(hwnd: HWND) -> Option<Kiwoom> {
        let hins: HMODULE = GetModuleHandleW(null_mut());


        // let h_container = CreateDialogParamW(
        //     hins,
        //     MAKEINTRESOURCEW(IDD_KHOPENAPITEST_DLG),
        //     hwnd, Some(ch_wnd_proc),  0);


        let kw_proid = rust_to_win_str("KHOPENAPI.KHOpenAPICtrl.1");


        let con: Container<AtlApi> = Container::load("atl100_32.dll").expect("atl 로딩실패");


        let mut h_result = con.AtlAxWinInit();


        let siid: LPIID = &mut zeroed::<GUID>();

        let mut retsink = IIDFromString(
            // rust_to_win_str("{A1574A0D-6BFA-4BD7-9020-DED88711818D}").as_ptr(),//키움 클래스
            rust_to_win_str("{7335F12D-8973-4BD5-B7F0-12DF03D175B7}").as_ptr(),//키움 이벤트

            siid
        );


        let mut pp_unk_container: *mut IUnknown = zeroed();
        let mut pp_unk_control: *mut IUnknown = zeroed();
        let mut p_kh_interface: *mut IDispatch = zeroed();




        // let mut p_sk_control: *mut IUnknown = zeroed();
        // let mut p_dkh_event: * mut DKHEvent
        // CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED);

        // h_result = CoCreateInstance(siid as *const IID,
        //                             null_mut(), CLSCTX_INPROC_SERVER, &IID_IUNKNOWN,
        //                             p_sk_control as *mut *mut c_void);

        // if FAILED(h_result) { println!("싱크 인스턴스 실패 {} {}", GetLastError(), h_result); }



        //키움 이벤트를 처리할 com 객체 생성
        let mut event = EventHandle::new();

        let mut dw_cookie: DWORD = 0;
        let p_event: *mut EventHandle = Box::into_raw(event);// 이벤트와 싱크할 포인터


        h_result = con.AtlAxCreateControlEx(
            kw_proid.as_ptr(), hwnd, null_mut(),
            <*mut *mut IUnknown>::cast(&mut pp_unk_container),
            <*mut *mut IUnknown>::cast(&mut pp_unk_control),
            // &IID_NULL,
            siid,
            <*mut IUnknown>::cast(p_event as *mut IUnknown),
            // null_mut(),
        );

        //
        //  AtlAxCreateControlEx 로 아래의 두 함수가 하는 일을 한번에 처리 가능.
        //
        // h_result = con.AtlAxCreateControl(
        //     kw_proid.as_ptr(), hwnd, null_mut(),
        //     <*mut *mut IUnknown>::cast(&mut pp_unk_container) );


        // h_result = con.AtlAxGetControl(
        //     hwnd,
        //     <*mut *mut IUnknown>::cast(&mut pp_unk_control));


        let riid: LPIID = &mut zeroed::<GUID>();

        let mut retval = IIDFromString(
            rust_to_win_str("{CF20FBB6-EDD4-4BE5-A473-FEF91977DEB6}").as_ptr(),
            riid
        );//키움 디스인터페이스


        h_result = (*pp_unk_control).QueryInterface(
            riid, &mut p_kh_interface as *mut *mut IDispatch as *mut *mut c_void);


        //
        //   AtlAxCreateContorlEx 사용시 한번에 sink 클래스까지 등록하는 경우 아래 iConnectionPoint 관련 코드는 불필요하다.
        //


        // let mut p_kh_sink: *mut IConnectionPoint = zeroed();
        // let mut p_connect_contain: *mut IConnectionPointContainer = zeroed();
        //
        // let mut retval = IIDFromString(
        //     rust_to_win_str("{B196B284-BAB4-101A-B69C-00AA00341D07}").as_ptr(),
        //     riid
        // );//IConnectionPointContainer guid
        //
        // h_result = (*pp_unk_control).QueryInterface(
        //     riid,
        //     &mut p_connect_contain as *mut *mut IConnectionPointContainer as *mut *mut c_void);
        //
        // let mut retval = IIDFromString(
        //     rust_to_win_str("{7335F12D-8973-4BD5-B7F0-12DF03D175B7}").as_ptr(),
        //     riid
        // );//키움 이벤트 인터페이스
        //
        //
        //
        // h_result = (*p_connect_contain).FindConnectionPoint(
        //     riid, &mut p_kh_sink as *mut *mut IConnectionPoint);
        //
        // h_result = (*p_kh_sink).Advise(  p_event as *mut IUnknown, &mut dw_cookie);
        //
        // (*p_connect_contain).Release();



        if SUCCEEDED(h_result) {

            Some(Kiwoom {
                con: con,
                hwnd: hwnd,
                com_container: ComPtr::new(pp_unk_container).unwrap(),
                com_control: ComPtr::new(pp_unk_control).unwrap(),
                comp_kiwoom: ComPtr::new(p_kh_interface).unwrap(),
                com_dkh_ev: ComPtr::new(p_event as *mut IUnknown).unwrap(),
                com_dw_cookie: &mut dw_cookie,
            })
        } else {
            println!("키움 싱크 콘트롤 실패 {}, {}", GetLastError(), h_result);
            None
        }

    }
    pub fn comm_connect(&self) -> LONG {
        invoke_wrap!( self.comp_kiwoom, 0x1, DISPATCH_METHOD, VT_I4, 0 )
    }

    fn CommRqData(&self, sRQName: &mut BSTR,
                  sTrCode: &mut BSTR,
                  nPrevNext: &mut LONG,
                  sScreenNo: &mut BSTR) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x3, DISPATCH_METHOD, VT_I4, 4,
                    sRQName,VT_BSTR,
                   sTrCode,VT_BSTR,
                   nPrevNext,VT_I4,
                   sScreenNo,VT_BSTR )
    }
    //
    pub fn GetLoginInfo(&self, sTag: &mut BSTR) -> BSTR {
        invoke_wrap!(self.comp_kiwoom, 0x4, DISPATCH_METHOD, VT_BSTR, 1,
                    sTag,VT_BSTR )
    }

    fn SendOrder(&self,
                 sRQName: &mut BSTR,
                 sScreenNo: &mut BSTR,
                 sAccNo: &mut BSTR,
                 nOrderType: &mut LONG,
                 sCode: &mut BSTR,
                 nQty: &mut LONG,
                 nPrice: &mut LONG,
                 sHogaGb: &mut BSTR,
                 sOrgOrderNo: &mut BSTR) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x5, DISPATCH_METHOD, VT_I4, 9,
                            sRQName,VT_BSTR,
                            sScreenNo,VT_BSTR,
                            sAccNo,VT_BSTR,
                            nOrderType,VT_I4,
                            sCode,VT_BSTR,
                            nQty,VT_I4,
                            nPrice,VT_I4,
                            sHogaGb,VT_BSTR,
                            sOrgOrderNo,VT_BSTR )
    }

    fn SendOrderFO(&self,
                   sRQName: &mut BSTR,
                   sScreenNo: &mut BSTR,
                   sAccNo: &mut BSTR,
                   sCode: &mut BSTR,
                   lOrdKind: &mut LONG,
                   sSlbyTp: &mut BSTR,
                   sOrdTp: &mut BSTR,
                   lQty: &mut LONG,
                   sPrice: &mut BSTR,
                   sOrgOrdNo: &mut BSTR) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x6, DISPATCH_METHOD, VT_I4, 10,
                    sRQName,VT_BSTR,
                    sScreenNo,VT_BSTR,
                    sAccNo,VT_BSTR,
                    sCode,VT_BSTR,
                    lOrdKind,VT_I4,
                    sSlbyTp,VT_BSTR,
                    sOrdTp,VT_BSTR,
                    lQty,VT_I4,
                    sPrice,VT_BSTR,
                    sOrgOrdNo,VT_BSTR )
    }

    fn SetInputValue(&self,
                     sID: &mut BSTR,
                     sValue: &mut BSTR) {
        invoke_wrap!(self.comp_kiwoom, 0x7, DISPATCH_METHOD, VOID, 2,
                            sID,VT_BSTR,
                            sValue,VT_BSTR );
    }

    fn SetOutputFID(&self, sID: &mut BSTR) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x8, DISPATCH_METHOD, VT_I4, 1,
                    sID,VT_BSTR )
    }

    fn CommGetData(&self,
                   sJongmokCode: &mut BSTR,
                   sRealType: &mut BSTR,
                   sFieldName: &mut BSTR,
                   nIndex: &mut LONG,
                   sInnerFieldName: &mut BSTR) -> BSTR {
        invoke_wrap!(self.comp_kiwoom, 0x9, DISPATCH_METHOD, VT_BSTR, 5,
                    sJongmokCode,VT_BSTR,
                    sRealType,VT_BSTR,
                    sFieldName,VT_BSTR,
                    nIndex, VT_I4,
                    sInnerFieldName,VT_BSTR)
    }

    fn DisconnectRealData(&self, sScnNo: &mut BSTR) {
        invoke_wrap!(self.comp_kiwoom, 0xa, DISPATCH_METHOD, VOID, 1,
                    sScnNo,VT_BSTR );
    }

    fn GetRepeatCnt(&self,
                    sTrCode: &mut BSTR,
                    sRecordName: &mut BSTR) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0xb, DISPATCH_METHOD, VT_I4, 2,
                    sTrCode,VT_BSTR,
                    sRecordName,VT_BSTR )
    }

//     fn CommKwRqData( &self,
//         sArrCode:&mut BSTR,
//         bNext:&mut LONG,
//         nCodeCount:&mut INT,
//         nTypeFlag:&mut INT,
//         sRQName:&mut BSTR,
//         sScreenNo:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0xc, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetAPIModulePath( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0xd, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetCodeListByMarket( &self, sMarket:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0xe, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetConnectState( &self)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0xf, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterCodeName( &self, sTrCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x10, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterListedStockCnt( &self,sTrCode:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x11, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterConstruction( &self, sTrCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x12, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterListedStockDate( &self, sTrCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x13, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterLastPrice( &self, sTrCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x14, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMasterStockState( &self, sTrCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x15, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetDataCount( &self, strRecordName:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x16, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetOutputValue( &self,
//         strRecordName:&mut BSTR,
//         nRepeatIdx:&mut LONG,
//         nItemIdx:&mut LONG)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x17, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetCommData( &self,
//         strTrCode:&mut BSTR,
//         strRecordName:&mut BSTR,
//         nIndex:&mut LONG,
//         strItemName:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x18, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetCommRealData( &self,
//         sTrCode:&mut BSTR,
//         nFid:&mut LONG)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x19, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetChejanData( &self,nFid:&mut LONG)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1a, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetThemeGroupList( &self,nType:&mut LONG)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1b, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetThemeGroupCode( &self, strThemeCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1c, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetFutureList( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1d, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetFutureCodeByIndex( &self,nIndex:&mut INT)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1e, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetActPriceList( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x1f, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetMonthList( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x20, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetOptionCode( &self,
//         strActPrice:&mut BSTR,
//         nCp:&mut INT,
//         strMonth:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x21, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetOptionCodeByMonth( &self,
//         sTrCode:&mut BSTR,
//         nCp:&mut INT,
//         strMonth:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x22, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetOptionCodeByActPrice( &self,
//         sTrCode:&mut BSTR,
//         nCp:&mut INT,
//         nTick:&mut INT)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x23, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSFutureList( &self, strBaseAssetCode:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x24, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSFutureCodeByIndex( &self,
//         strBaseAssetCode:&mut BSTR,
//         nIndex:&mut INT)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x25, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSActPriceList( &self, strBaseAssetGb:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x26, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSMonthList( &self, strBaseAssetGb:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x27, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSOptionCode( &self,
//         strBaseAssetGb:&mut BSTR,
//         strActPrice:&mut BSTR,
//         nCp:&mut INT,
//         strMonth:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x28, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSOptionCodeByMonth( &self,
//         strBaseAssetGb:&mut BSTR,
//         sTrCode:&mut BSTR,
//         nCp:&mut INT,
//         strMonth:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x29, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSOptionCodeByActPrice( &self,
//         strBaseAssetGb:&mut BSTR,
//         sTrCode:&mut BSTR,
//         nCp:&mut INT,
//         nTick:&mut INT)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2a, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSFOBasisAssetList( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2b, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetOptionATM( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2c, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetSOptionATM( &self, strBaseAssetGb:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2d, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetBranchCodeName( &self)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2e, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn CommInvestRqData( &self,
//         sMarketGb:&mut BSTR,
//         sRQName:&mut BSTR,
//         sScreenNo:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x2f, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn SendOrderCredit( &self,
//         sRQName:&mut BSTR,
//         sScreenNo:&mut BSTR,
//         sAccNo:&mut BSTR,
//         nOrderType:&mut LONG,
//         sCode:&mut BSTR,
//         nQty:&mut LONG,
//         nPrice:&mut LONG,
//         sHogaGb:&mut BSTR,
//         sCreditGb:&mut BSTR,
//         sLoanDate:&mut BSTR,
//         sOrgOrderNo:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x30, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn KOA_Functions( &self,
//         sFunctionName:&mut BSTR,
//         sParam:&mut BSTR)-> BSTR {
//
//         invoke_wrap!(self.comp_kiwoom, 0x31, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn SetInfoData( &self,sInfoData:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x32, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn SetRealReg( &self,
//         strScreenNo:&mut BSTR,
//         strCodeList:&mut BSTR,
//         strFidList:&mut BSTR,
//         strOptType:&mut BSTR)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x33, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetConditionLoad( &self)->LONG {
//
//         invoke_wrap!(self.comp_kiwoom, 0x34, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn GetConditionNameList( &self)-> BSTR {
//         invoke_wrap!(self.comp_kiwoom, 0x35, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn SendCondition( &self,
//         strScrNo:&mut BSTR,
//         strConditionName:&mut BSTR,
//         nIndex:&mut INT,
//         nSearch:&mut INT)->LONG {
//         invoke_wrap!(self.comp_kiwoom, 0x36, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
//     }
//
//     fn SendConditionStop( &self,
//         strScrNo:&mut BSTR,
//         strConditionName:&mut BSTR,
//         nIndex:&mut INT){
//         invoke_wrap!(self.comp_kiwoom, 0x37, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn GetCommDataEx( &self,
//         strTrCode:&mut BSTR,
//         strRecordName:&mut BSTR)->VARIANT {
//         invoke_wrap!(self.comp_kiwoom, 0x38, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn SetRealRemove( &self,
//         strScrNo:&mut BSTR,
//         strDelCode:&mut BSTR){
//         invoke_wrap!(self.comp_kiwoom, 0x39, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn GetMarketType( &self,sTrCode:&mut BSTR)->LONG {
//         invoke_wrap!(self.comp_kiwoom, 0x3a, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
}



// ////////////////////////////////////
// ///Event  메써드
// /////////////////////////////////////
//     fn OnReceiveTrData( &self,
//         sScrNo:&mut BSTR ,
//         sRQName:&mut BSTR ,
//         sTrCode:&mut BSTR ,
//         sRecordName:&mut BSTR ,
//         sPrevNext:&mut BSTR ,
//         nDataLength:&mut LONG,
//         sErrorCode:&mut BSTR ,
//         sMessage:&mut BSTR ,
//         sSplmMsg:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x1, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveRealData( &self,
//         sRealKey:&mut BSTR ,
//         sRealType:&mut BSTR ,
//         sRealData:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x2, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveMsg( &self,
//         sScrNo:&mut BSTR ,
//         sRQName:&mut BSTR ,
//         sTrCode:&mut BSTR ,
//         sMsg:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x3, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveChejanData( &self,
//         sGubun:&mut BSTR ,
//         nItemCnt:&mut LONG,
//         sFIdList:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x4, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnEventConnect( &self,nErrCode:&mut LONG ){
//         invoke_wrap!(self.comp_kiwoom, 0x5, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveInvestRealData( &self,sRealKey:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x6, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveRealCondition( &self,
//         sTrCode:&mut BSTR ,
//         strType:&mut BSTR ,
//         strConditionName:&mut BSTR ,
//         strConditionIndex:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x7, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveTrCondition( &self,
//         sScrNo:&mut BSTR ,
//         strCodeList:&mut BSTR ,
//         strConditionName:&mut BSTR ,
//         nIndex:&mut INT,
//         nNext:&mut INT ){
//         invoke_wrap!(self.comp_kiwoom, 0x8, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }
//
//     fn OnReceiveConditionVer( &self,
//         lRet:&mut LONG,
//         sMsg:&mut BSTR  ){
//         invoke_wrap!(self.comp_kiwoom, 0x9, DISPATCH_METHOD, VT_BSTR, 1,
//                     sTag,VT_BSTR )
// }

// RIDL!{#[uuid(0xa1574a0d,0x6bfa,0x4bd7,0x90,0x20,0xde,0xd8,0x87,0x11,0x81,0x8d)]class KHOpenAPI;
// }


/****

	CString GetLoginInfo(LPCTSTR sTag)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x4, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTag);
		return result;
	}
	long SendOrder(LPCTSTR sRQName, LPCTSTR sScreenNo, LPCTSTR sAccNo, long nOrderType, LPCTSTR sCode, long nQty, long nPrice, LPCTSTR sHogaGb, LPCTSTR sOrgOrderNo)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR VTS_I4 VTS_I4 VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x5, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sRQName, sScreenNo, sAccNo, nOrderType, sCode, nQty, nPrice, sHogaGb, sOrgOrderNo);
		return result;
	}
	long SendOrderFO(LPCTSTR sRQName, LPCTSTR sScreenNo, LPCTSTR sAccNo, LPCTSTR sCode, long lOrdKind, LPCTSTR sSlbyTp, LPCTSTR sOrdTp, long lQty, LPCTSTR sPrice, LPCTSTR sOrgOrdNo)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x6, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sRQName, sScreenNo, sAccNo, sCode, lOrdKind, sSlbyTp, sOrdTp, lQty, sPrice, sOrgOrdNo);
		return result;
	}
	void SetInputValue(LPCTSTR sID, LPCTSTR sValue)
	{
		static BYTE parms[] = VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x7, DISPATCH_METHOD, VT_EMPTY, NULL, parms, sID, sValue);
	}
	long SetOutputFID(LPCTSTR sID)
	{
		long result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x8, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sID);
		return result;
	}
	CString CommGetData(LPCTSTR sJongmokCode, LPCTSTR sRealType, LPCTSTR sFieldName, long nIndex, LPCTSTR sInnerFieldName)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x9, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sJongmokCode, sRealType, sFieldName, nIndex, sInnerFieldName);
		return result;
	}
	void DisconnectRealData(LPCTSTR sScnNo)
	{
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0xa, DISPATCH_METHOD, VT_EMPTY, NULL, parms, sScnNo);
	}
	long GetRepeatCnt(LPCTSTR sTrCode, LPCTSTR sRecordName)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR ;
		InvokeHelper(0xb, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sTrCode, sRecordName);
		return result;
	}
	long CommKwRqData(LPCTSTR sArrCode, long bNext, long nCodeCount, long nTypeFlag, LPCTSTR sRQName, LPCTSTR sScreenNo)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_I4 VTS_I4 VTS_I4 VTS_BSTR VTS_BSTR ;
		InvokeHelper(0xc, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sArrCode, bNext, nCodeCount, nTypeFlag, sRQName, sScreenNo);
		return result;
	}
	CString GetAPIModulePath()
	{
		CString result;
		InvokeHelper(0xd, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetCodeListByMarket(LPCTSTR sMarket)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0xe, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sMarket);
		return result;
	}
	long GetConnectState()
	{
		long result;
		InvokeHelper(0xf, DISPATCH_METHOD, VT_I4, (void*)&result, NULL);
		return result;
	}
	CString GetMasterCodeName(LPCTSTR sTrCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x10, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode);
		return result;
	}
	long GetMasterListedStockCnt(LPCTSTR sTrCode)
	{
		long result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x11, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sTrCode);
		return result;
	}
	CString GetMasterConstruction(LPCTSTR sTrCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x12, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode);
		return result;
	}
	CString GetMasterListedStockDate(LPCTSTR sTrCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x13, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode);
		return result;
	}
	CString GetMasterLastPrice(LPCTSTR sTrCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x14, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode);
		return result;
	}
	CString GetMasterStockState(LPCTSTR sTrCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x15, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode);
		return result;
	}
	long GetDataCount(LPCTSTR strRecordName)
	{
		long result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x16, DISPATCH_METHOD, VT_I4, (void*)&result, parms, strRecordName);
		return result;
	}
	CString GetOutputValue(LPCTSTR strRecordName, long nRepeatIdx, long nItemIdx)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 VTS_I4 ;
		InvokeHelper(0x17, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strRecordName, nRepeatIdx, nItemIdx);
		return result;
	}
	CString GetCommData(LPCTSTR strTrCode, LPCTSTR strRecordName, long nIndex, LPCTSTR strItemName)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x18, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strTrCode, strRecordName, nIndex, strItemName);
		return result;
	}
	CString GetCommRealData(LPCTSTR sTrCode, long nFid)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 ;
		InvokeHelper(0x19, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode, nFid);
		return result;
	}
	CString GetChejanData(long nFid)
	{
		CString result;
		static BYTE parms[] = VTS_I4 ;
		InvokeHelper(0x1a, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, nFid);
		return result;
	}
	CString GetThemeGroupList(long nType)
	{
		CString result;
		static BYTE parms[] = VTS_I4 ;
		InvokeHelper(0x1b, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, nType);
		return result;
	}
	CString GetThemeGroupCode(LPCTSTR strThemeCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x1c, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strThemeCode);
		return result;
	}
	CString GetFutureList()
	{
		CString result;
		InvokeHelper(0x1d, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetFutureCodeByIndex(long nIndex)
	{
		CString result;
		static BYTE parms[] = VTS_I4 ;
		InvokeHelper(0x1e, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, nIndex);
		return result;
	}
	CString GetActPriceList()
	{
		CString result;
		InvokeHelper(0x1f, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetMonthList()
	{
		CString result;
		InvokeHelper(0x20, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetOptionCode(LPCTSTR strActPrice, long nCp, LPCTSTR strMonth)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x21, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strActPrice, nCp, strMonth);
		return result;
	}
	CString GetOptionCodeByMonth(LPCTSTR sTrCode, long nCp, LPCTSTR strMonth)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x22, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode, nCp, strMonth);
		return result;
	}
	CString GetOptionCodeByActPrice(LPCTSTR sTrCode, long nCp, long nTick)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 VTS_I4 ;
		InvokeHelper(0x23, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sTrCode, nCp, nTick);
		return result;
	}
	CString GetSFutureList(LPCTSTR strBaseAssetCode)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x24, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetCode);
		return result;
	}
	CString GetSFutureCodeByIndex(LPCTSTR strBaseAssetCode, long nIndex)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_I4 ;
		InvokeHelper(0x25, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetCode, nIndex);
		return result;
	}
	CString GetSActPriceList(LPCTSTR strBaseAssetGb)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x26, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb);
		return result;
	}
	CString GetSMonthList(LPCTSTR strBaseAssetGb)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x27, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb);
		return result;
	}
	CString GetSOptionCode(LPCTSTR strBaseAssetGb, LPCTSTR strActPrice, long nCp, LPCTSTR strMonth)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x28, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb, strActPrice, nCp, strMonth);
		return result;
	}
	CString GetSOptionCodeByMonth(LPCTSTR strBaseAssetGb, LPCTSTR sTrCode, long nCp, LPCTSTR strMonth)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR ;
		InvokeHelper(0x29, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb, sTrCode, nCp, strMonth);
		return result;
	}
	CString GetSOptionCodeByActPrice(LPCTSTR strBaseAssetGb, LPCTSTR sTrCode, long nCp, long nTick)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I4 VTS_I4 ;
		InvokeHelper(0x2a, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb, sTrCode, nCp, nTick);
		return result;
	}
	CString GetSFOBasisAssetList()
	{
		CString result;
		InvokeHelper(0x2b, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetOptionATM()
	{
		CString result;
		InvokeHelper(0x2c, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	CString GetSOptionATM(LPCTSTR strBaseAssetGb)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR ;
		InvokeHelper(0x2d, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, strBaseAssetGb);
		return result;
	}
	CString GetBranchCodeName()
	{
		CString result;
		InvokeHelper(0x2e, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	long CommInvestRqData(LPCTSTR sMarketGb, LPCTSTR sRQName, LPCTSTR sScreenNo)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x2f, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sMarketGb, sRQName, sScreenNo);
		return result;
	}
	long SendOrderCredit(LPCTSTR sRQName, LPCTSTR sScreenNo, LPCTSTR sAccNo, long nOrderType, LPCTSTR sCode, long nQty, long nPrice, LPCTSTR sHogaGb, LPCTSTR sCreditGb, LPCTSTR sLoanDate, LPCTSTR sOrgOrderNo)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR VTS_I4 VTS_BSTR VTS_I4 VTS_I4 VTS_BSTR VTS_BSTR VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x30, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sRQName, sScreenNo, sAccNo, nOrderType, sCode, nQty, nPrice, sHogaGb, sCreditGb, sLoanDate, sOrgOrderNo);
		return result;
	}
	CString KOA_Functions(LPCTSTR sFunctionName, LPCTSTR sParam)
	{
		CString result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR ;
		InvokeHelper(0x31, DISPATCH_METHOD, VT_BSTR, (void*)&result, parms, sFunctionName, sParam);
		return result;
	}
	long SetInfoData(LPCTSTR sInfoData)
	{
		long result;
		static BYTE parms[] = VTS_BSTR;
		InvokeHelper(0x32, DISPATCH_METHOD, VT_I4, (void*)&result, parms, sInfoData);
		return result;
	}
	long SetRealReg(LPCTSTR strScreenNo, LPCTSTR strCodeList, LPCTSTR strFidList, LPCTSTR strOptType)
	{
		long result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_BSTR VTS_BSTR;
		InvokeHelper(0x33, DISPATCH_METHOD, VT_I4, (void*)&result, parms, strScreenNo, strCodeList, strFidList, strOptType);
		return result;
	}
	long GetConditionLoad()
	{
		long result;
		InvokeHelper(0x34, DISPATCH_METHOD, VT_I4, (void*)&result, NULL);
		return result;
	}
	CString GetConditionNameList()
	{
		CString result;
		InvokeHelper(0x35, DISPATCH_METHOD, VT_BSTR, (void*)&result, NULL);
		return result;
	}
	BOOL SendCondition(LPCTSTR strScrNo, LPCTSTR strConditionName, int nIndex, int nSearch)
	{
		BOOL result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I2 VTS_I2;
		InvokeHelper(0x36, DISPATCH_METHOD, VT_BOOL, (void*)&result, parms, strScrNo, strConditionName, nIndex, nSearch);
		return result;
	}
	void SendConditionStop(LPCTSTR strScrNo, LPCTSTR strConditionName, int nIndex)
	{
		static BYTE parms[] = VTS_BSTR VTS_BSTR VTS_I2;
		InvokeHelper(0x37, DISPATCH_METHOD, VT_EMPTY, NULL, parms, strScrNo, strConditionName, nIndex);
	}
	VARIANT GetCommDataEx(LPCTSTR strTrCode, LPCTSTR strRecordName)
	{
		VARIANT result;
		static BYTE parms[] = VTS_BSTR VTS_BSTR;
		InvokeHelper(0x38, DISPATCH_METHOD, VT_VARIANT, (void*)&result, parms, strTrCode, strRecordName);
		return result;
	}
	void SetRealRemove(LPCTSTR strScrNo, LPCTSTR strDelCode)
	{
		static BYTE parms[] = VTS_BSTR VTS_BSTR;
		InvokeHelper(0x39, DISPATCH_METHOD, VT_EMPTY, NULL, parms, strScrNo, strDelCode);
	}
	long GetMarketType(LPCTSTR strCode)
	{
		long result;
		static BYTE parms[] = VTS_BSTR;
		InvokeHelper(0x3a, DISPATCH_METHOD, VT_I4, (void*)&result, parms, strCode);
		return result;
	}

******/





