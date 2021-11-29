#[doc = "Register `RF1R` reader"]
pub struct R(crate::R<RF1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF1R` writer"]
pub struct W(crate::W<RF1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RF1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFOM1` reader - RFOM1"]
pub struct RFOM1_R(crate::FieldReader<bool, bool>);
impl RFOM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFOM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFOM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFOM1` writer - RFOM1"]
pub struct RFOM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FOVR1` reader - FOVR1"]
pub struct FOVR1_R(crate::FieldReader<bool, bool>);
impl FOVR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOVR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVR1` writer - FOVR1"]
pub struct FOVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FULL1` reader - FULL1"]
pub struct FULL1_R(crate::FieldReader<bool, bool>);
impl FULL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL1` writer - FULL1"]
pub struct FULL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FMP1` reader - FMP1"]
pub struct FMP1_R(crate::FieldReader<u8, u8>);
impl FMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP1"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&mut self) -> RFOM1_W {
        RFOM1_W { w: self }
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&mut self) -> FOVR1_W {
        FOVR1_W { w: self }
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&mut self) -> FULL1_W {
        FULL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf1r](index.html) module"]
pub struct RF1R_SPEC;
impl crate::RegisterSpec for RF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf1r::R](R) reader structure"]
impl crate::Readable for RF1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf1r::W](W) writer structure"]
impl crate::Writable for RF1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF1R to value 0"]
impl crate::Resettable for RF1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
