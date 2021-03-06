#[doc = "Register `ARR` reader"]
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARR` writer"]
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARRL` reader - Low Auto-reload value"]
pub struct ARRL_R(crate::FieldReader<u16, u16>);
impl ARRL_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARRL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARRL` writer - Low Auto-reload value"]
pub struct ARRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ARRH` reader - High Auto-reload value"]
pub struct ARRH_R(crate::FieldReader<u16, u16>);
impl ARRH_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARRH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARRH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARRH` writer - High Auto-reload value"]
pub struct ARRH_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arrl(&self) -> ARRL_R {
        ARRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arrh(&self) -> ARRH_R {
        ARRH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arrl(&mut self) -> ARRL_W {
        ARRL_W { w: self }
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arrh(&mut self) -> ARRH_W {
        ARRH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](index.html) module"]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arr::R](R) reader structure"]
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arr::W](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
