use std::cmp::PartialEq;

const ELF_HEADER: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];
const EMPTY_BYTE_PADDING: [u8; 7] = [0x00; 7];
#[cfg(target_arch = "x86_64")]
const HEADER_SIZE: HeaderSize = 0x40;
#[cfg(target_arch = "x86_64")]
const PROGRAM_TABLE_SIZE: u16 = 0x38;
#[cfg(target_arch = "x86")]
const HEADER_SIZE: HeaderSize = 0x38;
#[cfg(target_arch = "x86")]
const PROGRAM_TABLE_SIZE: ProgramTableSize = 0x20;

type ElfMagic = [u8; 4];
type ABIVersion = u8;
type BytePadding = [u8; 7];
type ElfVersion = u32;
#[cfg(target_arch = "x86_64")]
type Entry = u64;
#[cfg(target_arch = "x86_64")]
type ProgramHeader = u64;
#[cfg(target_arch = "x86_64")]
type SectionHeader = u64;
#[cfg(target_arch = "x86")]
type Entry = u32;
#[cfg(target_arch = "x86")]
type ProgramHeader = u32;
#[cfg(target_arch = "x86")]
type SectionHeader = u32;
type Flags = u32;
type HeaderSize = u16;
type EntryLength = u16;

pub struct ElfHeader {
    magic_bytes: ElfMagic,
    execution_class: ExecutionClass,
    endianness: Endianness,
    identifier_elf_version: IdentifierElfVersion,
    os_abi: OsABI,
    abi_version: ABIVersion,
    byte_padding: BytePadding,
    object_type: ObjectType,
    instruction_set: InstructionSet,
    elf_version: ElfVersion,
    entry: Entry,
    program_table: ProgramHeader,
    section_table: SectionHeader,
    flags: Flags,
    header_size: HeaderSize,
    program_size: HeaderSize,
    program_entries: EntryLength,
    section_size: HeaderSize,
    section_entries: EntryLength,
    section_index: HeaderSize,
}

impl Default for ElfHeader {
    fn default() -> Self {
        ElfHeader {
            magic_bytes: ELF_HEADER, // 0x7F and ELF in HEX 0x45 0x4C 0x46
            execution_class: ExecutionClass::Long, // Long x64
            endianness: Endianness::Little, // Little endian?
            identifier_elf_version: IdentifierElfVersion::V1, // ELF Version 1
            os_abi: OsABI::Linux, // Linux ABI
            abi_version: 0x00, // Linux ignores this
            byte_padding: EMPTY_BYTE_PADDING, // Ignored and currently unused
            object_type: ObjectType::ET_EXEC, // Executable
            instruction_set: InstructionSet::AMDx86_64, // x64 Instruction Set
            elf_version: 0x01, // Default Version 1
            entry: 0x00, // Program entry pointer
            program_table: 0x00, // Program table pointer
            section_table: 0x00, // Section table pointer
            flags: 0x00, // Flags (Instruction Set Specific)
            header_size: HEADER_SIZE, // Size of header
            program_size: PROGRAM_TABLE_SIZE, // Program header size
            program_entries: 0x00, // Program header length
            section_size: 0x00, // Section header size
            section_entries: 0x00, // Section header length
            section_index: 0x00, // Section Name Index pointer
        }
    }
}

// Only contains methods that may need to be edited after creation
impl ElfHeader {
    pub fn set_entry(&mut self, p_entry: Entry) {
        self.entry = p_entry;
    }
    pub fn set_p_table(&mut self, p_table: ProgramHeader) {
        self.program_table = p_table;
    }
    pub fn set_flags(&mut self, f: Flags) {
        self.flags = f;
    }
    pub fn add_flags(&mut self, f: Flags) {
        self.flags |= f
    }
    pub fn clear_flags(&mut self) {
        self.flags &= 0x00;
    }
    pub fn set_p_size(&mut self, p_size: &ProgramHeader) {
        self.program_size = size_of_val(p_size) as HeaderSize;
    }
    pub fn set_p_len(&mut self, p_len: EntryLength) {
        self.program_entries = p_len;
    }
    pub fn set_s_size(&mut self, s_size: HeaderSize) {
        self.section_size = s_size;
    }
    pub fn set_s_len(&mut self, s_len: EntryLength) {
        self.section_entries = s_len;
    }
    pub fn set_s_idx(&mut self, s_idx: HeaderSize) {
        self.section_index = s_idx;
    }
    pub fn set_s_header_size(&mut self, s_header: &SectionHeader) {
        self.section_size = size_of_val(s_header) as HeaderSize;
    }

    pub fn write_bytes(self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();
        let endianness = self.endianness == Endianness::Little;

        v.extend(self.magic_bytes);
        v.push(self.execution_class as u8);
        v.push(self.endianness as u8);
        v.push(self.identifier_elf_version as u8);
        v.push(self.os_abi as u8);
        v.push(self.abi_version);
        v.extend(self.byte_padding);
        if endianness {
            v.extend((self.object_type as u16).to_le_bytes());
            v.extend((self.instruction_set as u16).to_le_bytes());
            v.extend(self.elf_version.to_le_bytes());
            v.extend((self.entry as Entry).to_le_bytes());
            v.extend((self.program_table as ProgramHeader).to_le_bytes());
            v.extend((self.section_table as SectionHeader).to_le_bytes());
            v.extend(self.flags.to_le_bytes());
            v.extend(self.header_size.to_le_bytes());
            v.extend(self.program_size.to_le_bytes());
            v.extend(self.program_entries.to_le_bytes());
            v.extend(self.section_size.to_le_bytes());
            v.extend(self.section_entries.to_le_bytes());
            v.extend(self.section_index.to_le_bytes());
        } else {
            v.extend((self.object_type as u16).to_be_bytes());
            v.extend((self.instruction_set as u16).to_be_bytes());
            v.extend(self.elf_version.to_be_bytes());
            v.extend((self.entry as Entry).to_be_bytes());
            v.extend((self.program_table as ProgramHeader).to_be_bytes());
            v.extend((self.section_table as SectionHeader).to_be_bytes());
            v.extend(self.flags.to_be_bytes());
            v.extend(self.header_size.to_be_bytes());
            v.extend(self.program_size.to_be_bytes());
            v.extend(self.program_entries.to_be_bytes());
            v.extend(self.section_size.to_be_bytes());
            v.extend(self.section_entries.to_be_bytes());
            v.extend(self.section_index.to_be_bytes());
        }

        v
    }
}

pub enum ExecutionClass {
    Real = 0x01,
    Long = 0x02,
}

#[derive(PartialEq)]
pub enum Endianness {
    Little = 0x01,
    Big = 0x02,
}

pub enum IdentifierElfVersion {
    V1 = 0x01
}

pub enum OsABI {
    SystemV = 0x00,
    HPUX = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNUHurd = 0x04,
    Solaris = 0x05,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0E,
    AROS = 0x0F,
    FenixOS = 0x10,
    NuxiCloudABI = 0x11,
    StratusTechnologiesOpenVOS = 0x12,
}

#[allow(non_camel_case_types)]
pub enum ObjectType {
    ET_NONE = 0x00,
    ET_REL = 0x01,
    ET_EXEC = 0x02,
    ET_DYN = 0x03,
    ET_CORE = 0x04,
    OS_SPEC = 0xFE00,
    PROC_SPEC = 0xFF00,
}

pub enum InstructionSet {
    None = 0x00,
    ATATWE32100 = 0x01,
    SPARC = 0x02,
    X86 = 0x03,
    Motorola68000M68k = 0x04,
    Motorola88000M88k = 0x05,
    IntelMCU = 0x06,
    Intel80860 = 0x07,
    MIPS = 0x08,
    IBMSystem370 = 0x09,
    MIPSRS3000LittleEndian = 0x0A,
    HewlettPackardPARISC = 0x0F,
    Intel80960 = 0x13,
    PowerPC = 0x14,
    PowerPC64bit = 0x15,
    S390X = 0x16,
    IBMSPUSPC = 0x17,
    NECV800 = 0x24,
    FujitsuFR20 = 0x25,
    TRWRH32 = 0x26,
    MotorolaRCE = 0x27,
    Arm = 0x28,
    DigitalAlpha = 0x29,
    SuperH = 0x2A,
    SPARCVersion9 = 0x2B,
    SiemensTriCoreEmbeddedProcessor = 0x2C,
    ArgonautRISCCore = 0x2D,
    HitachiH8300 = 0x2E,
    HitachiH8300H = 0x2F,
    HitachiH8S = 0x30,
    HitachiH8500 = 0x31,
    IA64 = 0x32,
    StanfordMIPSX = 0x33,
    MotorolaColdFire = 0x34,
    MotorolaM68HC12 = 0x35,
    FujitsuMMAMultimediaAccelerator = 0x36,
    SiemensPCP = 0x37,
    SonynCPUEmbeddedRISCProcessor = 0x38,
    DensoNDR1microProcessor = 0x39,
    MotorolaStarCoreProcessor = 0x3A,
    ToyotaME16processor = 0x3B,
    STMicroelectronicsST100processor = 0x3C,
    AdvancedLogicCorpTinyJEmbeddedProcessorFamily = 0x3D,
    AMDx86_64 = 0x3E,
    SonyDSPProcessor = 0x3F,
    DigitalEquipmentCorpPDP10 = 0x40,
    DigitalEquipmentCorpPDP11 = 0x41,
    SiemensFX66Microcontroller = 0x42,
    STMicroelectronicsST9816bitMicrocontroller = 0x43,
    STMicroelectronicsST78bitMicrocontroller = 0x44,
    MotorolaMC68HC16Microcontroller = 0x45,
    MotorolaMC68HC11Microcontroller = 0x46,
    MotorolaMC68HC08Microcontroller = 0x47,
    MotorolaMC68HC05Microcontroller = 0x48,
    SiliconGraphicsSVx = 0x49,
    STMicroElectronicsST198bitMicrocontroller = 0x4A,
    DigitalVAX = 0x4B,
    AxisCommunications32bitEmbeddedProcessor = 0x4C,
    InfineonTechnologies32bitEmbeddedProcessor = 0x4D,
    Element1464bitDSPProcessor = 0x4E,
    LSILogic16bitDSPProcessor = 0x4F,
    TMS320C6000Family = 0x8C,
    MCSTElbruse2k = 0xAF,
    Arm64bitsArmv8AArch64 = 0xB7,
    ZilogZ80 = 0xDC,
    RISCV = 0xF3,
    BerkeleyPacketFilter = 0xF7,
    WDC65C816 = 0x101,
    LoongArch = 0x102,
}