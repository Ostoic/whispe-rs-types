///
use winapi::shared::{
    minwindef::ULONG,
    ntdef::{BOOLEAN, HANDLE, LIST_ENTRY, PVOID, SHORT, UNICODE_STRING},
};
use winapi::STRUCT;

STRUCT! {
    #[allow(non_snake_case)]
    struct LDR_MODULE {
        InLoadOrderModuleList: LIST_ENTRY,
        InMemoryOrderModuleList: LIST_ENTRY,
        InInitializationOrderModuleList: LIST_ENTRY,
        BaseAddress: PVOID,
        EntryPoint: PVOID,
        SizeOfImage: ULONG,
        FullDllName: UNICODE_STRING,
        BaseDllName: UNICODE_STRING,
        Flags: ULONG,
        LoadCount: SHORT,
        TlsIndex: SHORT,
        HashTableEntry: LIST_ENTRY,
        TimeDateStamp: ULONG,
    }
}

#[allow(non_camel_case_types)]
pub type PLDR_MODULE = *mut LDR_MODULE;

STRUCT! {
    #[allow(non_snake_case)]
    struct PEB_LDR_DATA {
        Length: ULONG,
        Initialized: BOOLEAN,
        SsHandle: HANDLE,
        InLoadOrderModuleList: LIST_ENTRY,
        InMemoryOrderModuleList: LIST_ENTRY,
        InInitializationOrderModuleList: LIST_ENTRY,
        EntryInProgress: PVOID,
        ShutdownInProgress: BOOLEAN,
        ShutdownThreadId: HANDLE,
    }
}
