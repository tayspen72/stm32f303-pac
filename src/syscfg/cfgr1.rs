#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub struct MEM_MODE_R(crate::FieldReader<u8, u8>);
impl MEM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub struct USB_IT_RMP_R(crate::FieldReader<bool, bool>);
impl USB_IT_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_IT_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_IT_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub struct USB_IT_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_IT_RMP_W<'a> {
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
#[doc = "Field `TIM1_ITR_RMP` reader - Timer 1 ITR3 selection"]
pub struct TIM1_ITR_RMP_R(crate::FieldReader<bool, bool>);
impl TIM1_ITR_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_ITR_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_ITR_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_ITR_RMP` writer - Timer 1 ITR3 selection"]
pub struct TIM1_ITR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_ITR_RMP_W<'a> {
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
#[doc = "Field `DAC_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub struct DAC_TRIG_RMP_R(crate::FieldReader<bool, bool>);
impl DAC_TRIG_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_TRIG_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_TRIG_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub struct DAC_TRIG_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_TRIG_RMP_W<'a> {
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
#[doc = "Field `ADC24_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub struct ADC24_DMA_RMP_R(crate::FieldReader<bool, bool>);
impl ADC24_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC24_DMA_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC24_DMA_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC24_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub struct ADC24_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC24_DMA_RMP_W<'a> {
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
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub struct TIM16_DMA_RMP_R(crate::FieldReader<bool, bool>);
impl TIM16_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16_DMA_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16_DMA_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub struct TIM16_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_DMA_RMP_W<'a> {
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
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub struct TIM17_DMA_RMP_R(crate::FieldReader<bool, bool>);
impl TIM17_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17_DMA_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17_DMA_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub struct TIM17_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_DMA_RMP_W<'a> {
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
#[doc = "Field `TIM6_DAC1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub struct TIM6_DAC1_DMA_RMP_R(crate::FieldReader<bool, bool>);
impl TIM6_DAC1_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6_DAC1_DMA_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6_DAC1_DMA_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6_DAC1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub struct TIM6_DAC1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6_DAC1_DMA_RMP_W<'a> {
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
#[doc = "Field `TIM7_DAC2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub struct TIM7_DAC2_DMA_RMP_R(crate::FieldReader<bool, bool>);
impl TIM7_DAC2_DMA_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7_DAC2_DMA_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7_DAC2_DMA_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7_DAC2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub struct TIM7_DAC2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7_DAC2_DMA_RMP_W<'a> {
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
#[doc = "Field `I2C_PB6_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB6_FM_R(crate::FieldReader<bool, bool>);
impl I2C_PB6_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB6_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB6_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB6_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB6_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB6_FM_W<'a> {
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
#[doc = "Field `I2C_PB7_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB7_FM_R(crate::FieldReader<bool, bool>);
impl I2C_PB7_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB7_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB7_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB7_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB7_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB7_FM_W<'a> {
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
#[doc = "Field `I2C_PB8_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB8_FM_R(crate::FieldReader<bool, bool>);
impl I2C_PB8_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB8_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB8_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB8_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB8_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB8_FM_W<'a> {
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
#[doc = "Field `I2C_PB9_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB9_FM_R(crate::FieldReader<bool, bool>);
impl I2C_PB9_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB9_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB9_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PB9_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub struct I2C_PB9_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB9_FM_W<'a> {
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
#[doc = "Field `I2C1_FM` reader - I2C1 Fast Mode Plus"]
pub struct I2C1_FM_R(crate::FieldReader<bool, bool>);
impl I2C1_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_FM` writer - I2C1 Fast Mode Plus"]
pub struct I2C1_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FM_W<'a> {
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
#[doc = "Field `I2C2_FM` reader - I2C2 Fast Mode Plus"]
pub struct I2C2_FM_R(crate::FieldReader<bool, bool>);
impl I2C2_FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_FM` writer - I2C2 Fast Mode Plus"]
pub struct I2C2_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FM_W<'a> {
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
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub struct ENCODER_MODE_R(crate::FieldReader<u8, u8>);
impl ENCODER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENCODER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCODER_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub struct ENCODER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODER_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `FPU_IT` reader - Interrupt enable bits from FPU"]
pub struct FPU_IT_R(crate::FieldReader<u8, u8>);
impl FPU_IT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPU_IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPU_IT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IT` writer - Interrupt enable bits from FPU"]
pub struct FPU_IT_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> USB_IT_RMP_R {
        USB_IT_RMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr_rmp(&self) -> TIM1_ITR_RMP_R {
        TIM1_ITR_RMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac_trig_rmp(&self) -> DAC_TRIG_RMP_R {
        DAC_TRIG_RMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc24_dma_rmp(&self) -> ADC24_DMA_RMP_R {
        ADC24_DMA_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&self) -> TIM6_DAC1_DMA_RMP_R {
        TIM6_DAC1_DMA_RMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac2_dma_rmp(&self) -> TIM7_DAC2_DMA_RMP_R {
        TIM7_DAC2_DMA_RMP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&self) -> I2C_PB6_FM_R {
        I2C_PB6_FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&self) -> I2C_PB7_FM_R {
        I2C_PB7_FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&self) -> I2C_PB8_FM_R {
        I2C_PB8_FM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&self) -> I2C_PB9_FM_R {
        I2C_PB9_FM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fm(&self) -> I2C1_FM_R {
        I2C1_FM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fm(&self) -> I2C2_FM_R {
        I2C2_FM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    pub fn fpu_it(&self) -> FPU_IT_R {
        FPU_IT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&mut self) -> USB_IT_RMP_W {
        USB_IT_RMP_W { w: self }
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr_rmp(&mut self) -> TIM1_ITR_RMP_W {
        TIM1_ITR_RMP_W { w: self }
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac_trig_rmp(&mut self) -> DAC_TRIG_RMP_W {
        DAC_TRIG_RMP_W { w: self }
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc24_dma_rmp(&mut self) -> ADC24_DMA_RMP_W {
        ADC24_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W {
        TIM16_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W {
        TIM17_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&mut self) -> TIM6_DAC1_DMA_RMP_W {
        TIM6_DAC1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac2_dma_rmp(&mut self) -> TIM7_DAC2_DMA_RMP_W {
        TIM7_DAC2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&mut self) -> I2C_PB6_FM_W {
        I2C_PB6_FM_W { w: self }
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&mut self) -> I2C_PB7_FM_W {
        I2C_PB7_FM_W { w: self }
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&mut self) -> I2C_PB8_FM_W {
        I2C_PB8_FM_W { w: self }
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&mut self) -> I2C_PB9_FM_W {
        I2C_PB9_FM_W { w: self }
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fm(&mut self) -> I2C1_FM_W {
        I2C1_FM_W { w: self }
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fm(&mut self) -> I2C2_FM_W {
        I2C2_FM_W { w: self }
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W {
        ENCODER_MODE_W { w: self }
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    pub fn fpu_it(&mut self) -> FPU_IT_W {
        FPU_IT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
