#[doc = "Register `RDT1R` reader"]
pub struct R(crate::R<RDT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDT1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME` reader - TIME"]
pub struct TIME_R(crate::FieldReader<u16, u16>);
impl TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMI` reader - FMI"]
pub struct FMI_R(crate::FieldReader<u8, u8>);
impl FMI_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` reader - DLC"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "receive FIFO mailbox data length control and time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdt1r](index.html) module"]
pub struct RDT1R_SPEC;
impl crate::RegisterSpec for RDT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdt1r::R](R) reader structure"]
impl crate::Readable for RDT1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDT1R to value 0"]
impl crate::Resettable for RDT1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
