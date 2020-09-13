extern crate com;

use com::co_class;
use com::interfaces::*;
use com::*;
use self::com::sys::{NOERROR, HRESULT};
use self::com::IID;
use winapi::ctypes::c_void;
use self::com::interfaces::IUnknown;
// use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::guiddef::{REFIID, REFGUID, IID_NULL, GUID};
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::shared::minwindef::{UINT, WORD};
use winapi::um::winnt::{LCID, LONG, INT};
use winapi::um::oaidl::{DISPID, ITypeInfo, DISPPARAMS, VARIANT, EXCEPINFO, ITypeLib};
use winapi::shared::wtypes::BSTR;
use dlopen::wrapper::{Container, WrapperApi};
use winapi::shared::guiddef;
use winapi::shared::winerror::DISP_E_UNKNOWNINTERFACE;
use winapi::_core::borrow::Borrow;

const IID_NULL_C :GUID = IID_NULL;


#[derive(WrapperApi)]
struct LoadType {
    LoadRegTypeLib: unsafe extern "stdcall" fn(  rguid:REFGUID,
         wVerMajor:WORD,
         wVerMinor:WORD,
         lcid:LCID,
     pptlib:*mut *mut ITypeLib) -> HRESULT,

    DispGetIDsOfNames:  unsafe extern "stdcall" fn(
     ptinfo:*mut ITypeInfo,
      rgszNames:*mut LPOLESTR,
          cNames:UINT,
        rgdispid:*mut DISPID
    )->HRESULT,

}


#[co_class(implements(IDispatch))]
pub struct EventHandle {
    num_owners: u32,
    // pTypeLib:*mut ITypeLib,

}


#[com_interface("00020400-0000-0000-C000-000000000046")]
pub trait IDispatch : IUnknown {

    unsafe fn get_type_info_count(
        &self,
        pctinfo: *mut UINT
    )->HRESULT;


    unsafe fn get_type_info(
        &self,
        iTInfo:UINT,
        lcid:LCID,
        ppTInfo:*mut *mut ITypeInfo
    )->HRESULT;

    unsafe fn get_ids_of_names(
        &self,
        riid: REFIID,
        rgszNames: *mut LPOLESTR ,
        cNames:UINT,
        lcid:LCID,
        rgDispId:*mut DISPID
    )->HRESULT;


    unsafe fn invoke (
    &self,
         dispIdMember:DISPID,
         riid:REFIID,
         lcid:LCID,
         wFlags:WORD,
         pDispParams:*mut DISPPARAMS,
         pVarResult:*mut VARIANT,
         pExcepInfo:*mut EXCEPINFO,
         puArgErr:*mut UINT
    )-> HRESULT;


}


impl IDispatch for EventHandle {
    unsafe fn get_type_info_count(&self, pctinfo: *mut u32) -> i32 {
        (*pctinfo) = 1;
        NOERROR
    }

    unsafe fn get_type_info(&self, iTInfo: u32, lcid: u32, ppTInfo: *mut *mut ITypeInfo) -> i32 {

        unimplemented!()
    }

    unsafe fn get_ids_of_names(&self, riid: *const guiddef::GUID, rgszNames: *mut *mut u16, cNames: u32, lcid: u32, rgDispId: *mut i32) -> i32 {
        unimplemented!()
    }

    unsafe fn invoke(&self, dispIdMember: i32, riid:REFIID, lcid: u32, wFlags: u16, pDispParams: *mut DISPPARAMS, pVarResult: *mut VARIANT, pExcepInfo: *mut EXCEPINFO, puArgErr: *mut u32) -> i32 {

        if riid.eq((&IID_NULL_C as *const GUID).borrow())  {
            DISP_E_UNKNOWNINTERFACE
        } else {
            match dispIdMember {
                // 0x1 => self.OnReceiveTrData()
                // 0x2 => self.OnReceiveRealData()
                // 0x3 => self.OnReceiveMsg()
                // 0x4 => self.OnReceiveChejanData()
                0x5 => self.OnEventConnect( 0),
                // 0x6 => self.OnReceiveInvestRealData()
                // 0x7 => self.OnReceiveRealCondition()
                // 0x8 => self.OnReceiveTrCondition()
                // 0x9 => self.OnReceiveConditionVer()
                _ => {}
            };
            NOERROR
        }



    }
}

impl EventHandle {

    unsafe fn OnReceiveTrData(&self,
                       sScrNo:BSTR ,
                       sRQName:BSTR ,
                       sTrCode:BSTR ,
                       sRecordName:BSTR ,
                       sPrevNext:BSTR ,
                       nDataLength:LONG,
                       sErrorCode:BSTR ,
                       sMessage:BSTR ,
                       sSplmMsg:BSTR  ){}


    unsafe fn OnReceiveRealData(&self,
                         sRealKey:BSTR ,
                         sRealType:BSTR ,
                         sRealData:BSTR  ){}

    unsafe fn OnReceiveMsg(&self,
                    sScrNo:BSTR ,
                    sRQName:BSTR ,
                    sTrCode:BSTR ,
                    sMsg:BSTR  ){}

    unsafe fn OnReceiveChejanData(&self,
                           sGubun:BSTR ,
                           nItemCnt:LONG,
                           sFIdList:BSTR  ) {}

    unsafe fn OnEventConnect(&self,nErrCode:LONG ) {
        println!("**********************************");
        println!(" 접속 이벤트가 발생했습니다. 축하합니다.");
        println!("**********************************");


    }

    unsafe fn OnReceiveInvestRealData(&self,sRealKey:BSTR  ) {}

    unsafe fn OnReceiveRealCondition(&self,
                              sTrCode:BSTR ,
                              strType:BSTR ,
                              strConditionName:BSTR ,
                              strConditionIndex:BSTR  ) {}

    unsafe fn OnReceiveTrCondition(&self,
                            sScrNo:BSTR ,
                            strCodeList:BSTR ,
                            strConditionName:BSTR ,
                            nIndex:INT,
                            nNext:INT ) {}

    unsafe fn OnReceiveConditionVer(&self,
                             lRet:LONG,
                             sMsg:BSTR  ) {}

    pub(crate) fn new() -> Box<EventHandle> {
        let num_owners = 20;
        // let mut cont: Container<LoadType> =
        //     unsafe { Container::load("oleaut32.dll") }.expect("타입 로딩");
        // cont.LoadRegTypeLib(
        //
        // );
        println!("생성자 호출");
        EventHandle::allocate(20)
    }
}