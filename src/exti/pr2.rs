#[doc = "Register `PR2` reader"]
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR2` writer"]
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR32` reader - Pending bit on line 32"]
pub struct PR32_R(crate::FieldReader<bool, bool>);
impl PR32_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR32` writer - Pending bit on line 32"]
pub struct PR32_W<'a> {
    w: &'a mut W,
}
impl<'a> PR32_W<'a> {
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
#[doc = "Field `PR33` reader - Pending bit on line 33"]
pub struct PR33_R(crate::FieldReader<bool, bool>);
impl PR33_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR33` writer - Pending bit on line 33"]
pub struct PR33_W<'a> {
    w: &'a mut W,
}
impl<'a> PR33_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&self) -> PR32_R {
        PR32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&self) -> PR33_R {
        PR33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&mut self) -> PR32_W {
        PR32_W { w: self }
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&mut self) -> PR33_W {
        PR33_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr2::R](R) reader structure"]
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr2::W](W) writer structure"]
impl crate::Writable for PR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
