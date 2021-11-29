#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPTERR` reader - Option byte error"]
pub struct OPTERR_R(crate::FieldReader<bool, bool>);
impl OPTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEVEL1_PROT` reader - Level 1 protection status"]
pub struct LEVEL1_PROT_R(crate::FieldReader<bool, bool>);
impl LEVEL1_PROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEVEL1_PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL1_PROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEVEL2_PROT` reader - Level 2 protection status"]
pub struct LEVEL2_PROT_R(crate::FieldReader<bool, bool>);
impl LEVEL2_PROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEVEL2_PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL2_PROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub struct WDG_SW_R(crate::FieldReader<bool, bool>);
impl WDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub struct NRST_STOP_R(crate::FieldReader<bool, bool>);
impl NRST_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub struct NRST_STDBY_R(crate::FieldReader<bool, bool>);
impl NRST_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT1` reader - BOOT1"]
pub struct BOOT1_R(crate::FieldReader<bool, bool>);
impl BOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub struct VDDA_MONITOR_R(crate::FieldReader<bool, bool>);
impl VDDA_MONITOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDA_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDA_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK"]
pub struct SRAM_PARITY_CHECK_R(crate::FieldReader<bool, bool>);
impl SRAM_PARITY_CHECK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PARITY_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PARITY_CHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data0` reader - Data0"]
pub struct DATA0_R(crate::FieldReader<u8, u8>);
impl DATA0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data1` reader - Data1"]
pub struct DATA1_R(crate::FieldReader<u8, u8>);
impl DATA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level 1 protection status"]
    #[inline(always)]
    pub fn level1_prot(&self) -> LEVEL1_PROT_R {
        LEVEL1_PROT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level 2 protection status"]
    #[inline(always)]
    pub fn level2_prot(&self) -> LEVEL2_PROT_R {
        LEVEL2_PROT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SRAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SRAM_PARITY_CHECK_R {
        SRAM_PARITY_CHECK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0xffff_ff02"]
impl crate::Resettable for OBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff02
    }
}
