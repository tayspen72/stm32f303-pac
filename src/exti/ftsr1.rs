#[doc = "Register `FTSR1` reader"]
pub struct R(crate::R<FTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR1` writer"]
pub struct W(crate::W<FTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR1_SPEC>;
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
impl From<crate::W<FTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR0` reader - Falling trigger event configuration of line 0"]
pub struct TR0_R(crate::FieldReader<bool, bool>);
impl TR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR0` writer - Falling trigger event configuration of line 0"]
pub struct TR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TR0_W<'a> {
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
#[doc = "Field `TR1` reader - Falling trigger event configuration of line 1"]
pub struct TR1_R(crate::FieldReader<bool, bool>);
impl TR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR1` writer - Falling trigger event configuration of line 1"]
pub struct TR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TR1_W<'a> {
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
#[doc = "Field `TR2` reader - Falling trigger event configuration of line 2"]
pub struct TR2_R(crate::FieldReader<bool, bool>);
impl TR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR2` writer - Falling trigger event configuration of line 2"]
pub struct TR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TR2_W<'a> {
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
#[doc = "Field `TR3` reader - Falling trigger event configuration of line 3"]
pub struct TR3_R(crate::FieldReader<bool, bool>);
impl TR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR3` writer - Falling trigger event configuration of line 3"]
pub struct TR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TR3_W<'a> {
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
#[doc = "Field `TR4` reader - Falling trigger event configuration of line 4"]
pub struct TR4_R(crate::FieldReader<bool, bool>);
impl TR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR4` writer - Falling trigger event configuration of line 4"]
pub struct TR4_W<'a> {
    w: &'a mut W,
}
impl<'a> TR4_W<'a> {
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
#[doc = "Field `TR5` reader - Falling trigger event configuration of line 5"]
pub struct TR5_R(crate::FieldReader<bool, bool>);
impl TR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR5` writer - Falling trigger event configuration of line 5"]
pub struct TR5_W<'a> {
    w: &'a mut W,
}
impl<'a> TR5_W<'a> {
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
#[doc = "Field `TR6` reader - Falling trigger event configuration of line 6"]
pub struct TR6_R(crate::FieldReader<bool, bool>);
impl TR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR6` writer - Falling trigger event configuration of line 6"]
pub struct TR6_W<'a> {
    w: &'a mut W,
}
impl<'a> TR6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TR7` reader - Falling trigger event configuration of line 7"]
pub struct TR7_R(crate::FieldReader<bool, bool>);
impl TR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR7` writer - Falling trigger event configuration of line 7"]
pub struct TR7_W<'a> {
    w: &'a mut W,
}
impl<'a> TR7_W<'a> {
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
#[doc = "Field `TR8` reader - Falling trigger event configuration of line 8"]
pub struct TR8_R(crate::FieldReader<bool, bool>);
impl TR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR8` writer - Falling trigger event configuration of line 8"]
pub struct TR8_W<'a> {
    w: &'a mut W,
}
impl<'a> TR8_W<'a> {
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
#[doc = "Field `TR9` reader - Falling trigger event configuration of line 9"]
pub struct TR9_R(crate::FieldReader<bool, bool>);
impl TR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR9` writer - Falling trigger event configuration of line 9"]
pub struct TR9_W<'a> {
    w: &'a mut W,
}
impl<'a> TR9_W<'a> {
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
#[doc = "Field `TR10` reader - Falling trigger event configuration of line 10"]
pub struct TR10_R(crate::FieldReader<bool, bool>);
impl TR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR10` writer - Falling trigger event configuration of line 10"]
pub struct TR10_W<'a> {
    w: &'a mut W,
}
impl<'a> TR10_W<'a> {
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
#[doc = "Field `TR11` reader - Falling trigger event configuration of line 11"]
pub struct TR11_R(crate::FieldReader<bool, bool>);
impl TR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR11` writer - Falling trigger event configuration of line 11"]
pub struct TR11_W<'a> {
    w: &'a mut W,
}
impl<'a> TR11_W<'a> {
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
#[doc = "Field `TR12` reader - Falling trigger event configuration of line 12"]
pub struct TR12_R(crate::FieldReader<bool, bool>);
impl TR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR12` writer - Falling trigger event configuration of line 12"]
pub struct TR12_W<'a> {
    w: &'a mut W,
}
impl<'a> TR12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TR13` reader - Falling trigger event configuration of line 13"]
pub struct TR13_R(crate::FieldReader<bool, bool>);
impl TR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR13` writer - Falling trigger event configuration of line 13"]
pub struct TR13_W<'a> {
    w: &'a mut W,
}
impl<'a> TR13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TR14` reader - Falling trigger event configuration of line 14"]
pub struct TR14_R(crate::FieldReader<bool, bool>);
impl TR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR14` writer - Falling trigger event configuration of line 14"]
pub struct TR14_W<'a> {
    w: &'a mut W,
}
impl<'a> TR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TR15` reader - Falling trigger event configuration of line 15"]
pub struct TR15_R(crate::FieldReader<bool, bool>);
impl TR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR15` writer - Falling trigger event configuration of line 15"]
pub struct TR15_W<'a> {
    w: &'a mut W,
}
impl<'a> TR15_W<'a> {
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
#[doc = "Field `TR16` reader - Falling trigger event configuration of line 16"]
pub struct TR16_R(crate::FieldReader<bool, bool>);
impl TR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR16` writer - Falling trigger event configuration of line 16"]
pub struct TR16_W<'a> {
    w: &'a mut W,
}
impl<'a> TR16_W<'a> {
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
#[doc = "Field `TR17` reader - Falling trigger event configuration of line 17"]
pub struct TR17_R(crate::FieldReader<bool, bool>);
impl TR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR17` writer - Falling trigger event configuration of line 17"]
pub struct TR17_W<'a> {
    w: &'a mut W,
}
impl<'a> TR17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TR18` reader - Falling trigger event configuration of line 18"]
pub struct TR18_R(crate::FieldReader<bool, bool>);
impl TR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR18` writer - Falling trigger event configuration of line 18"]
pub struct TR18_W<'a> {
    w: &'a mut W,
}
impl<'a> TR18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TR19` reader - Falling trigger event configuration of line 19"]
pub struct TR19_R(crate::FieldReader<bool, bool>);
impl TR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR19` writer - Falling trigger event configuration of line 19"]
pub struct TR19_W<'a> {
    w: &'a mut W,
}
impl<'a> TR19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TR20` reader - Falling trigger event configuration of line 20"]
pub struct TR20_R(crate::FieldReader<bool, bool>);
impl TR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR20` writer - Falling trigger event configuration of line 20"]
pub struct TR20_W<'a> {
    w: &'a mut W,
}
impl<'a> TR20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TR21` reader - Falling trigger event configuration of line 21"]
pub struct TR21_R(crate::FieldReader<bool, bool>);
impl TR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR21` writer - Falling trigger event configuration of line 21"]
pub struct TR21_W<'a> {
    w: &'a mut W,
}
impl<'a> TR21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TR22` reader - Falling trigger event configuration of line 22"]
pub struct TR22_R(crate::FieldReader<bool, bool>);
impl TR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR22` writer - Falling trigger event configuration of line 22"]
pub struct TR22_W<'a> {
    w: &'a mut W,
}
impl<'a> TR22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TR29` reader - Falling trigger event configuration of line 29"]
pub struct TR29_R(crate::FieldReader<bool, bool>);
impl TR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR29` writer - Falling trigger event configuration of line 29"]
pub struct TR29_W<'a> {
    w: &'a mut W,
}
impl<'a> TR29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TR30` reader - Falling trigger event configuration of line 30."]
pub struct TR30_R(crate::FieldReader<bool, bool>);
impl TR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR30` writer - Falling trigger event configuration of line 30."]
pub struct TR30_W<'a> {
    w: &'a mut W,
}
impl<'a> TR30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `TR31` reader - Falling trigger event configuration of line 31"]
pub struct TR31_R(crate::FieldReader<bool, bool>);
impl TR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR31` writer - Falling trigger event configuration of line 31"]
pub struct TR31_W<'a> {
    w: &'a mut W,
}
impl<'a> TR31_W<'a> {
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
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn tr19(&self) -> TR19_R {
        TR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn tr20(&self) -> TR20_R {
        TR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn tr21(&self) -> TR21_R {
        TR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn tr22(&self) -> TR22_R {
        TR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Falling trigger event configuration of line 29"]
    #[inline(always)]
    pub fn tr29(&self) -> TR29_R {
        TR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Falling trigger event configuration of line 30."]
    #[inline(always)]
    pub fn tr30(&self) -> TR30_R {
        TR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Falling trigger event configuration of line 31"]
    #[inline(always)]
    pub fn tr31(&self) -> TR31_R {
        TR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&mut self) -> TR0_W {
        TR0_W { w: self }
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&mut self) -> TR1_W {
        TR1_W { w: self }
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&mut self) -> TR2_W {
        TR2_W { w: self }
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&mut self) -> TR3_W {
        TR3_W { w: self }
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&mut self) -> TR4_W {
        TR4_W { w: self }
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&mut self) -> TR5_W {
        TR5_W { w: self }
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&mut self) -> TR6_W {
        TR6_W { w: self }
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&mut self) -> TR7_W {
        TR7_W { w: self }
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&mut self) -> TR8_W {
        TR8_W { w: self }
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&mut self) -> TR9_W {
        TR9_W { w: self }
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&mut self) -> TR10_W {
        TR10_W { w: self }
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&mut self) -> TR11_W {
        TR11_W { w: self }
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&mut self) -> TR12_W {
        TR12_W { w: self }
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&mut self) -> TR13_W {
        TR13_W { w: self }
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&mut self) -> TR14_W {
        TR14_W { w: self }
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&mut self) -> TR15_W {
        TR15_W { w: self }
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&mut self) -> TR16_W {
        TR16_W { w: self }
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&mut self) -> TR17_W {
        TR17_W { w: self }
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    pub fn tr18(&mut self) -> TR18_W {
        TR18_W { w: self }
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn tr19(&mut self) -> TR19_W {
        TR19_W { w: self }
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn tr20(&mut self) -> TR20_W {
        TR20_W { w: self }
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn tr21(&mut self) -> TR21_W {
        TR21_W { w: self }
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn tr22(&mut self) -> TR22_W {
        TR22_W { w: self }
    }
    #[doc = "Bit 29 - Falling trigger event configuration of line 29"]
    #[inline(always)]
    pub fn tr29(&mut self) -> TR29_W {
        TR29_W { w: self }
    }
    #[doc = "Bit 30 - Falling trigger event configuration of line 30."]
    #[inline(always)]
    pub fn tr30(&mut self) -> TR30_W {
        TR30_W { w: self }
    }
    #[doc = "Bit 31 - Falling trigger event configuration of line 31"]
    #[inline(always)]
    pub fn tr31(&mut self) -> TR31_W {
        TR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Falling Trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](index.html) module"]
pub struct FTSR1_SPEC;
impl crate::RegisterSpec for FTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr1::R](R) reader structure"]
impl crate::Readable for FTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](W) writer structure"]
impl crate::Writable for FTSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTSR1 to value 0"]
impl crate::Resettable for FTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
