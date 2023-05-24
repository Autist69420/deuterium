pub mod opcodes;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LuaHeader {
    pub header: [u8; 4], // four bytes
    pub version: u8, // one byte, Version number, 0x51 (81 decimal) for Lua 5.1
    pub format_version: u8, // one byte
    pub endianess_flag: u8, // one byte, default is 1,  0=big endian, 1=little endian
    pub int_size: u8, // one byte, default value is 4, Size of int (in bytes)
    pub size_t_size: u8, // one byte default value is 4, Size of size_t (in bytes)
    pub instruction_size: u8, // one byte, default value is 4, Size of Instruction (in bytes)
    pub lua_number_size: u8, // one byte, default value is 8, Size of lua_Number (in bytes)
    pub integral_flag: u8, // one byte default value 0, 0=floating-point, 1=integral number type
}