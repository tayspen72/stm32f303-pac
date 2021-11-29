#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub struct LOCUP_LOCK_R(crate::FieldReader<bool, bool>);
impl LOCUP_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCUP_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCUP_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub struct LOCUP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCUP_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_R(crate::FieldReader<bool, bool>);
impl SRAM_PARITY_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PARITY_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PARITY_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PARITY_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub struct PVD_LOCK_R(crate::FieldReader<bool, bool>);
impl PVD_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVD_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVD_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub struct PVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BYP_ADD_PAR` reader - Bypass address bit 29 in parity calculation"]
pub struct BYP_ADD_PAR_R(crate::FieldReader<bool, bool>);
impl BYP_ADD_PAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYP_ADD_PAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYP_ADD_PAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYP_ADD_PAR` writer - Bypass address bit 29 in parity calculation"]
pub struct BYP_ADD_PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BYP_ADD_PAR_W<'a> {
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
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub struct SRAM_PEF_R(crate::FieldReader<bool, bool>);
impl SRAM_PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub struct SRAM_PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn locup_lock(&self) -> LOCUP_LOCK_R {
        LOCUP_LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_add_par(&self) -> BYP_ADD_PAR_R {
        BYP_ADD_PAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn locup_lock(&mut self) -> LOCUP_LOCK_W {
        LOCUP_LOCK_W { w: self }
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W {
        SRAM_PARITY_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W {
        PVD_LOCK_W { w: self }
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_add_par(&mut self) -> BYP_ADD_PAR_W {
        BYP_ADD_PAR_W { w: self }
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W {
        SRAM_PEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
