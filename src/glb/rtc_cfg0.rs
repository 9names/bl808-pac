#[doc = "Register `rtc_cfg0` reader"]
pub struct R(crate::R<RTC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_cfg0` writer"]
pub struct W(crate::W<RTC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CFG0_SPEC>;
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
impl From<crate::W<RTC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_rtc_div` reader - "]
pub type CPU_RTC_DIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cpu_rtc_div` writer - "]
pub type CPU_RTC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_CFG0_SPEC, u32, u32, 17, O>;
#[doc = "Field `reserved_17` reader - "]
pub type RESERVED_17_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rtc_en` reader - "]
pub type CPU_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rtc_en` writer - "]
pub type CPU_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CFG0_SPEC, bool, O>;
#[doc = "Field `cpu_rtc_sel` reader - "]
pub type CPU_RTC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rtc_sel` writer - "]
pub type CPU_RTC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_20_31` reader - "]
pub type RESERVED_20_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CPU_RTC_DIV_R {
        CPU_RTC_DIV_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reserved_17(&self) -> RESERVED_17_R {
        RESERVED_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CPU_RTC_EN_R {
        CPU_RTC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CPU_RTC_SEL_R {
        CPU_RTC_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn reserved_20_31(&self) -> RESERVED_20_31_R {
        RESERVED_20_31_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_div(&mut self) -> CPU_RTC_DIV_W<0> {
        CPU_RTC_DIV_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_en(&mut self) -> CPU_RTC_EN_W<18> {
        CPU_RTC_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_sel(&mut self) -> CPU_RTC_SEL_W<19> {
        CPU_RTC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cfg0](index.html) module"]
pub struct RTC_CFG0_SPEC;
impl crate::RegisterSpec for RTC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cfg0::R](R) reader structure"]
impl crate::Readable for RTC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cfg0::W](W) writer structure"]
impl crate::Writable for RTC_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_cfg0 to value 0x0008_0010"]
impl crate::Resettable for RTC_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0010;
}
