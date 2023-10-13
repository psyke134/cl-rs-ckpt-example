#![allow(unused_variables)]

use crate::clBindings::clOsalApi::*;
use crate::clBindings::clCommon::*;
use crate::clBindings::clIocApi::*;
use crate::clBindings::clEoApi::*;
use crate::clBindings::clEoConfigApi::*;

pub const COMP_NAME: &str = "SAFComponent0";
pub const COMP_EO_NAME: &str = "SAFComponent0_EO";

pub const COMP_EO_THREAD_PRIORITY: ClOsalThreadPriorityT = ClOsalThreadPriorityT_CL_OSAL_THREAD_PRI_MEDIUM;
pub const COMP_EO_NUM_THREAD: ClUint32T = 2;
pub const COMP_IOC_PORT: ClIocPortT = 0;
pub const COMP_EO_USER_CLIENT_ID: ClEoClientIdT = (ClEoClientIdT_CL_EO_CLOVIS_RESERVED_CLIENTID_END + 0);
pub const COMP_EO_USE_THREAD_MODEL: ClEoApplicationTypeT = ClEoApplicationTypeT_CL_EO_USE_THREAD_FOR_RECV;

const TRUE: ClUint8T = 1;
const FALSE: ClUint8T = 0;
                                                                                                                             
/* Component EO Basic Libraries */
pub const COMP_EO_BASICLIB_OSAL: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_TIMER: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_BUFFER: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_IOC: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_RMD: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_EO: ClUint8T = TRUE;
pub const COMP_EO_BASICLIB_OM: ClUint8T = FALSE;
pub const COMP_EO_BASICLIB_HAL: ClUint8T = FALSE;
pub const COMP_EO_BASICLIB_DBAL: ClUint8T = FALSE;
                                                                                                                             
/* Component EO Client Libraries */                                                                                                                            

pub const COMP_EO_CLIENTLIB_COR: ClUint8T = TRUE;
pub const COMP_EO_CLIENTLIB_CM : ClUint8T = FALSE;                  
pub const COMP_EO_CLIENTLIB_NAME : ClUint8T = TRUE;                  
pub const COMP_EO_CLIENTLIB_LOG : ClUint8T = TRUE;                  
pub const COMP_EO_CLIENTLIB_TRACE : ClUint8T = FALSE;                 
pub const COMP_EO_CLIENTLIB_DIAG : ClUint8T = FALSE;
pub const COMP_EO_CLIENTLIB_TXN : ClUint8T = TRUE;
pub const COMP_EO_CLIENTLIB_MSO : ClUint8T = FALSE;
pub const COMP_EO_CLIENTLIB_PROV : ClUint8T = FALSE;
pub const COMP_EO_CLIENTLIB_ALARM : ClUint8T = FALSE;
pub const COMP_EO_CLIENTLIB_DEBUG : ClUint8T = TRUE;
pub const COMP_EO_CLIENTLIB_GMS : ClUint8T = FALSE;
pub const COMP_EO_CLIENTLIB_PM : ClUint8T = FALSE;

/* Public global extern variables */

#[no_mangle]
pub static mut clEoConfig: ClEoConfigT = ClEoConfigT {
    EOname: [0; 32],
    pri: COMP_EO_THREAD_PRIORITY,
    noOfThreads: COMP_EO_NUM_THREAD,
    reqIocPort: COMP_IOC_PORT,
    maxNoClients: COMP_EO_USER_CLIENT_ID,
    appType: COMP_EO_USE_THREAD_MODEL,
    clEoCreateCallout: None,
    clEoDeleteCallout: None,
    clEoStateChgCallout: None,
    clEoHealthCheckCallout: None,
    clEoCustomAction: None,
    needSerialization: 0
};

#[no_mangle]
pub static mut clEoBasicLibs: [ClUint8T; 9] =
[
    COMP_EO_BASICLIB_OSAL,      /* Lib: Operating System Adaptation Layer   */
    COMP_EO_BASICLIB_TIMER,     /* Lib: Timer                               */
    COMP_EO_BASICLIB_BUFFER,    /* Lib: Buffer Management                   */
    COMP_EO_BASICLIB_IOC,       /* Lib: Intelligent Object Communication    */
    COMP_EO_BASICLIB_RMD,       /* Lib: Remote Method Dispatch              */
    COMP_EO_BASICLIB_EO,        /* Lib: Execution Object                    */
    COMP_EO_BASICLIB_OM,        /* Lib: Object Management                   */
    COMP_EO_BASICLIB_HAL,       /* Lib: Hardware Adaptation Layer           */
    COMP_EO_BASICLIB_DBAL,      /* Lib: Database Adaptation Layer           */
];

#[no_mangle]
pub static mut clEoClientLibs: [ClUint8T; 13] =
[
    COMP_EO_CLIENTLIB_COR,      /* Lib: Common Object Repository            */
    COMP_EO_CLIENTLIB_CM,       /* Lib: Chassis Management                  */
    COMP_EO_CLIENTLIB_NAME,     /* Lib: Name Service                        */
    COMP_EO_CLIENTLIB_LOG,      /* Lib: Log Service                         */
    COMP_EO_CLIENTLIB_TRACE,    /* Lib: Trace Service                       */
    COMP_EO_CLIENTLIB_DIAG,     /* Lib: Diagnostics                         */
    COMP_EO_CLIENTLIB_TXN,      /* Lib: Transaction Management              */
    COMP_EO_CLIENTLIB_MSO,      /* Lib: MSO Management                      */
    COMP_EO_CLIENTLIB_PROV,     /* Lib: Provisioning Management             */
    COMP_EO_CLIENTLIB_ALARM,    /* Lib: Alarm Management                    */
    COMP_EO_CLIENTLIB_DEBUG,    /* Lib: Debug Service                       */
    COMP_EO_CLIENTLIB_GMS,      /* Lib: Cluster/Group Membership Service    */
    COMP_EO_CLIENTLIB_PM        /* Lib: PM Management                       */
];

/* Utilities */

pub fn to32ByteArray(src: &str) -> [i8; 32] {
    let mut rs: [i8; 32] = [0; 32];
    let bytes = src.as_bytes();
    let len = bytes.len().min(31);
    
    rs[..len].copy_from_slice(&bytes[..len]
        .iter()
        .map(|&b| b as i8)
        .collect::<Vec<i8>>());
    return rs;
}
