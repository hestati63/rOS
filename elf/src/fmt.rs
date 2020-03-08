use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EIClass {
    Bit32 = 1,
    Bit64 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EIData {
    LEndian = 1,
    BEndian = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OSabi {
    SystemV = 0,
    HPUX = 1,
    NetBsd = 2,
    Linux = 3,
    GnuHurd = 4,
    Solaris = 6,
    AIX = 7,
    IRIX = 8,
    FreeBsd = 9,
    Tru64 = 10,
    NovellModesto = 11,
    OpenBSD = 12,
    OpenVMS = 13,
    NonStopKernel = 14,
    AROS = 15,
    FenixOS = 16,
    CloudABI = 17,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EType {
    NONE = 0x0,
    REL = 0x1,
    EXEC = 0x2,
    DYN = 0x3,
    CORE = 0x4,
    LOOS = 0xfe00,
    HIOS = 0xfeff,
    LOPROC = 0xff00,
    HIPROC = 0xffff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EMachine {
    SPARC = 0x2,
    X86 = 0x3,
    MIPS = 0x8,
    PowerPC = 0x14,
    S390 = 0x16,
    ARM = 0x28,
    SuperH = 0x2A,
    IA64 = 0x32,
    X8664 = 0x3E,
    AArch64 = 0xB7,
    RISCV = 0xF3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PType {
    NULL = 0x0,
    LOAD = 0x1,
    DYNAMIC = 0x2,
    INTERP = 0x3,
    NOTE = 0x4,
    SHLIB = 0x5,
    PHDR = 0x6,
    LOOS = 0x60000000,
    HIOS = 0x6FFFFFFF,
    LOPROC = 0x70000000,
    HIPROC = 0x7FFFFFFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SHType {
    NULL = 0x0,
    PROGBITS = 0x1,
    SYMTAB = 0x2,
    STRTAB = 0x3,
    RELA = 0x4,
    HASH = 0x5,
    DYNAMIC = 0x6,
    NOTE = 0x7,
    NOBITS = 0x8,
    REL = 0x9,
    SHLIB = 0xa,
    DYNSYM = 0xb,
    INITARRAY = 0xe,
    FINIARRAY = 0xf,
    PREINITARRAY = 0x10,
    GROUP = 0x11,
    SYMTABSHNDX = 0x12,
    NUM = 0x13,
    LOOS = 0x60000000,
}

bitflags! {
    pub struct SHFlags64: u64 {
        const WRITE            = 0x1;
        const ALLOC            = 0x2;
        const EXECINSTR        = 0x4;
        const MERGE            = 0x10;
        const STRINGS          = 0x20;
        const INFO_LINK        = 0x40;
        const LINK_ORDER       = 0x80;
        const OS_NONCONFORMING = 0x100;
        const GROUP            = 0x200;
        const TLS              = 0x400;
        const MASKOS           = 0x0ff00000;
        const MASKPROC         = 0xf0000000;
        const ORDERED          = 0x4000000;
        const EXCLUDE          = 0x8000000;
    }
}

// The 64 bit ELF Header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ELFHeader64 {
    // 0x7F followed by ELF(45 4c 46) in ASCII;
    // these four bytes constitute the magic number.
    pub ei_magic: u32,
    // This byte is set to either 1 or 2 to signify 32- or 64-bit format,
    // respectively.
    pub ei_class: u8,
    // This byte is set to either 1 or 2 to signify little or big endianness,
    // respectively. This affects interpretation of multi-byte fields starting
    // with offset 0x10.
    pub ei_data: u8,
    // Set to 1 for the original and current version of ELF.
    pub ei_version: u8,
    // Identifies the target operating system ABI.
    pub ei_osabi: u8,
    // Further specifies the ABI version. Its interpretation depends on the
    // target ABI. Linux kernel (after at least 2.6) has no definition of it.
    // In that case, offset and size of EI_PAD are 8.
    pub ei_abiversion: u8,
    // currently unused.
    pub ei_pad: [u8; 7],
    // Identifies object file type.
    pub e_type: u16,
    // Specifies target instruction set architecture.
    pub e_machine: u16,
    // Set to 1 for the original version of ELF.
    pub e_version: u32,
    // This is the memory address of the entry point from where the process
    // starts executing. This field is either 32 or 64 bits long depending
    // on the format defined earlier.
    pub e_entry: u64,
    // Points to the start of the program header table. It usually follows
    // the file header immediately, making the offset 0x34 or 0x40
    // for 32- and 64-bit ELF executables, respectively.
    pub e_phoff: u64,
    // Points to the start of the section header table.
    pub e_shoff: u64,
    // Interpretation of this field depends on the target architecture.
    pub e_flags: u32,
    // Contains the size of this header, normally 64 Bytes for 64-bit and
    // 52 Bytes for 32-bit format.
    pub e_ehsize: u16,
    // Contains the size of a program header table entry.
    pub e_phentsize: u16,
    // Contains the number of entries in the program header table.
    pub e_phnum: u16,
    // Contains the size of a section header table entry.
    pub e_shentsize: u16,
    // Contains the number of entries in the section header table.
    pub e_shnum: u16,
    // Contains index of the section header table entry that contains the section names.
    pub e_shstrndx: u16,
}

// The 64 bit program header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ProgHeader64 {
    // Identifies the type of the segment.
    pub p_type: u32,
    // Segment-dependent flags (position for 64-bit structure).
    pub p_flags: u32,
    // Offset of the segment in the file image.
    pub p_offset: u64,
    // Virtual address of the segment in memory.
    pub p_vaddr: u64,
    // On systems where physical address is relevant,
    // reserved for segment's physical address.
    pub p_paddr: u64,
    // Size in bytes of the segment in the file image. May be 0.
    pub p_filesz: u64,
    // Size in bytes of the segment in memory. May be 0.
    pub p_memsz: u64,
    // 0 and 1 specify no alignment. Otherwise should be a positive,
    // integral power of 2, with p_vaddr equating p_offset modulus p_align.
    pub p_align: u64,
}

// The 64 bit section header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SectHeader64 {
    // An offset to a string in the .shstrtab section that represents
    // the name of this section.
    pub sh_name: u32,
    // Identifies the type of this header.
    pub sh_type: u32,
    // Identifies the attributes of the section.
    pub sh_flags: u64,
    // Virtual address of the section in memory, for sections that are loaded.
    pub sh_addr: u64,
    // Offset of the section in the file image.
    pub sh_offset: u64,
    // Size in bytes of the section in the file image. May be 0.
    pub sh_size: u64,
    // Contains the section index of an associated section. This field is used
    // for several purposes, depending on the type of section.
    pub sh_link: u32,
    // Contains extra information about the section. This field is used for
    // several purposes, depending on the type of section.
    pub sh_info: u32,
    // Contains the required alignment of the section.
    // This field must be a power of two.
    pub sh_addralign: u64,
    // Contains the size, in bytes, of each entry, for sections that contain
    // fixed-size entries. Otherwise, this field contains zero.
    pub sh_entsize: u64,
}
