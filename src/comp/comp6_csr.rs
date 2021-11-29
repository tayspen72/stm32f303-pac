#[doc = "Register `COMP6_CSR` reader"]
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_CSR` writer"]
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub struct COMP6EN_R(crate::FieldReader<bool, bool>);
impl COMP6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub struct COMP6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6EN_W<'a> {
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
#[doc = "Field `COMP6MODE` reader - Comparator 6 mode"]
pub struct COMP6MODE_R(crate::FieldReader<u8, u8>);
impl COMP6MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6MODE` writer - Comparator 6 mode"]
pub struct COMP6MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP6INSEL` reader - Comparator 6 inverting input selection"]
pub struct COMP6INSEL_R(crate::FieldReader<u8, u8>);
impl COMP6INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6INSEL` writer - Comparator 6 inverting input selection"]
pub struct COMP6INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP6INPSEL` reader - Comparator 6 non inverted input selection"]
pub struct COMP6INPSEL_R(crate::FieldReader<bool, bool>);
impl COMP6INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6INPSEL` writer - Comparator 6 non inverted input selection"]
pub struct COMP6INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6INPSEL_W<'a> {
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
#[doc = "Field `COM6WINMODE` reader - Comparator 6 window mode"]
pub struct COM6WINMODE_R(crate::FieldReader<bool, bool>);
impl COM6WINMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM6WINMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COM6WINMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COM6WINMODE` writer - Comparator 6 window mode"]
pub struct COM6WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COM6WINMODE_W<'a> {
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
#[doc = "Field `COMP6_OUT_SEL` reader - Comparator 6 output selection"]
pub struct COMP6_OUT_SEL_R(crate::FieldReader<u8, u8>);
impl COMP6_OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6_OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6_OUT_SEL` writer - Comparator 6 output selection"]
pub struct COMP6_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub struct COMP6POL_R(crate::FieldReader<bool, bool>);
impl COMP6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub struct COMP6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6POL_W<'a> {
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
#[doc = "Field `COMP6HYST` reader - Comparator 6 hysteresis"]
pub struct COMP6HYST_R(crate::FieldReader<u8, u8>);
impl COMP6HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6HYST` writer - Comparator 6 hysteresis"]
pub struct COMP6HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub struct COMP6_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP6_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub struct COMP6_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub struct COMP6OUT_R(crate::FieldReader<bool, bool>);
impl COMP6OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub struct COMP6LOCK_R(crate::FieldReader<bool, bool>);
impl COMP6LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP6LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub struct COMP6LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&self) -> COMP6MODE_R {
        COMP6MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6insel(&self) -> COMP6INSEL_R {
        COMP6INSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    pub fn comp6inpsel(&self) -> COMP6INPSEL_R {
        COMP6INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn com6winmode(&self) -> COM6WINMODE_R {
        COM6WINMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6_out_sel(&self) -> COMP6_OUT_SEL_R {
        COMP6_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&self) -> COMP6HYST_R {
        COMP6HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&mut self) -> COMP6EN_W {
        COMP6EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&mut self) -> COMP6MODE_W {
        COMP6MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6insel(&mut self) -> COMP6INSEL_W {
        COMP6INSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    pub fn comp6inpsel(&mut self) -> COMP6INPSEL_W {
        COMP6INPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn com6winmode(&mut self) -> COM6WINMODE_W {
        COM6WINMODE_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6_out_sel(&mut self) -> COMP6_OUT_SEL_W {
        COMP6_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&mut self) -> COMP6POL_W {
        COMP6POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&mut self) -> COMP6HYST_W {
        COMP6HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W {
        COMP6_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W {
        COMP6LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](index.html) module"]
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_csr::R](R) reader structure"]
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](W) writer structure"]
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for COMP6_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
