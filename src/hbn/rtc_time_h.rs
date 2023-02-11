#[doc = "Register `rtc_time_h` reader"]
pub struct R(crate::R<RTC_TIME_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_time_h` writer"]
pub struct W(crate::W<RTC_TIME_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_H_SPEC>;
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
impl From<crate::W<RTC_TIME_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_time_latch_h` reader - "]
pub type RTC_TIME_LATCH_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_8_30` reader - "]
pub type RESERVED_8_30_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rtc_time_latch` writer - "]
pub type RTC_TIME_LATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_TIME_H_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&self) -> RTC_TIME_LATCH_H_R {
        RTC_TIME_LATCH_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:30"]
    #[inline(always)]
    pub fn reserved_8_30(&self) -> RESERVED_8_30_R {
        RESERVED_8_30_R::new((self.bits >> 8) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_time_latch(&mut self) -> RTC_TIME_LATCH_W<31> {
        RTC_TIME_LATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TIME_H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_h](index.html) module"]
pub struct RTC_TIME_H_SPEC;
impl crate::RegisterSpec for RTC_TIME_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_h::R](R) reader structure"]
impl crate::Readable for RTC_TIME_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_h::W](W) writer structure"]
impl crate::Writable for RTC_TIME_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_time_h to value 0"]
impl crate::Resettable for RTC_TIME_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
