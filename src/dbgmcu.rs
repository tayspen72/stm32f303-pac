#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - APB Low Freeze Register"]
    pub apb1fz: crate::Reg<apb1fz::APB1FZ_SPEC>,
    #[doc = "0x0c - APB High Freeze Register"]
    pub apb2fz: crate::Reg<apb2fz::APB2FZ_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1FZ register accessor: an alias for `Reg<APB1FZ_SPEC>`"]
pub type APB1FZ = crate::Reg<apb1fz::APB1FZ_SPEC>;
#[doc = "APB Low Freeze Register"]
pub mod apb1fz;
#[doc = "APB2FZ register accessor: an alias for `Reg<APB2FZ_SPEC>`"]
pub type APB2FZ = crate::Reg<apb2fz::APB2FZ_SPEC>;
#[doc = "APB High Freeze Register"]
pub mod apb2fz;
