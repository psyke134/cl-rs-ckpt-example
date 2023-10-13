/* automatically generated by rust-bindgen 0.66.1 */

#[doc = " ARP protocol."]
pub const ClIocProtocols_CL_IOC_PROTO_ARP: ClIocProtocols = 1;
#[doc = " Flow control Protocol."]
pub const ClIocProtocols_CL_IOC_PROTO_FLOWCONTROL: ClIocProtocols = 2;
#[doc = " IOC Heartbeat protocol."]
pub const ClIocProtocols_CL_IOC_PROTO_HB: ClIocProtocols = 3;
#[doc = " IOC Control Protocol (for discovery and capability negotiation)."]
pub const ClIocProtocols_CL_IOC_PROTO_CTL: ClIocProtocols = 4;
#[doc = " Transparency Layer protocol."]
pub const ClIocProtocols_CL_IOC_PROTO_TL: ClIocProtocols = 5;
#[doc = "  Messaging service protocol."]
pub const ClIocProtocols_CL_IOC_PROTO_MSG: ClIocProtocols = 6;
#[doc = " Group communication related message."]
pub const ClIocProtocols_CL_IOC_PROTO_ICMP: ClIocProtocols = 7;
#[doc = " IOC internal reserved protocols end."]
pub const ClIocProtocols_CL_IOC_INTERNAL_PROTO_END: ClIocProtocols = 15;
#[doc = " RMD synchronous request."]
pub const ClIocProtocols_CL_IOC_RMD_SYNC_REQUEST_PROTO: ClIocProtocols = 16;
#[doc = " RMD synchronous reply."]
pub const ClIocProtocols_CL_IOC_RMD_SYNC_REPLY_PROTO: ClIocProtocols = 17;
#[doc = " RMD asynchronous request."]
pub const ClIocProtocols_CL_IOC_RMD_ASYNC_REQUEST_PROTO: ClIocProtocols = 18;
#[doc = " RMD asynchronous reply."]
pub const ClIocProtocols_CL_IOC_RMD_ASYNC_REPLY_PROTO: ClIocProtocols = 19;
#[doc = " Port close notification for CPM."]
pub const ClIocProtocols_CL_IOC_PORT_NOTIFICATION_PROTO: ClIocProtocols = 20;
#[doc = " Sys log protocol."]
pub const ClIocProtocols_CL_IOC_SYSLOG_PROTO: ClIocProtocols = 21;
#[doc = " Sys log protocol."]
pub const ClIocProtocols_CL_IOC_RMD_ACK_PROTO: ClIocProtocols = 22;
#[doc = " Sys log protocol."]
pub const ClIocProtocols_CL_IOC_RMD_ORDERED_PROTO: ClIocProtocols = 23;
#[doc = "  SAF Messaging Protocol"]
pub const ClIocProtocols_CL_IOC_SAF_MSG_REQUEST_PROTO: ClIocProtocols = 24;
#[doc = "  SAF Messaging Protocol"]
pub const ClIocProtocols_CL_IOC_SAF_MSG_REPLY_PROTO: ClIocProtocols = 25;
#[doc = "  SAF Messaging Protocol"]
pub const ClIocProtocols_CL_IOC_CONFIG_CHANGE_PROTO: ClIocProtocols = 32;
#[doc = " Here the reserved protocols for ASP end."]
pub const ClIocProtocols_CL_IOC_ASP_RESERVERD_PROTO_END: ClIocProtocols = 127;
#[doc = " If the application wants to specify its own protocols it can start from here, For example \\c CL_IOC_USER_PROTO_START+1."]
pub const ClIocProtocols_CL_IOC_USER_PROTO_START: ClIocProtocols = 128;
#[doc = " The application should specify its protocol number less than this value, if they plan to use some."]
pub const ClIocProtocols_CL_IOC_PROTO_END: ClIocProtocols = 254;
#[doc = " Last protocol Id for EO."]
pub const ClIocProtocols_CL_IOC_INVALID_PROTO: ClIocProtocols = 255;
#[doc = " Last protocol Id for EO."]
pub const ClIocProtocols_CL_IOC_NUM_PROTOS: ClIocProtocols = 256;
#[doc = " Reserved Protocol Type"]
pub type ClIocProtocols = ::std::os::raw::c_uint;
pub const ClConfigChange_CL_CONFIG_TIME_ZONE: ClConfigChange = 1;
pub type ClConfigChange = ::std::os::raw::c_uint;
