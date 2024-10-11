mod program_header;
mod elf_header;
mod section_header;

#[cfg(test)]
mod elf_tests {
    use crate::elf_header::ElfHeader;

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_size() {
        let header = ElfHeader::default();
        assert_eq!(header.write_bytes().len(), 64)
    }


    #[test]
    #[cfg(target_arch = "x86")]
    fn test_size() {
        let header = ElfHeader::default();
        assert_eq!(header.write_bytes().len(), 54)
    }
}
