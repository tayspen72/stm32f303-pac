#[doc = "Register `PR1` reader"]
pub struct R(crate::R<PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR1` writer"]
pub struct W(crate::W<PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1_SPEC>;
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
impl From<crate::W<PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - Pending bit 0"]
pub struct PR0_R(crate::FieldReader<bool, bool>);
impl PR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR0` writer - Pending bit 0"]
pub struct PR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR0_W<'a> {
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
#[doc = "Field `PR1` reader - Pending bit 1"]
pub struct PR1_R(crate::FieldReader<bool, bool>);
impl PR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR1` writer - Pending bit 1"]
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
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
#[doc = "Field `PR2` reader - Pending bit 2"]
pub struct PR2_R(crate::FieldReader<bool, bool>);
impl PR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR2` writer - Pending bit 2"]
pub struct PR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PR2_W<'a> {
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
#[doc = "Field `PR3` reader - Pending bit 3"]
pub struct PR3_R(crate::FieldReader<bool, bool>);
impl PR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR3` writer - Pending bit 3"]
pub struct PR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PR3_W<'a> {
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
#[doc = "Field `PR4` reader - Pending bit 4"]
pub struct PR4_R(crate::FieldReader<bool, bool>);
impl PR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR4` writer - Pending bit 4"]
pub struct PR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PR4_W<'a> {
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
#[doc = "Field `PR5` reader - Pending bit 5"]
pub struct PR5_R(crate::FieldReader<bool, bool>);
impl PR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR5` writer - Pending bit 5"]
pub struct PR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PR5_W<'a> {
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
#[doc = "Field `PR6` reader - Pending bit 6"]
pub struct PR6_R(crate::FieldReader<bool, bool>);
impl PR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR6` writer - Pending bit 6"]
pub struct PR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PR6_W<'a> {
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
#[doc = "Field `PR7` reader - Pending bit 7"]
pub struct PR7_R(crate::FieldReader<bool, bool>);
impl PR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR7` writer - Pending bit 7"]
pub struct PR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PR7_W<'a> {
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
#[doc = "Field `PR8` reader - Pending bit 8"]
pub struct PR8_R(crate::FieldReader<bool, bool>);
impl PR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR8` writer - Pending bit 8"]
pub struct PR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PR8_W<'a> {
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
#[doc = "Field `PR9` reader - Pending bit 9"]
pub struct PR9_R(crate::FieldReader<bool, bool>);
impl PR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR9` writer - Pending bit 9"]
pub struct PR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PR9_W<'a> {
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
#[doc = "Field `PR10` reader - Pending bit 10"]
pub struct PR10_R(crate::FieldReader<bool, bool>);
impl PR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR10` writer - Pending bit 10"]
pub struct PR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PR10_W<'a> {
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
#[doc = "Field `PR11` reader - Pending bit 11"]
pub struct PR11_R(crate::FieldReader<bool, bool>);
impl PR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR11` writer - Pending bit 11"]
pub struct PR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PR11_W<'a> {
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
#[doc = "Field `PR12` reader - Pending bit 12"]
pub struct PR12_R(crate::FieldReader<bool, bool>);
impl PR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR12` writer - Pending bit 12"]
pub struct PR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PR12_W<'a> {
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
#[doc = "Field `PR13` reader - Pending bit 13"]
pub struct PR13_R(crate::FieldReader<bool, bool>);
impl PR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR13` writer - Pending bit 13"]
pub struct PR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PR13_W<'a> {
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
#[doc = "Field `PR14` reader - Pending bit 14"]
pub struct PR14_R(crate::FieldReader<bool, bool>);
impl PR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR14` writer - Pending bit 14"]
pub struct PR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PR14_W<'a> {
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
#[doc = "Field `PR15` reader - Pending bit 15"]
pub struct PR15_R(crate::FieldReader<bool, bool>);
impl PR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR15` writer - Pending bit 15"]
pub struct PR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PR15_W<'a> {
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
#[doc = "Field `PR16` reader - Pending bit 16"]
pub struct PR16_R(crate::FieldReader<bool, bool>);
impl PR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR16` writer - Pending bit 16"]
pub struct PR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PR16_W<'a> {
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
#[doc = "Field `PR17` reader - Pending bit 17"]
pub struct PR17_R(crate::FieldReader<bool, bool>);
impl PR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR17` writer - Pending bit 17"]
pub struct PR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PR17_W<'a> {
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
#[doc = "Field `PR18` reader - Pending bit 18"]
pub struct PR18_R(crate::FieldReader<bool, bool>);
impl PR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR18` writer - Pending bit 18"]
pub struct PR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PR18_W<'a> {
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
#[doc = "Field `PR19` reader - Pending bit 19"]
pub struct PR19_R(crate::FieldReader<bool, bool>);
impl PR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR19` writer - Pending bit 19"]
pub struct PR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PR19_W<'a> {
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
#[doc = "Field `PR20` reader - Pending bit 20"]
pub struct PR20_R(crate::FieldReader<bool, bool>);
impl PR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR20` writer - Pending bit 20"]
pub struct PR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PR20_W<'a> {
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
#[doc = "Field `PR21` reader - Pending bit 21"]
pub struct PR21_R(crate::FieldReader<bool, bool>);
impl PR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR21` writer - Pending bit 21"]
pub struct PR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PR21_W<'a> {
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
#[doc = "Field `PR22` reader - Pending bit 22"]
pub struct PR22_R(crate::FieldReader<bool, bool>);
impl PR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR22` writer - Pending bit 22"]
pub struct PR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PR22_W<'a> {
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
#[doc = "Field `PR29` reader - Pending bit 29"]
pub struct PR29_R(crate::FieldReader<bool, bool>);
impl PR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR29` writer - Pending bit 29"]
pub struct PR29_W<'a> {
    w: &'a mut W,
}
impl<'a> PR29_W<'a> {
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
#[doc = "Field `PR30` reader - Pending bit 30"]
pub struct PR30_R(crate::FieldReader<bool, bool>);
impl PR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR30` writer - Pending bit 30"]
pub struct PR30_W<'a> {
    w: &'a mut W,
}
impl<'a> PR30_W<'a> {
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
#[doc = "Field `PR31` reader - Pending bit 31"]
pub struct PR31_R(crate::FieldReader<bool, bool>);
impl PR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR31` writer - Pending bit 31"]
pub struct PR31_W<'a> {
    w: &'a mut W,
}
impl<'a> PR31_W<'a> {
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
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pr22(&self) -> PR22_R {
        PR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pending bit 29"]
    #[inline(always)]
    pub fn pr29(&self) -> PR29_R {
        PR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pending bit 30"]
    #[inline(always)]
    pub fn pr30(&self) -> PR30_R {
        PR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pending bit 31"]
    #[inline(always)]
    pub fn pr31(&self) -> PR31_R {
        PR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W { w: self }
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W {
        PR2_W { w: self }
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W { w: self }
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W { w: self }
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W { w: self }
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W {
        PR6_W { w: self }
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W {
        PR7_W { w: self }
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W {
        PR8_W { w: self }
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W {
        PR9_W { w: self }
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W {
        PR10_W { w: self }
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W {
        PR11_W { w: self }
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W {
        PR12_W { w: self }
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W {
        PR13_W { w: self }
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W {
        PR14_W { w: self }
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W {
        PR15_W { w: self }
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W {
        PR16_W { w: self }
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W {
        PR17_W { w: self }
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W {
        PR18_W { w: self }
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W {
        PR19_W { w: self }
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pr20(&mut self) -> PR20_W {
        PR20_W { w: self }
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pr21(&mut self) -> PR21_W {
        PR21_W { w: self }
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pr22(&mut self) -> PR22_W {
        PR22_W { w: self }
    }
    #[doc = "Bit 29 - Pending bit 29"]
    #[inline(always)]
    pub fn pr29(&mut self) -> PR29_W {
        PR29_W { w: self }
    }
    #[doc = "Bit 30 - Pending bit 30"]
    #[inline(always)]
    pub fn pr30(&mut self) -> PR30_W {
        PR30_W { w: self }
    }
    #[doc = "Bit 31 - Pending bit 31"]
    #[inline(always)]
    pub fn pr31(&mut self) -> PR31_W {
        PR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](index.html) module"]
pub struct PR1_SPEC;
impl crate::RegisterSpec for PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr1::R](R) reader structure"]
impl crate::Readable for PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr1::W](W) writer structure"]
impl crate::Writable for PR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
