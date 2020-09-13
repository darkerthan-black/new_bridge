
use winapi::RIDL;
use winapi::um::oaidl::{IDispatch,* };
use winapi::um::unknwnbase::{IUnknown,*};
use winapi::shared::guiddef::{LPIID, GUID, IID_NULL, REFIID, *};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::minwindef::{HMODULE, UINT, WPARAM, LPARAM, LRESULT, LOWORD, *};
//
//
//
//   AtlAxCreateControlEx 사용시 IConnectionPoint 불필요 삭제해도 상관없으나, 후에 참고를 위하여 남겨둠.
//
//
RIDL!{#[uuid(0xB196B286,0xBAB4, 0x101A,0xB6,0x9C,0x0,0xAA,0x0,0x34,0x1D,0x7)]
interface IConnectionPoint(IConnectionPointVtbl): IUnknown(IUnknownVtbl){
        fn  GetConnectionInterface(  pIID:*mut IID ,)-> HRESULT,
        fn  GetConnectionPointContainer(   ppCPC:*mut *mut IConnectionPointContainer,)-> HRESULT,
        fn  Advise(pUnkSink:*mut IUnknown, pdwCookie:*mut DWORD,)-> HRESULT,
        fn  Unadvise(dwCookie:DWORD, )-> HRESULT,
        fn  EnumConnections( ppEnum: *mut *mut IEnumConnectionPoints,)-> HRESULT,
    }
}

RIDL!{#[uuid(0xB196B284,0xBAB4, 0x101A,0xB6,0x9C,0x0,0xAA,0x0,0x34,0x1D,0x7)]
interface IConnectionPointContainer(IConnectionPointContainerVtbl): IUnknown(IUnknownVtbl){
        fn EnumConnectionPoints ( ppEnum :*mut *mut IEnumConnectionPoints,)-> HRESULT,
        fn FindConnectionPoint( riid : REFIID, ppCP:*mut *mut IConnectionPoint ,)-> HRESULT,
    }
}

RIDL!{#[uuid(0xB196B285,0xBAB4, 0x101A,0xB6,0x9C,0x0,0xAA,0x0,0x34,0x1D,0x7)]
interface IEnumConnectionPoints(IEnumConnectionPointsContainerVtbl): IUnknown(IUnknownVtbl){
        fn Clone(ppEnum : *mut *mut IEnumConnectionPoints,)-> HRESULT,
        fn Next(  cConnections :ULONG, ppCP :*mut *mut IEnumConnectionPoints,
               pcFetched:*mut ULONG,)-> HRESULT,
        fn Reset() -> HRESULT,
        fn Skip(cConnections : ULONG ,) -> HRESULT,

    }
}



