use crate::program_header::SegmentType::PT_NULL;

type SegmentFlags = [u8; 4];
#[cfg(target_arch = "x86_64")]
type Offset = u64;
#[cfg(target_arch = "x86_64")]
type VirtualAddress = u64;
#[cfg(target_arch = "x86_64")]
type PhysicalAddress = u64;
#[cfg(target_arch = "x86_64")]
type FileSize = u64;
#[cfg(target_arch = "x86_64")]
type MemorySize = u64;
#[cfg(target_arch = "x86_64")]
type Alignment = u64;
#[cfg(target_arch = "x86")]
type Offset = u32;
#[cfg(target_arch = "x86")]
type VirtualAddress = u32;
#[cfg(target_arch = "x86")]
type PhysicalAddress = u32;
#[cfg(target_arch = "x86")]
type FileSize = u32;
#[cfg(target_arch = "x86")]
type FileSize = u32;
#[cfg(target_arch = "x86")]
type Alignment = u32;

pub struct ProgramHeader {
    segment_type: SegmentType,
    #[cfg(target_arch = "x86_64")]
    segment_flags: SegmentFlags,
    segment_offset: Offset,
    virtual_address: VirtualAddress,
    physical_address: PhysicalAddress,
    file_size: FileSize,
    memory_size: MemorySize,
    #[cfg(target_arch = "x86")]
    segment_flags: SegmentFlags,
    alignment: Alignment,
}

impl ProgramHeader {
    pub fn write_bytes(self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();

        v.extend((self.segment_type as u32).to_ne_bytes());
        #[cfg(target_arch = "x86_64")]
        v.extend(self.segment_flags);
        v.extend(self.segment_offset.to_ne_bytes());
        v.extend(self.virtual_address.to_ne_bytes());
        v.extend(self.physical_address.to_ne_bytes());
        v.extend(self.file_size.to_ne_bytes());
        v.extend(self.memory_size.to_ne_bytes());
        #[cfg(target_arch = "x86")]
        v.extend(self.segment_flags);
        v.extend(self.alignment.to_ne_bytes());

        v
    }
}

impl Default for ProgramHeader {
    fn default() -> Self {
        ProgramHeader {
            segment_type: PT_NULL,
            segment_flags: [SegmentFlag::Executable as u8, SegmentFlag::Readable as u8, 0x00, 0x00],
            segment_offset: 0,
            virtual_address: 0,
            physical_address: 0,
            file_size: 0,
            memory_size: 0,
            alignment: 0,
        }
    }
}


#[allow(non_camel_case_types)]
pub enum SegmentType {
    PT_NULL = 0x00000000,
    PT_LOAD = 0x00000001,
    PT_DYNAMIC = 0x00000002,
    PT_INTERP = 0x00000003,
    PT_NOTE = 0x00000004,
    PT_SHLIB = 0x00000005,
    PT_PHDR = 0x00000006,
    PT_TLS = 0x00000007,
    PT_LOOS = 0x60000000,
    PT_HIOS = 0x6FFFFFFF,
    PT_LOPROC = 0x70000000,
    PT_HIPROC = 0x7FFFFFFF,
}

pub enum SegmentFlag {
    Executable = 0x01,
    Writeable = 0x02,
    Readable = 0x04,
}
