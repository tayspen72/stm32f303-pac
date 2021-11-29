#[doc = "Register `COMP4_CSR` reader"]
pub struct R(crate::R<COMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP4_CSR` writer"]
pub struct W(crate::W<COMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP4_CSR_SPEC>;
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
impl From<crate::W<COMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub struct COMP4EN_R(crate::FieldReader<bool, bool>);
impl COMP4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub struct COMP4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4EN_W<'a> {
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
#[doc = "Field `COMP4MODE` reader - Comparator 4 mode"]
pub struct COMP4MODE_R(crate::FieldReader<u8, u8>);
impl COMP4MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4MODE` writer - Comparator 4 mode"]
pub struct COMP4MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP4INSEL` reader - Comparator 4 inverting input selection"]
pub struct COMP4INSEL_R(crate::FieldReader<u8, u8>);
impl COMP4INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4INSEL` writer - Comparator 4 inverting input selection"]
pub struct COMP4INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP4INPSEL` reader - Comparator 4 non inverted input selection"]
pub struct COMP4INPSEL_R(crate::FieldReader<bool, bool>);
impl COMP4INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4INPSEL` writer - Comparator 4 non inverted input selection"]
pub struct COMP4INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4INPSEL_W<'a> {
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
#[doc = "Field `COM4WINMODE` reader - Comparator 4 window mode"]
pub struct COM4WINMODE_R(crate::FieldReader<bool, bool>);
impl COM4WINMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM4WINMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COM4WINMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COM4WINMODE` writer - Comparator 4 window mode"]
pub struct COM4WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COM4WINMODE_W<'a> {
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
#[doc = "Field `COMP4_OUT_SEL` reader - Comparator 4 output selection"]
pub struct COMP4_OUT_SEL_R(crate::FieldReader<u8, u8>);
impl COMP4_OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4_OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4_OUT_SEL` writer - Comparator 4 output selection"]
pub struct COMP4_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub struct COMP4POL_R(crate::FieldReader<bool, bool>);
impl COMP4POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub struct COMP4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4POL_W<'a> {
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
#[doc = "Field `COMP4HYST` reader - Comparator 4 hysteresis"]
pub struct COMP4HYST_R(crate::FieldReader<u8, u8>);
impl COMP4HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4HYST` writer - Comparator 4 hysteresis"]
pub struct COMP4HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub struct COMP4_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP4_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub struct COMP4_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub struct COMP4OUT_R(crate::FieldReader<bool, bool>);
impl COMP4OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub struct COMP4LOCK_R(crate::FieldReader<bool, bool>);
impl COMP4LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP4LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub struct COMP4LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> COMP4EN_R {
        COMP4EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> COMP4MODE_R {
        COMP4MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4insel(&self) -> COMP4INSEL_R {
        COMP4INSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> COMP4INPSEL_R {
        COMP4INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn com4winmode(&self) -> COM4WINMODE_R {
        COM4WINMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4_out_sel(&self) -> COMP4_OUT_SEL_R {
        COMP4_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> COMP4POL_R {
        COMP4POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> COMP4HYST_R {
        COMP4HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> COMP4_BLANKING_R {
        COMP4_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> COMP4OUT_R {
        COMP4OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> COMP4LOCK_R {
        COMP4LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&mut self) -> COMP4EN_W {
        COMP4EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&mut self) -> COMP4MODE_W {
        COMP4MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4insel(&mut self) -> COMP4INSEL_W {
        COMP4INSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    pub fn comp4inpsel(&mut self) -> COMP4INPSEL_W {
        COMP4INPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn com4winmode(&mut self) -> COM4WINMODE_W {
        COM4WINMODE_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4_out_sel(&mut self) -> COMP4_OUT_SEL_W {
        COMP4_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&mut self) -> COMP4POL_W {
        COMP4POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&mut self) -> COMP4HYST_W {
        COMP4HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&mut self) -> COMP4_BLANKING_W {
        COMP4_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&mut self) -> COMP4LOCK_W {
        COMP4LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4_csr](index.html) module"]
pub struct COMP4_CSR_SPEC;
impl crate::RegisterSpec for COMP4_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp4_csr::R](R) reader structure"]
impl crate::Readable for COMP4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp4_csr::W](W) writer structure"]
impl crate::Writable for COMP4_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for COMP4_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
