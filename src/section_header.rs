
type SectionNameOffset = u32;
#[cfg(target_arch = "x86_64")]
type SectionFlags = u64;
#[cfg(target_arch = "x86_64")]
type SectionAddress = u64;
#[cfg(target_arch = "x86_64")]
type SectionOffset = u64;
#[cfg(target_arch = "x86_64")]
type SectionSize = u64;
#[cfg(target_arch = "x86_64")]
type SectionAlignment = u64;
#[cfg(target_arch = "x86_64")]
type EntrySize = u64;

#[cfg(target_arch = "x86")]
type SectionFlags = u32;
#[cfg(target_arch = "x86")]
type SectionAddress = u32;
#[cfg(target_arch = "x86")]
type SectionOffset = u32;
#[cfg(target_arch = "x86")]
type SectionSize = u32;
#[cfg(target_arch = "x86")]
type SectionAlignment = u32;
#[cfg(target_arch = "x86")]
type EntrySize = u32;

type SectionLink = u32;
type SectionInfo = u32;

pub struct SectionHeader {
    section_name: SectionNameOffset,
    section_type: SectionType,
    section_flags: SectionFlags,
    section_address: SectionAddress,
    section_offset: SectionOffset,
    section_link: SectionLink,
    section_info: SectionInfo,
    section_alignment: SectionAlignment,
    section_size: EntrySize
}

impl Default for SectionHeader {
    fn default() -> Self {
        SectionHeader {
            section_name: 0x00,
            section_type: SectionType::SHT_NULL,
            section_flags: 0x00,
            section_address: 0x00,
            section_offset: 0x00,
            section_link: 0x00,
            section_info: 0x00,
            section_alignment: 0x00,
            section_size: 0x00,
        }
    }
}

impl SectionHeader {
    pub fn write_bytes(self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();

        v.extend(self.section_name.to_ne_bytes());
        v.extend((self.section_type as u32).to_ne_bytes());
        v.extend(self.section_flags.to_ne_bytes());
        v.extend(self.section_address.to_ne_bytes());
        v.extend(self.section_offset.to_ne_bytes());
        v.extend(self.section_link.to_ne_bytes());
        v.extend(self.section_info.to_ne_bytes());
        v.extend(self.section_alignment.to_ne_bytes());
        v.extend(self.section_size.to_ne_bytes());

        v
    }
}

#[allow(non_camel_case_types)]
pub enum SectionType {
    SHT_NULL = 0x0,
    SHT_PROGBITS = 0x1,
    SHT_SYMTAB = 0x2,
    SHT_STRTAB = 0x3,
    SHT_RELA = 0x4,
    SHT_HASH = 0x5,
    SHT_DYNAMIC = 0x6,
    SHT_NOTE = 0x7,
    SHT_NOBITS = 0x8,
    SHT_REL = 0x9,
    SHT_SHLIB = 0x0A,
    SHT_DYNSYM = 0x0B,
    SHT_INIT_ARRAY = 0x0E,
    SHT_FINI_ARRAY = 0x0F,
    SHT_PREINIT_ARRAY = 0x10,
    SHT_GROUP = 0x11,
    SHT_SYMTAB_SHNDX = 0x12,
    SHT_NUM = 0x13,
    SHT_LOOS = 0x60000000,
}