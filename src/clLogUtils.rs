use crate::clBindings::clLogApi::*;

pub fn clprintf (msg: String) {
    /*let mut formatted_msg = msg;
        if formatted_msg.ends_with('\n') {
            formatted_msg.pop();
        }*/
    let pArea: *const i8 = "---\0".as_ptr() as *const i8;
    let pContext: *const i8 = "---\0".as_ptr() as *const i8;
    let pFileName: *const i8 = std::ptr::null();
    let lineNum: u32 = 42;
    let pFmtStr: *const c_char = msg.as_bytes().as_ptr() as *const c_char;
    unsafe {clLogMsgWrite(CL_LOG_HANDLE_APP, CL_LOG_INFO, 10, pArea, pContext, pFileName, lineNum, pFmtStr);}
}