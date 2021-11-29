#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control register"]
    pub opamp1_cr: crate::Reg<opamp1_cr::OPAMP1_CR_SPEC>,
    #[doc = "0x04 - OPAMP2 control register"]
    pub opamp2_cr: crate::Reg<opamp2_cr::OPAMP2_CR_SPEC>,
    #[doc = "0x08 - OPAMP3 control register"]
    pub opamp3_cr: crate::Reg<opamp3_cr::OPAMP3_CR_SPEC>,
    #[doc = "0x0c - OPAMP4 control register"]
    pub opamp4_cr: crate::Reg<opamp4_cr::OPAMP4_CR_SPEC>,
}
#[doc = "OPAMP1_CR register accessor: an alias for `Reg<OPAMP1_CR_SPEC>`"]
pub type OPAMP1_CR = crate::Reg<opamp1_cr::OPAMP1_CR_SPEC>;
#[doc = "OPAMP1 control register"]
pub mod opamp1_cr;
#[doc = "OPAMP2_CR register accessor: an alias for `Reg<OPAMP2_CR_SPEC>`"]
pub type OPAMP2_CR = crate::Reg<opamp2_cr::OPAMP2_CR_SPEC>;
#[doc = "OPAMP2 control register"]
pub mod opamp2_cr;
#[doc = "OPAMP3_CR register accessor: an alias for `Reg<OPAMP3_CR_SPEC>`"]
pub type OPAMP3_CR = crate::Reg<opamp3_cr::OPAMP3_CR_SPEC>;
#[doc = "OPAMP3 control register"]
pub mod opamp3_cr;
#[doc = "OPAMP4_CR register accessor: an alias for `Reg<OPAMP4_CR_SPEC>`"]
pub type OPAMP4_CR = crate::Reg<opamp4_cr::OPAMP4_CR_SPEC>;
#[doc = "OPAMP4 control register"]
pub mod opamp4_cr;
