use super::inner_prelude::*;
use shared::ws2def::SCOPE_ID;
use shared::ws2def::SOCKADDR;
use shared::netioapi::IP_ADDRESS_PREFIX;

#[repr(C)]
pub enum FWPS_CALLOUT_NOTIFY_TYPE {
    FWPS_CALLOUT_NOTIFY_ADD_FILTER = 0,
    FWPS_CALLOUT_NOTIFY_DELETE_FILTER = 1,
    FWPS_CALLOUT_NOTIFY_ADD_FILTER_POST_COMMIT = 2,
    FWPS_CALLOUT_NOTIFY_TYPE_MAX = 3,
}

STRUCT! {
#[debug]
struct FWPS_INCOMING_VALUES0_ {
     layerId: UINT16,
     valueCount: UINT32,
     incomingValue: *mut FWPS_INCOMING_VALUE0,
}
}

STRUCT! {
struct FWPS_INCOMING_VALUE0_ {
     value: FWP_VALUE0,
}
}

pub type FWPS_INCOMING_VALUE0 = FWPS_INCOMING_VALUE0_;


pub type FWPS_INCOMING_VALUES0 = FWPS_INCOMING_VALUES0_;

FN! {stdcall FWPS_CALLOUT_CLASSIFY_FN1(
    inFixedValues: *const FWPS_INCOMING_VALUES0,
    inMetaValues:*const FWPS_INCOMING_METADATA_VALUES0,
    layerData: PVOID,
    classifyContext: PCVOID,
    filter: *const FWPS_FILTER1,
    flowContext: u64,
    classifyOut:*mut FWPS_CLASSIFY_OUT0,
) -> ()}

FN! {stdcall FWPS_CALLOUT_NOTIFY_FN1(
    notifyType: FWPS_CALLOUT_NOTIFY_TYPE,
    filterKey: PGUID,
    filter: *const FWPS_FILTER1,
) -> NTSTATUS}

FN! {stdcall FWPS_CALLOUT_FLOW_DELETE_NOTIFY_FN0(
    layerId: u16,
    calloutId: u32,
    flowContext: u64,
) -> NTSTATUS}

STRUCT! { struct FWPS_FILTER1 {
filterId:  UINT64                 ,
weight  :FWP_VALUE0,
subLayerWeight:  UINT16,
flags:  UINT16,
numFilterConditions:  UINT32,
filterCondition: MISS_TYPE_PTR,
action:  FWPS_ACTION0,
context:  UINT64,
providerContext:  MISS_TYPE_PTR,
}}
//TODO
STRUCT! { struct FWPS_CLASSIFY_OUT0 {
 actionType: FWP_ACTION_TYPE ,
 outContext: UINT64,
 filterId: UINT64,
 rights: UINT32,
 flags: UINT32,
 reserved: UINT32,
}}

ENUM! { enum FWP_ACTION_FLAG{
FWP_ACTION_FLAG_TERMINATING   =  (0x00001000),
FWP_ACTION_FLAG_NON_TERMINATING = (0x00002000),
FWP_ACTION_FLAG_CALLOUT         =(0x00004000),

}}
ENUM! { enum FWP_ACTION_TYPE{
FWP_ACTION_BLOCK  =  (0x00000001 | FWP_ACTION_FLAG_TERMINATING),
FWP_ACTION_PERMIT =  (0x00000002 | FWP_ACTION_FLAG_TERMINATING),
FWP_ACTION_CALLOUT_TERMINATING =  (0x00000003 | FWP_ACTION_FLAG_CALLOUT | FWP_ACTION_FLAG_TERMINATING),
FWP_ACTION_CALLOUT_INSPECTION =  (0x00000004 | FWP_ACTION_FLAG_CALLOUT | FWP_ACTION_FLAG_NON_TERMINATING),
FWP_ACTION_CALLOUT_UNKNOWN =  (0x00000005 | FWP_ACTION_FLAG_CALLOUT),
FWP_ACTION_CONTINUE =  (0x00000006 | FWP_ACTION_FLAG_NON_TERMINATING),
FWP_ACTION_NONE =  (0x00000007),
FWP_ACTION_NONE_NO_MATCH =  (0x00000008),
}}
STRUCT! {
struct FWPS_INCOMING_METADATA_VALUES0 {
    currentMetadataValues: UINT32,
    flags: UINT32,
    reserved: UINT64,
    discardMetadata: FWPS_DISCARD_METADATA0,
    flowHandle: UINT64,
    ipHeaderSize: UINT32,
    transportHeaderSize: UINT32,
    processPath: *const FWP_BYTE_BLOB,
    token: UINT64,
    processId: UINT64,
    sourceInterfaceIndex: UINT32,
    destinationInterfaceIndex: UINT32,
    compartmentId: ULONG,
    fragmentMetadata: FWPS_INBOUND_FRAGMENT_METADATA0,
    pathMtu: ULONG,
    completionHandle: HANDLE,
    transportEndpointHandle: UINT64,
    remoteScopeId: SCOPE_ID,
    controlData: MISS_TYPE_PTR,
    controlDataLength: ULONG,
    packetDirection: FWP_DIRECTION,
    headerIncludeHeader: PVOID,
    headerIncludeHeaderLength: ULONG,
    destinationPrefix: IP_ADDRESS_PREFIX,
    frameLength: UINT16,
    parentEndpointHandle: UINT64,
    icmpIdAndSequence: UINT32,
    localRedirectTargetPID: DWORD,
    originalDestination: *const SOCKADDR,
}
}


type PGUID = *const GUID;

STRUCT! {
struct FWPS_CALLOUT1 {
    calloutKey: GUID,
    flags: u32,
    classifyFn: FWPS_CALLOUT_CLASSIFY_FN1,
    notifyFn: FWPS_CALLOUT_NOTIFY_FN1,
    flowDeleteFn: FWPS_CALLOUT_FLOW_DELETE_NOTIFY_FN0,
}
}

pub type P_FWPS_CALLOUT1 = *const FWPS_CALLOUT1;






ENUM! {enum FWPS_BUILTIN_LAYERS {
    FWPS_LAYER_INBOUND_IPPACKET_V4, // 0
    FWPS_LAYER_INBOUND_IPPACKET_V4_DISCARD,
    FWPS_LAYER_INBOUND_IPPACKET_V6,
    FWPS_LAYER_INBOUND_IPPACKET_V6_DISCARD,
    FWPS_LAYER_OUTBOUND_IPPACKET_V4,
    FWPS_LAYER_OUTBOUND_IPPACKET_V4_DISCARD, // 5
    FWPS_LAYER_OUTBOUND_IPPACKET_V6,
    FWPS_LAYER_OUTBOUND_IPPACKET_V6_DISCARD,
    FWPS_LAYER_IPFORWARD_V4,
    FWPS_LAYER_IPFORWARD_V4_DISCARD,
    FWPS_LAYER_IPFORWARD_V6, // 10
    FWPS_LAYER_IPFORWARD_V6_DISCARD,
    FWPS_LAYER_INBOUND_TRANSPORT_V4,
    FWPS_LAYER_INBOUND_TRANSPORT_V4_DISCARD,
    FWPS_LAYER_INBOUND_TRANSPORT_V6,
    FWPS_LAYER_INBOUND_TRANSPORT_V6_DISCARD, // 15
    FWPS_LAYER_OUTBOUND_TRANSPORT_V4,
    FWPS_LAYER_OUTBOUND_TRANSPORT_V4_DISCARD,
    FWPS_LAYER_OUTBOUND_TRANSPORT_V6,
    FWPS_LAYER_OUTBOUND_TRANSPORT_V6_DISCARD,
    FWPS_LAYER_STREAM_V4, // 20
    FWPS_LAYER_STREAM_V4_DISCARD,
    FWPS_LAYER_STREAM_V6,
    FWPS_LAYER_STREAM_V6_DISCARD,
    FWPS_LAYER_DATAGRAM_DATA_V4,
    FWPS_LAYER_DATAGRAM_DATA_V4_DISCARD, // 25
    FWPS_LAYER_DATAGRAM_DATA_V6,
    FWPS_LAYER_DATAGRAM_DATA_V6_DISCARD,
    FWPS_LAYER_INBOUND_ICMP_ERROR_V4,
    FWPS_LAYER_INBOUND_ICMP_ERROR_V4_DISCARD,
    FWPS_LAYER_INBOUND_ICMP_ERROR_V6, // 30
    FWPS_LAYER_INBOUND_ICMP_ERROR_V6_DISCARD,
    FWPS_LAYER_OUTBOUND_ICMP_ERROR_V4,
    FWPS_LAYER_OUTBOUND_ICMP_ERROR_V4_DISCARD,
    FWPS_LAYER_OUTBOUND_ICMP_ERROR_V6,
    FWPS_LAYER_OUTBOUND_ICMP_ERROR_V6_DISCARD, // 35
    FWPS_LAYER_ALE_RESOURCE_ASSIGNMENT_V4,
    FWPS_LAYER_ALE_RESOURCE_ASSIGNMENT_V4_DISCARD,
    FWPS_LAYER_ALE_RESOURCE_ASSIGNMENT_V6,
    FWPS_LAYER_ALE_RESOURCE_ASSIGNMENT_V6_DISCARD,
    FWPS_LAYER_ALE_AUTH_LISTEN_V4, // 40
    FWPS_LAYER_ALE_AUTH_LISTEN_V4_DISCARD,
    FWPS_LAYER_ALE_AUTH_LISTEN_V6,
    FWPS_LAYER_ALE_AUTH_LISTEN_V6_DISCARD,
    FWPS_LAYER_ALE_AUTH_RECV_ACCEPT_V4,
    FWPS_LAYER_ALE_AUTH_RECV_ACCEPT_V4_DISCARD, // 45
    FWPS_LAYER_ALE_AUTH_RECV_ACCEPT_V6,
    FWPS_LAYER_ALE_AUTH_RECV_ACCEPT_V6_DISCARD,
    FWPS_LAYER_ALE_AUTH_CONNECT_V4,
    FWPS_LAYER_ALE_AUTH_CONNECT_V4_DISCARD,
    FWPS_LAYER_ALE_AUTH_CONNECT_V6, // 50
    FWPS_LAYER_ALE_AUTH_CONNECT_V6_DISCARD,
    FWPS_LAYER_ALE_FLOW_ESTABLISHED_V4,
    FWPS_LAYER_ALE_FLOW_ESTABLISHED_V4_DISCARD,
    FWPS_LAYER_ALE_FLOW_ESTABLISHED_V6,
    FWPS_LAYER_ALE_FLOW_ESTABLISHED_V6_DISCARD, // 55
}
}
#[link(name = "fwpkclnt")]
extern "system" {
    pub fn FwpsCalloutRegister1(
        deviceObject: PVOID,
        callout: P_FWPS_CALLOUT1,
        calloutId: *mut u32,
    ) -> NTSTATUS;
}
