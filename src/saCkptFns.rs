/*
 *  This is based on csa103Comp/checkpointFns.c
 */

use std::os::raw::c_void;
use std::mem::size_of;

use crate::clBindings::clAmsTypes::ClUint32T;
use crate::clBindings::saAis::*;
use crate::clBindings::saCkpt::*;
use crate::clLogUtils::*;

pub const CKPT_NAME: &str = "csa103Ckpt";
static mut ckptLibraryHandle: SaCkptHandleT = 0;
static mut ckptHandle: SaCkptCheckpointHandleT = 0;
pub const CKPT_SID_NAME: &str = "csa103 counter";
static mut sidBuff: Vec<u8> = Vec::new();

unsafe impl Sync for SaCkptSectionIdT {}
static mut ckptSid: SaCkptSectionIdT = SaCkptSectionIdT {
    idLen: 0,
    id: 0x0 as *mut SaUint8T    // NULL
};

static mut syncCount: SaInvocationT = 0;

unsafe extern "C" fn checkpointOpenCompleted(invocation: SaInvocationT, checkpointHandle: SaCkptCheckpointHandleT, error: SaAisErrorT) {
    clprintf(String::from("Checkpoint open completed"));
}

unsafe extern "C" fn checkpointSynchronizeCompleted(invocation: SaInvocationT, error: SaAisErrorT) {
    if invocation % 10 == 0 {
        clprintf(format!("Checkpoint synchronize [{}] completed", invocation));
    }
}

fn sectionIdInitialize() {
    unsafe {
        sidBuff = CKPT_SID_NAME.bytes().collect();
        sidBuff.push(0);

        let ptr: *mut u8 = sidBuff.as_mut_ptr();

        ckptSid.idLen = CKPT_SID_NAME.len() as SaUint16T;
        ckptSid.id = ptr as *mut SaUint8T;
    }
}

pub fn checkpointInitialize() -> SaAisErrorT {
    sectionIdInitialize();

    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;
    let mut ckptVersion: SaVersionT = SaVersionT {
        releaseCode: 'B' as u8,
        majorVersion: 1,
        minorVersion: 1
    };

    let nameSlice = CKPT_NAME.as_bytes();
    let mut buff: [u8; 256] = [0; 256];
    buff[..nameSlice.len()].copy_from_slice(nameSlice);
    let ckptName: SaNameT = SaNameT {
        length: CKPT_NAME.len() as SaUint16T,
        value: buff
    };

    let callbacks: SaCkptCallbacksT = SaCkptCallbacksT {
        saCkptCheckpointOpenCallback: Some(checkpointOpenCompleted),
        saCkptCheckpointSynchronizeCallback: Some(checkpointSynchronizeCompleted)
    };
    let attrs: SaCkptCheckpointCreationAttributesT = SaCkptCheckpointCreationAttributesT{
        creationFlags: SA_CKPT_WR_ACTIVE_REPLICA_WEAK | SA_CKPT_CHECKPOINT_COLLOCATED,
        checkpointSize: size_of::<ClUint32T>() as SaSizeT,
        retentionDuration: 10,
        maxSections: 2,
        maxSectionSize: size_of::<ClUint32T>() as SaSizeT,
        maxSectionIdSize: 64
    };

    clprintf(String::from("Checkpoint Initialize"));

    unsafe {
        rc = saCkptInitialize(&mut ckptLibraryHandle, &callbacks, &mut ckptVersion);

        if rc != SaAisErrorT_SA_AIS_OK {
            clprintf(format!("Failed to initialize checkpoint service with rc [0x{:X}]", rc));
            return rc;
        }
        clprintf(format!("Checkpoint service initialized (handle=0x{:X})", unsafe { ckptLibraryHandle }));

        rc = saCkptCheckpointOpen(
            ckptLibraryHandle,
            &ckptName,
            &attrs,
            SA_CKPT_CHECKPOINT_READ | SA_CKPT_CHECKPOINT_WRITE | SA_CKPT_CHECKPOINT_CREATE,
            SA_TIME_MAX as SaTimeT,
            &mut ckptHandle
        );
    }

    if rc != SaAisErrorT_SA_AIS_OK
    {
        clprintf(format!("Failed [0x{:X}] to open checkpoint", rc));
        return rc;
    }
    clprintf(format!("Checkpoint opened (handle=0x{:X})", unsafe { ckptHandle }));
    return rc;
}

pub fn checkpointFinalize() {
    let mut rc: SaAisErrorT =  SaAisErrorT_SA_AIS_OK;

    unsafe {
        rc = saCkptCheckpointClose(ckptHandle);
        if rc != SaAisErrorT_SA_AIS_OK {
            clprintf(format!("Failed [0x{:X}] to close checkpoint handle 0x{:X}", rc, ckptHandle));
        }
        rc = saCkptFinalize(ckptLibraryHandle);
        if rc != SaAisErrorT_SA_AIS_OK {
            clprintf(format!("Failed [0x{:X}] to finalize checkpoint", rc));
        }
    }
}

pub fn checkpointWriteSeq(seq: &ClUint32T) -> SaAisErrorT {
    let mut rc: SaAisErrorT =  SaAisErrorT_SA_AIS_OK;
    let seqNo: ClUint32T = seq.to_be();    // seq_no = htonl(seq);

    unsafe {
        rc = saCkptSectionOverwrite(
            ckptHandle,
            &ckptSid,
            &seqNo as *const u32 as *const c_void,
            size_of::<ClUint32T>() as SaSizeT
        );
        if rc != SaAisErrorT_SA_AIS_OK {
            if rc == 0x1000a || rc == SaAisErrorT_SA_AIS_ERR_NOT_EXIST  {
                let mut sectionCrAttr: SaCkptSectionCreationAttributesT = SaCkptSectionCreationAttributesT {
                    expirationTime: SA_TIME_END as SaTimeT,
                    sectionId: &mut ckptSid
                };
                rc = saCkptSectionCreate(
                    ckptHandle,
                    &mut sectionCrAttr,
                    &seqNo as *const ClUint32T as *const SaUint8T,
                    size_of::<ClUint32T>() as SaSizeT
                ); 
            }
            if rc != SaAisErrorT_SA_AIS_OK {
                clprintf(format!("Failed [0x{:X}] to write to section", rc));
            }
        }
        if rc != SaAisErrorT_SA_AIS_OK {
            rc = saCkptCheckpointSynchronizeAsync(ckptHandle,syncCount);
            syncCount += 1;
        }
    }

    return rc;
}

pub fn checkpointReadSeq(seq: &mut ClUint32T) -> SaAisErrorT {
    let mut rc: SaAisErrorT =  SaAisErrorT_SA_AIS_OK;
    let mut errIdx: ClUint32T = 0;
    let mut seqNo: ClUint32T = 0;
    unsafe {
        let mut iov: SaCkptIOVectorElementT = SaCkptIOVectorElementT {
            sectionId: SaCkptSectionIdT { idLen: ckptSid.idLen, id: ckptSid.id },
            dataBuffer: &mut seqNo as *mut ClUint32T as *mut c_void,
            dataSize: size_of::<ClUint32T>() as SaSizeT,
            dataOffset: 0,
            readSize: size_of::<ClUint32T>() as SaSizeT
        };

        rc = saCkptCheckpointRead(ckptHandle, &mut iov, 1, &mut errIdx);
    }
    if rc != SaAisErrorT_SA_AIS_OK {
        if rc != SaAisErrorT_SA_AIS_ERR_NOT_EXIST {
            return SaAisErrorT_SA_AIS_OK;
        }
        clprintf(format!("Error: [0x{:X}] from checkpoint read, err_idx = {}", rc, errIdx));
    }

    *seq = u32::from_be(seqNo);  // *seq = ntohl(seq_no);

    return SaAisErrorT_SA_AIS_OK;
}

pub fn checkpointReplicaActivate() -> SaAisErrorT {
    let mut rc: SaAisErrorT =  SaAisErrorT_SA_AIS_OK;

    unsafe {
        rc = saCkptActiveReplicaSet(ckptHandle);
        if rc != SaAisErrorT_SA_AIS_OK {
            clprintf(format!("checkpoint_replica_activate failed [0x{:X}] in ActiveReplicaSet", rc));
        }
        else {
            rc = SaAisErrorT_SA_AIS_OK;
        }
    }

    return rc;
}
