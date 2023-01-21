#[doc = "Register `mcu_e907_rtc` reader"]
pub struct R(crate::R<MCU_E907_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU_E907_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU_E907_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU_E907_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu_e907_rtc` writer"]
pub struct W(crate::W<MCU_E907_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU_E907_RTC_SPEC>;
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
impl From<crate::W<MCU_E907_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU_E907_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mcu_rtc_div` reader - "]
pub type REG_MCU_RTC_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mcu_rtc_div` writer - "]
pub type REG_MCU_RTC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCU_E907_RTC_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_10_29` reader - "]
pub type RESERVED_10_29_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_mcu_rtc_rst` reader - "]
pub type REG_MCU_RTC_RST_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_rtc_rst` writer - "]
pub type REG_MCU_RTC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_E907_RTC_SPEC, bool, O>;
#[doc = "Field `reg_mcu_rtc_en` reader - "]
pub type REG_MCU_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_rtc_en` writer - "]
pub type REG_MCU_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_E907_RTC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reg_mcu_rtc_div(&self) -> REG_MCU_RTC_DIV_R {
        REG_MCU_RTC_DIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:29"]
    #[inline(always)]
    pub fn reserved_10_29(&self) -> RESERVED_10_29_R {
        RESERVED_10_29_R::new((self.bits >> 10) & 0x000f_ffff)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_mcu_rtc_rst(&self) -> REG_MCU_RTC_RST_R {
        REG_MCU_RTC_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mcu_rtc_en(&self) -> REG_MCU_RTC_EN_R {
        REG_MCU_RTC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_div(&mut self) -> REG_MCU_RTC_DIV_W<0> {
        REG_MCU_RTC_DIV_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_rst(&mut self) -> REG_MCU_RTC_RST_W<30> {
        REG_MCU_RTC_RST_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_en(&mut self) -> REG_MCU_RTC_EN_W<31> {
        REG_MCU_RTC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu_e907_rtc\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_e907_rtc](index.html) module"]
pub struct MCU_E907_RTC_SPEC;
impl crate::RegisterSpec for MCU_E907_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu_e907_rtc::R](R) reader structure"]
impl crate::Readable for MCU_E907_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu_e907_rtc::W](W) writer structure"]
impl crate::Writable for MCU_E907_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_e907_rtc to value 0x8000_000a"]
impl crate::Resettable for MCU_E907_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_000a;
}
