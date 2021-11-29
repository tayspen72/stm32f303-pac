#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - master status register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x08 - transmit status register"]
    pub tsr: crate::Reg<tsr::TSR_SPEC>,
    #[doc = "0x0c - receive FIFO 0 register"]
    pub rf0r: crate::Reg<rf0r::RF0R_SPEC>,
    #[doc = "0x10 - receive FIFO 1 register"]
    pub rf1r: crate::Reg<rf1r::RF1R_SPEC>,
    #[doc = "0x14 - interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - error status register"]
    pub esr: crate::Reg<esr::ESR_SPEC>,
    #[doc = "0x1c - bit timing register"]
    pub btr: crate::Reg<btr::BTR_SPEC>,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - TX mailbox identifier register"]
    pub ti0r: crate::Reg<ti0r::TI0R_SPEC>,
    #[doc = "0x184 - mailbox data length control and time stamp register"]
    pub tdt0r: crate::Reg<tdt0r::TDT0R_SPEC>,
    #[doc = "0x188 - mailbox data low register"]
    pub tdl0r: crate::Reg<tdl0r::TDL0R_SPEC>,
    #[doc = "0x18c - mailbox data high register"]
    pub tdh0r: crate::Reg<tdh0r::TDH0R_SPEC>,
    #[doc = "0x190 - TX mailbox identifier register"]
    pub ti1r: crate::Reg<ti1r::TI1R_SPEC>,
    #[doc = "0x194 - mailbox data length control and time stamp register"]
    pub tdt1r: crate::Reg<tdt1r::TDT1R_SPEC>,
    #[doc = "0x198 - mailbox data low register"]
    pub tdl1r: crate::Reg<tdl1r::TDL1R_SPEC>,
    #[doc = "0x19c - mailbox data high register"]
    pub tdh1r: crate::Reg<tdh1r::TDH1R_SPEC>,
    #[doc = "0x1a0 - TX mailbox identifier register"]
    pub ti2r: crate::Reg<ti2r::TI2R_SPEC>,
    #[doc = "0x1a4 - mailbox data length control and time stamp register"]
    pub tdt2r: crate::Reg<tdt2r::TDT2R_SPEC>,
    #[doc = "0x1a8 - mailbox data low register"]
    pub tdl2r: crate::Reg<tdl2r::TDL2R_SPEC>,
    #[doc = "0x1ac - mailbox data high register"]
    pub tdh2r: crate::Reg<tdh2r::TDH2R_SPEC>,
    #[doc = "0x1b0 - receive FIFO mailbox identifier register"]
    pub ri0r: crate::Reg<ri0r::RI0R_SPEC>,
    #[doc = "0x1b4 - receive FIFO mailbox data length control and time stamp register"]
    pub rdt0r: crate::Reg<rdt0r::RDT0R_SPEC>,
    #[doc = "0x1b8 - receive FIFO mailbox data low register"]
    pub rdl0r: crate::Reg<rdl0r::RDL0R_SPEC>,
    #[doc = "0x1bc - receive FIFO mailbox data high register"]
    pub rdh0r: crate::Reg<rdh0r::RDH0R_SPEC>,
    #[doc = "0x1c0 - receive FIFO mailbox identifier register"]
    pub ri1r: crate::Reg<ri1r::RI1R_SPEC>,
    #[doc = "0x1c4 - receive FIFO mailbox data length control and time stamp register"]
    pub rdt1r: crate::Reg<rdt1r::RDT1R_SPEC>,
    #[doc = "0x1c8 - receive FIFO mailbox data low register"]
    pub rdl1r: crate::Reg<rdl1r::RDL1R_SPEC>,
    #[doc = "0x1cc - receive FIFO mailbox data high register"]
    pub rdh1r: crate::Reg<rdh1r::RDH1R_SPEC>,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - filter master register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: crate::Reg<fm1r::FM1R_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: crate::Reg<fs1r::FS1R_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: crate::Reg<ffa1r::FFA1R_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - CAN filter activation register"]
    pub fa1r: crate::Reg<fa1r::FA1R_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: crate::Reg<f0r1::F0R1_SPEC>,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: crate::Reg<f0r2::F0R2_SPEC>,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: crate::Reg<f1r1::F1R1_SPEC>,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: crate::Reg<f1r2::F1R2_SPEC>,
    _reserved37: [u8; 0xc8],
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: crate::Reg<f27r1::F27R1_SPEC>,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: crate::Reg<f27r2::F27R2_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "master control register"]
pub mod mcr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "master status register"]
pub mod msr;
#[doc = "TSR register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "RF0R register accessor: an alias for `Reg<RF0R_SPEC>`"]
pub type RF0R = crate::Reg<rf0r::RF0R_SPEC>;
#[doc = "receive FIFO 0 register"]
pub mod rf0r;
#[doc = "RF1R register accessor: an alias for `Reg<RF1R_SPEC>`"]
pub type RF1R = crate::Reg<rf1r::RF1R_SPEC>;
#[doc = "receive FIFO 1 register"]
pub mod rf1r;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ESR register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "error status register"]
pub mod esr;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "bit timing register"]
pub mod btr;
#[doc = "TI0R register accessor: an alias for `Reg<TI0R_SPEC>`"]
pub type TI0R = crate::Reg<ti0r::TI0R_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod ti0r;
#[doc = "TDT0R register accessor: an alias for `Reg<TDT0R_SPEC>`"]
pub type TDT0R = crate::Reg<tdt0r::TDT0R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt0r;
#[doc = "TDL0R register accessor: an alias for `Reg<TDL0R_SPEC>`"]
pub type TDL0R = crate::Reg<tdl0r::TDL0R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl0r;
#[doc = "TDH0R register accessor: an alias for `Reg<TDH0R_SPEC>`"]
pub type TDH0R = crate::Reg<tdh0r::TDH0R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh0r;
#[doc = "TI1R register accessor: an alias for `Reg<TI1R_SPEC>`"]
pub type TI1R = crate::Reg<ti1r::TI1R_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod ti1r;
#[doc = "TDT1R register accessor: an alias for `Reg<TDT1R_SPEC>`"]
pub type TDT1R = crate::Reg<tdt1r::TDT1R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt1r;
#[doc = "TDL1R register accessor: an alias for `Reg<TDL1R_SPEC>`"]
pub type TDL1R = crate::Reg<tdl1r::TDL1R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl1r;
#[doc = "TDH1R register accessor: an alias for `Reg<TDH1R_SPEC>`"]
pub type TDH1R = crate::Reg<tdh1r::TDH1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh1r;
#[doc = "TI2R register accessor: an alias for `Reg<TI2R_SPEC>`"]
pub type TI2R = crate::Reg<ti2r::TI2R_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod ti2r;
#[doc = "TDT2R register accessor: an alias for `Reg<TDT2R_SPEC>`"]
pub type TDT2R = crate::Reg<tdt2r::TDT2R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt2r;
#[doc = "TDL2R register accessor: an alias for `Reg<TDL2R_SPEC>`"]
pub type TDL2R = crate::Reg<tdl2r::TDL2R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl2r;
#[doc = "TDH2R register accessor: an alias for `Reg<TDH2R_SPEC>`"]
pub type TDH2R = crate::Reg<tdh2r::TDH2R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh2r;
#[doc = "RI0R register accessor: an alias for `Reg<RI0R_SPEC>`"]
pub type RI0R = crate::Reg<ri0r::RI0R_SPEC>;
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri0r;
#[doc = "RDT0R register accessor: an alias for `Reg<RDT0R_SPEC>`"]
pub type RDT0R = crate::Reg<rdt0r::RDT0R_SPEC>;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdt0r;
#[doc = "RDL0R register accessor: an alias for `Reg<RDL0R_SPEC>`"]
pub type RDL0R = crate::Reg<rdl0r::RDL0R_SPEC>;
#[doc = "receive FIFO mailbox data low register"]
pub mod rdl0r;
#[doc = "RDH0R register accessor: an alias for `Reg<RDH0R_SPEC>`"]
pub type RDH0R = crate::Reg<rdh0r::RDH0R_SPEC>;
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh0r;
#[doc = "RI1R register accessor: an alias for `Reg<RI1R_SPEC>`"]
pub type RI1R = crate::Reg<ri1r::RI1R_SPEC>;
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri1r;
#[doc = "RDT1R register accessor: an alias for `Reg<RDT1R_SPEC>`"]
pub type RDT1R = crate::Reg<rdt1r::RDT1R_SPEC>;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdt1r;
#[doc = "RDL1R register accessor: an alias for `Reg<RDL1R_SPEC>`"]
pub type RDL1R = crate::Reg<rdl1r::RDL1R_SPEC>;
#[doc = "receive FIFO mailbox data low register"]
pub mod rdl1r;
#[doc = "RDH1R register accessor: an alias for `Reg<RDH1R_SPEC>`"]
pub type RDH1R = crate::Reg<rdh1r::RDH1R_SPEC>;
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh1r;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "filter master register"]
pub mod fmr;
#[doc = "FM1R register accessor: an alias for `Reg<FM1R_SPEC>`"]
pub type FM1R = crate::Reg<fm1r::FM1R_SPEC>;
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "FS1R register accessor: an alias for `Reg<FS1R_SPEC>`"]
pub type FS1R = crate::Reg<fs1r::FS1R_SPEC>;
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "FFA1R register accessor: an alias for `Reg<FFA1R_SPEC>`"]
pub type FFA1R = crate::Reg<ffa1r::FFA1R_SPEC>;
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "FA1R register accessor: an alias for `Reg<FA1R_SPEC>`"]
pub type FA1R = crate::Reg<fa1r::FA1R_SPEC>;
#[doc = "CAN filter activation register"]
pub mod fa1r;
#[doc = "F0R1 register accessor: an alias for `Reg<F0R1_SPEC>`"]
pub type F0R1 = crate::Reg<f0r1::F0R1_SPEC>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "F0R2 register accessor: an alias for `Reg<F0R2_SPEC>`"]
pub type F0R2 = crate::Reg<f0r2::F0R2_SPEC>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "F1R1 register accessor: an alias for `Reg<F1R1_SPEC>`"]
pub type F1R1 = crate::Reg<f1r1::F1R1_SPEC>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "F1R2 register accessor: an alias for `Reg<F1R2_SPEC>`"]
pub type F1R2 = crate::Reg<f1r2::F1R2_SPEC>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "F27R1 register accessor: an alias for `Reg<F27R1_SPEC>`"]
pub type F27R1 = crate::Reg<f27r1::F27R1_SPEC>;
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "F27R2 register accessor: an alias for `Reg<F27R2_SPEC>`"]
pub type F27R2 = crate::Reg<f27r2::F27R2_SPEC>;
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
