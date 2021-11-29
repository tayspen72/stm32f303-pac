#[doc = "Register `CCMR3_Output` reader"]
pub struct R(crate::R<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR3_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR3_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR3_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR3_Output` writer"]
pub struct W(crate::W<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR3_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR3_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR3_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable"]
pub struct OC5FE_R(crate::FieldReader<bool, bool>);
impl OC5FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable"]
pub struct OC5FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5FE_W<'a> {
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
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable"]
pub struct OC5PE_R(crate::FieldReader<bool, bool>);
impl OC5PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable"]
pub struct OC5PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5PE_W<'a> {
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
#[doc = "Field `OC5M` reader - Output compare 5 mode"]
pub struct OC5M_R(crate::FieldReader<u8, u8>);
impl OC5M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC5M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5M` writer - Output compare 5 mode"]
pub struct OC5M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable"]
pub struct OC5CE_R(crate::FieldReader<bool, bool>);
impl OC5CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable"]
pub struct OC5CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable"]
pub struct OC6FE_R(crate::FieldReader<bool, bool>);
impl OC6FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable"]
pub struct OC6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6FE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable"]
pub struct OC6PE_R(crate::FieldReader<bool, bool>);
impl OC6PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable"]
pub struct OC6PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `OC6M` reader - Output compare 6 mode"]
pub struct OC6M_R(crate::FieldReader<u8, u8>);
impl OC6M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC6M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6M` writer - Output compare 6 mode"]
pub struct OC6M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable"]
pub struct OC6CE_R(crate::FieldReader<bool, bool>);
impl OC6CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable"]
pub struct OC6CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `OC5M_3` reader - Outout Compare 5 mode bit 3"]
pub struct OC5M_3_R(crate::FieldReader<bool, bool>);
impl OC5M_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5M_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5M_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5M_3` writer - Outout Compare 5 mode bit 3"]
pub struct OC5M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OC6M_3` reader - Outout Compare 6 mode bit 3"]
pub struct OC6M_3_R(crate::FieldReader<bool, bool>);
impl OC6M_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6M_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6M_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6M_3` writer - Outout Compare 6 mode bit 3"]
pub struct OC6M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Outout Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_3(&self) -> OC5M_3_R {
        OC5M_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Outout Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_3(&self) -> OC6M_3_R {
        OC6M_3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W {
        OC5FE_W { w: self }
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W {
        OC5PE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W {
        OC5M_W { w: self }
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W {
        OC5CE_W { w: self }
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W {
        OC6FE_W { w: self }
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W {
        OC6PE_W { w: self }
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W {
        OC6M_W { w: self }
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W {
        OC6CE_W { w: self }
    }
    #[doc = "Bit 16 - Outout Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_3(&mut self) -> OC5M_3_W {
        OC5M_3_W { w: self }
    }
    #[doc = "Bit 24 - Outout Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_3(&mut self) -> OC6M_3_W {
        OC6M_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 3 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr3_output](index.html) module"]
pub struct CCMR3_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR3_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr3_output::R](R) reader structure"]
impl crate::Readable for CCMR3_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr3_output::W](W) writer structure"]
impl crate::Writable for CCMR3_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR3_Output to value 0"]
impl crate::Resettable for CCMR3_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
