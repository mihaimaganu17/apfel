use std::fmt;

pub use crate::utils;

pub static MH_MAGIC: u32 = 0xfeedfacf;      /* The mach magic number */


#[derive(Debug)]
pub struct MachHeader64 {
    magic: u32,         /* Mach magic number identified */
    cputype: i32,       /* Cpu specifier */
    cpusubtype: i32,    /* Machine specifier */
    filetype: u32,      /* Type of file */
    ncmds: u32,         /* Number of load commands */
    sizeofcmds: u32,    /* The size of all the load commands */
    flags: u32,         /* Flags */
}

impl fmt::Display for MachHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!("MACH_HEADER_64\n \
                                \tMagic: {:#02x}\n \
                                \tCpuType: {}\n \
                                \tCpuSubType: {}\n \
                                \tFileType: {}\n \
                                \tNumber of Commands: {}\n \
                                \tSize of Commands: {}\n \
                                \tFlags: {:#02x}\n",
            self.magic, self.cputype, self.cpusubtype, self.filetype,
            self.ncmds, self.sizeofcmds, self.flags);
        f.write_str(&string)
    }
}

impl MachHeader64 {
    /// Returns a MachHeader64 from a vector of [u8] bytes
    ///
    /// # Arguments
    ///
    /// * `bytes` - A vector of [u8] bytes
    ///
    pub fn parse(bytes: &[u8]) -> MachHeader64 {
        MachHeader64 {
            magic: utils::read_u32_from_offset(bytes, 0),
            cputype: utils::read_i32_from_offset(bytes, 4),
            cpusubtype: utils::read_i32_from_offset(bytes, 8),
            filetype: utils::read_u32_from_offset(bytes, 12),
            ncmds: utils::read_u32_from_offset(bytes, 16),
            sizeofcmds: utils::read_u32_from_offset(bytes, 20),
            flags: utils::read_u32_from_offset(bytes, 24),
        }
    }
}
