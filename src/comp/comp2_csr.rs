#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub struct COMP2EN_R(crate::FieldReader<bool, bool>);
impl COMP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
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
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub struct COMP2MODE_R(crate::FieldReader<u8, u8>);
impl COMP2MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub struct COMP2MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub struct COMP2INSEL_R(crate::FieldReader<u8, u8>);
impl COMP2INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub struct COMP2INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 non inverted input selection"]
pub struct COMP2INPSEL_R(crate::FieldReader<bool, bool>);
impl COMP2INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 non inverted input selection"]
pub struct COMP2INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INPSEL_W<'a> {
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
#[doc = "Field `COMP2INMSEL` reader - Comparator 1inverting input selection"]
pub struct COMP2INMSEL_R(crate::FieldReader<bool, bool>);
impl COMP2INMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INMSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INMSEL` writer - Comparator 1inverting input selection"]
pub struct COMP2INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INMSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `COMP2_OUT_SEL` reader - Comparator 2 output selection"]
pub struct COMP2_OUT_SEL_R(crate::FieldReader<u8, u8>);
impl COMP2_OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_OUT_SEL` writer - Comparator 2 output selection"]
pub struct COMP2_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub struct COMP2POL_R(crate::FieldReader<bool, bool>);
impl COMP2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub struct COMP2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POL_W<'a> {
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
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub struct COMP2HYST_R(crate::FieldReader<u8, u8>);
impl COMP2HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub struct COMP2HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub struct COMP2_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP2_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub struct COMP2_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub struct COMP2OUT_R(crate::FieldReader<bool, bool>);
impl COMP2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub struct COMP2LOCK_R(crate::FieldReader<bool, bool>);
impl COMP2LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub struct COMP2LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2_out_sel(&self) -> COMP2_OUT_SEL_R {
        COMP2_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&mut self) -> COMP2MODE_W {
        COMP2MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W {
        COMP2INSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W {
        COMP2INPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W {
        COMP2INMSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2_out_sel(&mut self) -> COMP2_OUT_SEL_W {
        COMP2_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W {
        COMP2POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W {
        COMP2HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W {
        COMP2_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W {
        COMP2LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
