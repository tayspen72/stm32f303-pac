#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL7_R(crate::FieldReader<u8, u8>);
impl AFRL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL6_R(crate::FieldReader<u8, u8>);
impl AFRL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL5_R(crate::FieldReader<u8, u8>);
impl AFRL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL4_R(crate::FieldReader<u8, u8>);
impl AFRL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL3_R(crate::FieldReader<u8, u8>);
impl AFRL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL2_R(crate::FieldReader<u8, u8>);
impl AFRL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL1_R(crate::FieldReader<u8, u8>);
impl AFRL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL0_R(crate::FieldReader<u8, u8>);
impl AFRL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub struct AFRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&mut self) -> AFRL7_W {
        AFRL7_W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&mut self) -> AFRL6_W {
        AFRL6_W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&mut self) -> AFRL5_W {
        AFRL5_W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&mut self) -> AFRL4_W {
        AFRL4_W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&mut self) -> AFRL3_W {
        AFRL3_W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&mut self) -> AFRL2_W {
        AFRL2_W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&mut self) -> AFRL1_W {
        AFRL1_W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&mut self) -> AFRL0_W {
        AFRL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
