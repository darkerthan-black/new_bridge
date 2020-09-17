
extern crate invoke_wrap;
extern crate oaidl;
extern crate widestring;


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


use widestring::U16String;
use widestring::UCString;
use winapi::um::oaidl::VARIANT;

use oaidl::{BStringExt, IntoVariantError, VariantExt, Ptr};
use self::oaidl::Variant;
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
        //   AtlAxCreateContorlEx 사용시 한번에 sink 클래스까지 등록하는 경우 아래 IConnectionPoint 관련 코드는 불필요하다.
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


        // let mut dw_cookie: DWORD = 0;
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

            })
        } else {
            println!("키움 싱크 콘트롤 실패 {}, {}", GetLastError(), h_result);
            None
        }

    }
    pub fn comm_connect(&self) -> LONG {
        invoke_wrap!( self.comp_kiwoom, 0x1, DISPATCH_METHOD, VT_I4, 0 )





    }

    fn CommRqData(&self, sRQName: &str,
                  sTrCode: &str,
                  nPrevNext: LONG,
                  sScreenNo: &str) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x3, DISPATCH_METHOD, VT_I4, 4,
                    sRQName,VT_BSTR,
                   sTrCode,VT_BSTR,
                   nPrevNext,VT_I4,
                   sScreenNo,VT_BSTR )

    }
    //
    pub fn GetLoginInfo(&self, sTag:&str) -> String {
        invoke_wrap!(self.comp_kiwoom, 0x4, DISPATCH_METHOD, VT_BSTR, 1,
                    sTag,VT_BSTR )


    }

    fn SendOrder(&self,
                 sRQName: &str,
                 sScreenNo: &str,
                 sAccNo: &str,
                 nOrderType: LONG,
                 sCode: &str,
                 nQty: LONG,
                 nPrice: LONG,
                 sHogaGb: &str,
                 sOrgOrderNo: &str) -> LONG {
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
                   sRQName: &str,
                   sScreenNo: &str,
                   sAccNo: &str,
                   sCode: &str,
                   lOrdKind: LONG,
                   sSlbyTp: &str,
                   sOrdTp: &str,
                   lQty: LONG,
                   sPrice: &str,
                   sOrgOrdNo: &str) -> LONG {
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
                     sID: &str,
                     sValue: &str) {
        invoke_wrap!(self.comp_kiwoom, 0x7, DISPATCH_METHOD, VT_VOID, 2,
                            sID,VT_BSTR,
                            sValue,VT_BSTR );
    }

    fn SetOutputFID(&self, sID: &str) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0x8, DISPATCH_METHOD, VT_I4, 1,
                    sID,VT_BSTR )
    }

    fn CommGetData(&self,
                   sJongmokCode: &str,
                   sRealType: &str,
                   sFieldName: &str,
                   nIndex: LONG,
                   sInnerFieldName: &str) -> String {
        invoke_wrap!(self.comp_kiwoom, 0x9, DISPATCH_METHOD, VT_BSTR, 5,
                    sJongmokCode,VT_BSTR,
                    sRealType,VT_BSTR,
                    sFieldName,VT_BSTR,
                    nIndex, VT_I4,
                    sInnerFieldName,VT_BSTR)
    }

    fn DisconnectRealData(&self, sScnNo: &str) {
        invoke_wrap!(self.comp_kiwoom, 0xa, DISPATCH_METHOD, VT_VOID, 1,
                    sScnNo,VT_BSTR );
    }

    fn GetRepeatCnt(&self,
                    sTrCode: &str,
                    sRecordName: &str) -> LONG {
        invoke_wrap!(self.comp_kiwoom, 0xb, DISPATCH_METHOD, VT_I4, 2,
                    sTrCode,VT_BSTR,
                    sRecordName,VT_BSTR )
    }

    //반드시 테스트 해봐야 하는 함수  INT를 c++ 예제에서는 long 으로 처리
    fn CommKwRqData( &self,
        sArrCode:&str,
        bNext:LONG,
        nCodeCount:INT,
        nTypeFlag:INT,
        sRQName:&str,
        sScreenNo:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0xc, DISPATCH_METHOD, VT_BSTR, 6,
                    sArrCode,VT_BSTR,
        bNext,VT_I4,
        nCodeCount,VT_INT,
        nTypeFlag,VT_INT,
        sRQName,VT_BSTR,
        sScreenNo,VT_BSTR )
    }

    fn GetAPIModulePath( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0xd, DISPATCH_METHOD, VT_BSTR, 0
                   )
    }

    fn GetCodeListByMarket( &self, sMarket:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0xe, DISPATCH_METHOD, VT_BSTR, 1,
                    sMarket,VT_BSTR )
    }

    fn GetConnectState( &self)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0xf, DISPATCH_METHOD, VT_I4, 0
                    )
    }

    fn GetMasterCodeName( &self, sTrCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x10, DISPATCH_METHOD, VT_BSTR, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetMasterListedStockCnt( &self,sTrCode:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x11, DISPATCH_METHOD, VT_I4, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetMasterConstruction( &self, sTrCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x12, DISPATCH_METHOD, VT_BSTR, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetMasterListedStockDate( &self, sTrCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x13, DISPATCH_METHOD, VT_BSTR, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetMasterLastPrice( &self, sTrCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x14, DISPATCH_METHOD, VT_BSTR, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetMasterStockState( &self, sTrCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x15, DISPATCH_METHOD, VT_BSTR, 1,
                    sTrCode,VT_BSTR )
    }

    fn GetDataCount( &self, strRecordName:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x16, DISPATCH_METHOD, VT_I4, 1,
                    strRecordName,VT_BSTR )
    }

    fn GetOutputValue( &self,
        strRecordName:&str,
        nRepeatIdx:LONG,
        nItemIdx:LONG)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x17, DISPATCH_METHOD, VT_BSTR, 3,
                            strRecordName, VT_BSTR,
        nRepeatIdx,VT_I4,
        nItemIdx,VT_I4 )
    }

    fn GetCommData( &self,
        strTrCode:&str,
        strRecordName:&str,
        nIndex:LONG,
        strItemName:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x18, DISPATCH_METHOD, VT_BSTR, 4,
                            strTrCode,VT_BSTR,
        strRecordName,VT_BSTR,
        nIndex,VT_I4,
        strItemName,VT_BSTR)
    }

    fn GetCommRealData( &self,
        sTrCode:&str,
        nFid:LONG)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x19, DISPATCH_METHOD, VT_BSTR, 2,
                           sTrCode,VT_BSTR,
        nFid,VT_I4 )
    }

    fn GetChejanData( &self,nFid:LONG)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1a, DISPATCH_METHOD, VT_BSTR, 1,
                    nFid,VT_I4 )
    }

    fn GetThemeGroupList( &self,nType:LONG)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1b, DISPATCH_METHOD, VT_BSTR, 1,
                    nType,VT_I4 )
    }

    fn GetThemeGroupCode( &self, strThemeCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1c, DISPATCH_METHOD, VT_BSTR, 1,
                    strThemeCode,VT_BSTR )
    }

    fn GetFutureList( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1d, DISPATCH_METHOD, VT_BSTR, 0
                    )
    }

    fn GetFutureCodeByIndex( &self,nIndex:INT)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1e, DISPATCH_METHOD, VT_BSTR, 1,
                    nIndex,VT_INT )
    }

    fn GetActPriceList( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x1f, DISPATCH_METHOD, VT_BSTR, 0
                    )
    }

    fn GetMonthList( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x20, DISPATCH_METHOD, VT_BSTR, 0
                     )
    }

    fn GetOptionCode( &self,
        strActPrice:&str,
        nCp:INT,
        strMonth:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x21, DISPATCH_METHOD, VT_BSTR, 3,
                    strActPrice,VT_BSTR,
        nCp,VT_INT,
        strMonth,VT_BSTR )
    }

    fn GetOptionCodeByMonth( &self,
        sTrCode:&str,
        nCp:INT,
        strMonth:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x22, DISPATCH_METHOD, VT_BSTR, 3,
                    sTrCode,VT_BSTR,
        nCp,VT_INT,
        strMonth,VT_BSTR )
    }

    fn GetOptionCodeByActPrice( &self,
        sTrCode:&str,
        nCp:INT,
        nTick:INT)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x23, DISPATCH_METHOD, VT_BSTR, 3,
                    sTrCode,VT_BSTR,
        nCp,VT_INT,
        nTick,VT_INT )
    }

    fn GetSFutureList( &self, strBaseAssetCode:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x24, DISPATCH_METHOD, VT_BSTR, 1,
                    strBaseAssetCode,VT_BSTR )
    }

    fn GetSFutureCodeByIndex( &self,
        strBaseAssetCode:&str,
        nIndex:INT)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x25, DISPATCH_METHOD, VT_BSTR, 2,
                     strBaseAssetCode,VT_BSTR,
        nIndex,VT_INT )
    }

    fn GetSActPriceList( &self, strBaseAssetGb:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x26, DISPATCH_METHOD, VT_BSTR, 1,
                    strBaseAssetGb,VT_BSTR )
    }

    fn GetSMonthList( &self, strBaseAssetGb:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x27, DISPATCH_METHOD, VT_BSTR, 1,
                    strBaseAssetGb,VT_BSTR )
    }

    fn GetSOptionCode( &self,
        strBaseAssetGb:&str,
        strActPrice:&str,
        nCp:INT,
        strMonth:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x28, DISPATCH_METHOD, VT_BSTR, 4,
                            strBaseAssetGb,VT_BSTR,
        strActPrice,VT_BSTR,
        nCp,VT_INT,
        strMonth,VT_BSTR )
    }

    fn GetSOptionCodeByMonth( &self,
        strBaseAssetGb:&str,
        sTrCode:&str,
        nCp:INT,
        strMonth:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x29, DISPATCH_METHOD, VT_BSTR, 4,
                    strBaseAssetGb,VT_BSTR,
        sTrCode,VT_BSTR,
        nCp,VT_INT,
        strMonth,VT_BSTR)
    }

    fn GetSOptionCodeByActPrice( &self,
        strBaseAssetGb:&str,
        sTrCode:&str,
        nCp:INT,
        nTick:INT)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x2a, DISPATCH_METHOD, VT_BSTR, 4,
                     strBaseAssetGb,VT_BSTR,
        sTrCode,VT_BSTR,
        nCp,VT_INT,
        nTick,VT_INT)
    }

    fn GetSFOBasisAssetList( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x2b, DISPATCH_METHOD, VT_BSTR, 0
                    )
    }

    fn GetOptionATM( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x2c, DISPATCH_METHOD, VT_BSTR, 0
                     )
    }

    fn GetSOptionATM( &self, strBaseAssetGb:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x2d, DISPATCH_METHOD, VT_BSTR, 1,
                    strBaseAssetGb,VT_BSTR )
    }

    fn GetBranchCodeName( &self)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x2e, DISPATCH_METHOD, VT_BSTR, 0
                     )
    }

    fn CommInvestRqData( &self,
        sMarketGb:&str,
        sRQName:&str,
        sScreenNo:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x2f, DISPATCH_METHOD, VT_I4, 3,
                    sMarketGb,VT_BSTR,
        sRQName,VT_BSTR,
        sScreenNo,VT_BSTR )
    }

    fn SendOrderCredit( &self,
        sRQName:&str,
        sScreenNo:&str,
        sAccNo:&str,
        nOrderType:LONG,
        sCode:&str,
        nQty:LONG,
        nPrice:LONG,
        sHogaGb:&str,
        sCreditGb:&str,
        sLoanDate:&str,
        sOrgOrderNo:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x30, DISPATCH_METHOD, VT_I4, 11,
                    sRQName,VT_BSTR,
        sScreenNo,VT_BSTR,
        sAccNo,VT_BSTR,
        nOrderType,VT_I4,
        sCode,VT_BSTR,
        nQty,VT_I4,
        nPrice,VT_I4,
        sHogaGb,VT_BSTR,
        sCreditGb,VT_BSTR,
        sLoanDate,VT_BSTR,
        sOrgOrderNo,VT_BSTR)
    }

    fn KOA_Functions( &self,
        sFunctionName:&str,
        sParam:&str)-> String {

        invoke_wrap!(self.comp_kiwoom, 0x31, DISPATCH_METHOD, VT_BSTR, 2,
                    sFunctionName,VT_BSTR,
        sParam,VT_BSTR )
    }

    fn SetInfoData( &self,sInfoData:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x32, DISPATCH_METHOD, VT_I4, 1,
                    sInfoData,VT_BSTR )
    }

    fn SetRealReg( &self,
        strScreenNo:&str,
        strCodeList:&str,
        strFidList:&str,
        strOptType:&str)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x33, DISPATCH_METHOD, VT_I4, 4,
                    strScreenNo,VT_BSTR,
        strCodeList,VT_BSTR,
        strFidList,VT_BSTR,
        strOptType,VT_BSTR)
    }

    fn GetConditionLoad( &self)->LONG {

        invoke_wrap!(self.comp_kiwoom, 0x34, DISPATCH_METHOD, VT_I4, 0
                    )
    }

    fn GetConditionNameList( &self)-> String {
        invoke_wrap!(self.comp_kiwoom, 0x35, DISPATCH_METHOD, VT_BSTR, 0
                     )
    }

    fn SendCondition( &self,
        strScrNo:&str,
        strConditionName:&str,
        nIndex:INT,
        nSearch:INT)->LONG {
        invoke_wrap!(self.comp_kiwoom, 0x36, DISPATCH_METHOD, VT_I4, 4,
                    strScrNo,VT_BSTR,
        strConditionName,VT_BSTR,
        nIndex,VT_INT,
        nSearch,VT_INT )
    }

    fn SendConditionStop( &self,
        strScrNo:&str,
        strConditionName:&str,
        nIndex:INT){
        invoke_wrap!(self.comp_kiwoom, 0x37, DISPATCH_METHOD, VT_VOID, 3,
                    strScrNo,VT_BSTR,
        strConditionName,VT_BSTR,
        nIndex,VT_INT )
}

    fn GetCommDataEx( &self,
        strTrCode:&str,
        strRecordName:&str)->*mut VARIANT {
        invoke_wrap!(self.comp_kiwoom, 0x38, DISPATCH_METHOD, VT_VARIANT, 2,
                    strTrCode,VT_BSTR,
        strRecordName,VT_BSTR )
}

    fn SetRealRemove( &self,
        strScrNo:&str,
        strDelCode:&str){
        invoke_wrap!(self.comp_kiwoom, 0x39, DISPATCH_METHOD, VT_VOID, 2,
                    strScrNo,VT_BSTR,
        strDelCode,VT_BSTR)
}

    fn GetMarketType( &self,sTrCode:&str)->LONG {
        invoke_wrap!(self.comp_kiwoom, 0x3a, DISPATCH_METHOD, VT_I4, 1,
                    sTrCode,VT_BSTR )
}
}




