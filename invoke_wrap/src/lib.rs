
//
// proc-macro crate's code
//

// invoke!(function_name, p_inter, disp_id, DISPATCH_METHOD, var_num, v(type, val), ...  )
#[macro_use]
extern crate proc_macro;

use quote::quote;
use quote::format_ident;
use proc_macro::TokenTree::Ident;
use std::env::args;
use syn::export::ToTokens;
use proc_macro2::Span;
use syn::parse::ParseBuffer;
use syn::parse::ParseStream;
use winapi::um::winnt::{LONG, SHORT};


struct ParsedArguments {
    // func_name: syn::Ident,
    p_disinter: syn::ExprField,
    disp_id: syn::LitInt,
    w_flag: syn::Ident,
    return_type: syn::Ident,
    var_num: syn::LitInt,
    arg_types: Vec<syn::Ident>,
}

// enum KiwoomResult {
//     BSTR,
//     LONG,
//     BOOL,
//     VARIANT,
//     HRESULT,
// }

impl syn::parse::Parse for ParsedArguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        // if input.is_empty() {
        //     // Err(syn::Error::new(err.span(), format!(
        //     //     "The first token must be an identifier.")))
        //     Err();
        // }

        let mut parsed_args = ParsedArguments {
            // func_name: input.parse()?,//함수 이름
            p_disinter: {
                // input.parse::<syn::Token![,]>()?;
                input.parse()?},//쉼표를 건너뛰고 파싱
            disp_id: {input.parse::<syn::Token![,]>()?; input.parse()?},
            w_flag: {input.parse::<syn::Token![,]>()?; input.parse()?},
            return_type: {
                input.parse::<syn::Token![,]>()?;input.parse()?
            },
            var_num: {input.parse::<syn::Token![,]>()?; input.parse()?},
            arg_types: {

                let mut args: Vec<syn::Ident> = Vec::new();
                //튜플변수가 제공되는 경우에만 파싱.
                while !input.is_empty() {
                    input.parse::<syn::Token![,]>()?;
                    args.push(input.parse()?);
                }
                args
            },

        };

        Ok(parsed_args)

        }



}

#[proc_macro]
pub fn invoke_wrap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_args = syn::parse_macro_input!(input as ParsedArguments);
    // let func_name = parsed_args.func_name;
    let p_disinter = parsed_args.p_disinter;
    let disp_id = parsed_args.disp_id;
    let w_flag = parsed_args.w_flag;
    let return_type  = parsed_args.return_type;
    let var_num = parsed_args.var_num;
    let arg_types = parsed_args.arg_types;


    //InvokeHelper 매크로와 유사한 기능을 하는 iDispatch invoke 호출
    let mut main_tokens = quote! {



        //DISPPARAMS 초기화
        let mut dp = DISPPARAMS { rgvarg: null_mut(), rgdispidNamedArgs: null_mut(), cArgs: 0, cNamedArgs: 0, };
        let p_dp: *mut DISPPARAMS = &mut dp;




        //예외 정보를 저장할 포인터 변수. 예외 미구현
        let mut exception_info: *mut EXCEPINFO = null_mut();



        // 함수 매개 변수할당  변수가 0이면 for는 건너뜀.
        // let mut var_arg = vec![__tagVARIANT::default(); #var_num];//변수 숫자만큼 생성
        // let mut pvar_arg:*mut Vec<__tagVARIANT> = &mut var_arg; //포인터 변수 생성

        let mut var_arg = vec![VARIANT::default(); #var_num]; //변수 숫자만큼 생성
        // let mut pvar_arg: *mut Vec<VARIANT> = &mut var_arg; //변수 숫자만큼 생성
    };

    //전달된 변수와 타입 정보를 별도 벡터로 변환한다.
    let mut arg_v:Vec<syn::Ident> = Vec::new();//밸류 스트링
    let mut arg_t:Vec<syn::Ident> = Vec::new();//타입 스트링


    for (i, s) in arg_types.into_iter().enumerate() {
        if i%2 ==0 {
            arg_v.push(s);
        }
        else {
            arg_t.push(s);
        }
    };

    let mut t_ident =  format!("");

    let var_count = var_num.base10_parse().unwrap();
        // let concatenated = format_ident!("{} ")


    // DISPPARAM 변수 할당은 역순으로
    for (i , r) in (0..var_count).rev().enumerate() {
        // t_ident = match arg_t[i].to_string().as_str() {
        //
        //     "VT_BSTR" => format!( "{}  let mut vt_val = var_arg[{}].n1.n2_mut().n3.bstrVal_mut(); vt_val={} ;",t_ident, i, arg_v[i] ),
        //     "VT_BOOL" => format!( "{}  let mut vt_val =  var_arg[{}].n1.n2_mut().n3.boolVal_mut() ;vt_val= {} ;",t_ident, i, arg_v[i] ),
        //     "VT_I4" => format!( "{}  let mut vt_val = var_arg[{}].n1.n2_mut().n3.lVal_mut(); vt_val= {} ;",t_ident, i, arg_v[i] ),
        //     "VT_I2" => format!( "{}  let mut vt_val = var_arg[{}].n1.n2_mut().n3.iVal_mut(); vt_val= {};",t_ident, i, arg_v[i] ),
        //     _ => format!("{} ", t_ident),//void
        //
        // }



        // var_arg[0] = *(varianted.as_ptr());
        // println!("왜 반복을 안하지");
        t_ident = match arg_t[r].to_string().as_str() {

            "VT_BSTR" => format!( "{} let var_val = U16String::from_str({}); let varianted = VariantExt::<*mut u16>::into_variant(var_val).unwrap() ; var_arg[{}] = *(varianted.as_ptr()); ",t_ident, arg_v[r], i ),
            "VT_BOOL" => format!( "{}  let varianted = VariantExt::<BOOL>::into_variant({}).unwrap() ;var_arg[{}] = *(varianted.as_ptr());",t_ident,  arg_v[r],i ),
            "VT_I4" => format!( "{}  let varianted = VariantExt::<LONG>::into_variant({}).unwrap() ;var_arg[{}] = *(varianted.as_ptr());",t_ident, arg_v[r],i ),
            "VT_INT" => format!( "{}  let varianted = VariantExt::<INT>::into_variant({}).unwrap() ;var_arg[{}] = *(varianted.as_ptr());",t_ident, arg_v[r],i ),
            _ => format!("{} ", t_ident),//VT_VOID

        }


    };





    //전달될 변수의 갯수만큼 반복할 튜플 반복용 인덱스
    // let  i = (0usize..arg_t.len()).map(syn::Index::from);

    // let mid_token =  quote! {
    //     // // 변수 타입은 벡터에 저장
    //     // let at = vec![#(#arg_t),*];
    //
    //
    //     // //바리언트에 타입 할당
    //     // for x in (0..#var_num) {
    //     //
    //     //     var_arg[x].n1.n2_mut().vt = at[x] as u16;
    //     // }
    //
    //
    // };

    //바리언트 변수 할당 스트링을 토큰으로 파싱
    let var_token: proc_macro2::TokenStream = t_ident.parse().unwrap();

    let val_token = quote! { #var_token };


    let last_token = quote! {


        let mut bvar = Box::new(var_arg);

        //DISPPARAMS cArgs 는 변수 갯수, rgvarg 는 변수 포인터
        dp.cArgs = #var_num;
        // dp.rgvarg = pvar_arg as *mut VARIANT ; //바리언트 배열에 저장된 함수 매개 변수를 포인터로 저장.
        // dp.rgvarg =  pvar_arg;
        dp.rgvarg =bvar.as_mut_ptr();

        //autotype & 리턴값 대신 전달된 변수 포인터를 변경하는 경우, 키움에는 필요 없어서 미구현, 다운
        if (#w_flag != DISPATCH_METHOD ) {panic!("현재 DISPATCH_METHOD 관련만 구현된 상태입니다.");}

        //리턴 값을 저장할 바리언트 유니언
        let mut var_return = VARIANT::default();
        let p_varet: *mut VARIANT = &mut var_return;


        //invoke 로 호출
        let hr = #p_disinter.Invoke(
        // <*const p_kh_interface>::as_ref().Invoke(
            #disp_id,
            &IID_NULL,
            LOCALE_USER_DEFAULT,
            #w_flag,
            p_dp,
            p_varet,
            exception_info,
            null_mut()
        );

        // 예외처리 구현
        // if FAILED(hr)  { Err("invoke 실패") }

    };


    //함수 리턴 타임에 맞게 리턴 값을 받아온다. U16String::from_bstr(val_str).to_string_lossy()
    let ret_token = match return_type.to_string().as_str() {
        "VT_BSTR" => quote!{ let r_str = *(var_return.n1.n2_mut().n3.bstrVal()); U16String::from_bstr(r_str).to_string_lossy() },
        "VT_INT" => quote!{ *(var_return.n1.n2_mut().n3.intVal())  },
        "VT_I2" => quote!{ *(var_return.n1.n2_mut().n3.iVal())  },
        "VT_I4" => quote!{ *(var_return.n1.n2_mut().n3.lVal()) },
        "VT_BOOL" => quote!{ *(var_return.n1.n2_mut().n3.boolVal())  },
        "VT_VARIANT" => quote!{ *(var_return.n1.n2_mut().n3.pvarVal())  },
        _ => quote!{},
    };


    //함수 변수 숫자가 없으면 중간 함수 변수 할당 부분을 건너뛴다.

    let tokens = if var_num.base10_parse::<u32>().unwrap()  >0 {
        quote! { unsafe { #main_tokens  #val_token #last_token #ret_token } }
    } else {
        quote! { unsafe { #main_tokens  #last_token #ret_token } }
    };

    // println!("{}",tokens);// 매크로 출력물 확인용.
    tokens.into()

}

