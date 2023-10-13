pub use crate::clBindings::clCommon::*;
pub use crate::clBindings::clCommonErrors::*;
pub use crate::clBindings::clMD5Api::*;
pub use crate::clBindings::clHash::*;
/* automatically generated by rust-bindgen 0.66.1 */

pub const CL_DIFFERENCE_VECTOR_BLOCK_SHIFT: u32 = 12;
pub const CL_DIFFERENCE_VECTOR_BLOCK_SIZE: u32 = 4096;
pub const CL_DIFFERENCE_VECTOR_BLOCK_MASK: u32 = 4095;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClDifferenceVectorKey {
    pub groupKey: *mut ClStringT,
    pub sectionKey: *mut ClStringT,
}
#[test]
fn bindgen_test_layout_ClDifferenceVectorKey() {
    const UNINIT: ::std::mem::MaybeUninit<ClDifferenceVectorKey> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClDifferenceVectorKey>(),
        16usize,
        concat!("Size of: ", stringify!(ClDifferenceVectorKey))
    );
    assert_eq!(
        ::std::mem::align_of::<ClDifferenceVectorKey>(),
        8usize,
        concat!("Alignment of ", stringify!(ClDifferenceVectorKey))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).groupKey) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVectorKey),
            "::",
            stringify!(groupKey)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sectionKey) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVectorKey),
            "::",
            stringify!(sectionKey)
        )
    );
}
pub type ClDifferenceVectorKeyT = ClDifferenceVectorKey;
#[repr(C)]
pub struct ClDifferenceBlock {
    pub hash: hashStruct,
    pub key: ClDifferenceVectorKeyT,
    pub size: ClSizeT,
    pub data: *mut ClUint8T,
    pub md5Blocks: ClUint32T,
    pub md5List: *mut ClMD5T,
}
#[test]
fn bindgen_test_layout_ClDifferenceBlock() {
    const UNINIT: ::std::mem::MaybeUninit<ClDifferenceBlock> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClDifferenceBlock>(),
        64usize,
        concat!("Size of: ", stringify!(ClDifferenceBlock))
    );
    assert_eq!(
        ::std::mem::align_of::<ClDifferenceBlock>(),
        8usize,
        concat!("Alignment of ", stringify!(ClDifferenceBlock))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hash) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md5Blocks) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(md5Blocks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md5List) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceBlock),
            "::",
            stringify!(md5List)
        )
    );
}
pub type ClDifferenceBlockT = ClDifferenceBlock;
#[repr(C)]
pub struct ClDataVector {
    pub dataBlock: ClUint32T,
    pub dataSize: ClSizeT,
    pub dataBase: *mut ClUint8T,
}
#[test]
fn bindgen_test_layout_ClDataVector() {
    const UNINIT: ::std::mem::MaybeUninit<ClDataVector> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClDataVector>(),
        24usize,
        concat!("Size of: ", stringify!(ClDataVector))
    );
    assert_eq!(
        ::std::mem::align_of::<ClDataVector>(),
        8usize,
        concat!("Alignment of ", stringify!(ClDataVector))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dataBlock) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDataVector),
            "::",
            stringify!(dataBlock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dataSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDataVector),
            "::",
            stringify!(dataSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dataBase) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDataVector),
            "::",
            stringify!(dataBase)
        )
    );
}
pub type ClDataVectorT = ClDataVector;
#[repr(C)]
pub struct ClDifferenceVector {
    pub numDataVectors: ClUint32T,
    pub dataVectors: *mut ClDataVectorT,
    pub md5Blocks: ClUint32T,
    pub md5List: *mut ClMD5T,
}
#[test]
fn bindgen_test_layout_ClDifferenceVector() {
    const UNINIT: ::std::mem::MaybeUninit<ClDifferenceVector> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClDifferenceVector>(),
        32usize,
        concat!("Size of: ", stringify!(ClDifferenceVector))
    );
    assert_eq!(
        ::std::mem::align_of::<ClDifferenceVector>(),
        8usize,
        concat!("Alignment of ", stringify!(ClDifferenceVector))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numDataVectors) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVector),
            "::",
            stringify!(numDataVectors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dataVectors) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVector),
            "::",
            stringify!(dataVectors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md5Blocks) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVector),
            "::",
            stringify!(md5Blocks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md5List) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ClDifferenceVector),
            "::",
            stringify!(md5List)
        )
    );
}
pub type ClDifferenceVectorT = ClDifferenceVector;
extern "C" {
    pub fn clDifferenceVectorGet(
        key: *mut ClDifferenceVectorKeyT,
        data: *mut ClUint8T,
        offset: ClOffsetT,
        size: ClSizeT,
        copyData: ClBoolT,
        vector: *mut ClDifferenceVectorT,
    ) -> ClRcT;
}
extern "C" {
    pub fn clDifferenceVectorGetWithReset(
        key: *mut ClDifferenceVectorKeyT,
        data: *mut ClUint8T,
        offset: ClOffsetT,
        size: ClSizeT,
        copyData: ClBoolT,
        vector: *mut ClDifferenceVectorT,
    ) -> ClRcT;
}
extern "C" {
    pub fn clDifferenceVectorMergeWithData(
        lastData: *mut ClUint8T,
        lastDataSize: ClSizeT,
        vector: *mut ClDifferenceVectorT,
        offset: ClOffsetT,
        size: ClSizeT,
    ) -> *mut ClUint8T;
}
extern "C" {
    pub fn clDifferenceVectorMerge(
        key: *mut ClDifferenceVectorKeyT,
        vector: *mut ClDifferenceVectorT,
        offset: ClOffsetT,
        size: ClSizeT,
    ) -> *mut ClUint8T;
}
extern "C" {
    pub fn clDifferenceVectorMergeWithReset(
        key: *mut ClDifferenceVectorKeyT,
        vector: *mut ClDifferenceVectorT,
        offset: ClOffsetT,
        size: ClSizeT,
    ) -> *mut ClUint8T;
}
extern "C" {
    pub fn clDifferenceVectorCopy(dest: *mut ClDifferenceVectorT, src: *mut ClDifferenceVectorT);
}
extern "C" {
    pub fn clDifferenceVectorDelete(key: *mut ClDifferenceVectorKeyT) -> ClRcT;
}
extern "C" {
    pub fn clDifferenceVectorDestroy();
}
extern "C" {
    pub fn clDifferenceVectorKeyFree(key: *mut ClDifferenceVectorKeyT);
}
extern "C" {
    pub fn clDifferenceVectorFree(
        differenceVector: *mut ClDifferenceVectorT,
        freeDataVector: ClBoolT,
    );
}
extern "C" {
    pub fn clDifferenceVectorKeyMake(
        key: *mut ClDifferenceVectorKeyT,
        groupKey: *const ClNameT,
        sectionFmt: *const ClCharT,
        ...
    ) -> *mut ClDifferenceVectorKeyT;
}
extern "C" {
    pub fn clDifferenceVectorKeyCheck(key: *mut ClDifferenceVectorKeyT) -> ClBoolT;
}
extern "C" {
    pub fn clDifferenceVectorKeyCheckAndAdd(key: *mut ClDifferenceVectorKeyT) -> ClBoolT;
}