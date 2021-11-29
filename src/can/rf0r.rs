#[doc = "Register `RF0R` reader"]
pub struct R(crate::R<RF0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF0R` writer"]
pub struct W(crate::W<RF0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF0R_SPEC>;
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
impl From<crate::W<RF0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFOM0` reader - RFOM0"]
pub struct RFOM0_R(crate::FieldReader<bool, bool>);
impl RFOM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFOM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFOM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFOM0` writer - RFOM0"]
pub struct RFOM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM0_W<'a> {
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
#[doc = "Field `FOVR0` reader - FOVR0"]
pub struct FOVR0_R(crate::FieldReader<bool, bool>);
impl FOVR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOVR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVR0` writer - FOVR0"]
pub struct FOVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR0_W<'a> {
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
#[doc = "Field `FULL0` reader - FULL0"]
pub struct FULL0_R(crate::FieldReader<bool, bool>);
impl FULL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL0` writer - FULL0"]
pub struct FULL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL0_W<'a> {
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
#[doc = "Field `FMP0` reader - FMP0"]
pub struct FMP0_R(crate::FieldReader<u8, u8>);
impl FMP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&mut self) -> RFOM0_W {
        RFOM0_W { w: self }
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&mut self) -> FOVR0_W {
        FOVR0_W { w: self }
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&mut self) -> FULL0_W {
        FULL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf0r](index.html) module"]
pub struct RF0R_SPEC;
impl crate::RegisterSpec for RF0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf0r::R](R) reader structure"]
impl crate::Readable for RF0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf0r::W](W) writer structure"]
impl crate::Writable for RF0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF0R to value 0"]
impl crate::Resettable for RF0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
