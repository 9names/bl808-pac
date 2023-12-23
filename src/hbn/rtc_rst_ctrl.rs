#[doc = "Register `rtc_rst_ctrl` reader"]
pub struct R(crate::R<RTC_RST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_RST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_RST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_RST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_rst_ctrl` writer"]
pub struct W(crate::W<RTC_RST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_RST_CTRL_SPEC>;
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
impl From<crate::W<RTC_RST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_RST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_rst_wait_cnt_rtc` reader - "]
pub type RTC_RST_WAIT_CNT_RTC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rtc_rst_wait_cnt_rtc` writer - "]
pub type RTC_RST_WAIT_CNT_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_RST_CTRL_SPEC, u16, u16, 16, O>;
#[doc = "Field `rtc_rst_refdiv_rtc` reader - "]
pub type RTC_RST_REFDIV_RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_rst_refdiv_rtc` writer - "]
pub type RTC_RST_REFDIV_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_RST_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `rtc_rst_ctrl_misc` reader - "]
pub type RTC_RST_CTRL_MISC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rtc_rst_ctrl_misc` writer - "]
pub type RTC_RST_CTRL_MISC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_RST_CTRL_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rtc_rst_wait_cnt_rtc(&self) -> RTC_RST_WAIT_CNT_RTC_R {
        RTC_RST_WAIT_CNT_RTC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rtc_rst_refdiv_rtc(&self) -> RTC_RST_REFDIV_RTC_R {
        RTC_RST_REFDIV_RTC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn rtc_rst_ctrl_misc(&self) -> RTC_RST_CTRL_MISC_R {
        RTC_RST_CTRL_MISC_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_rst_wait_cnt_rtc(&mut self) -> RTC_RST_WAIT_CNT_RTC_W<0> {
        RTC_RST_WAIT_CNT_RTC_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_rst_refdiv_rtc(&mut self) -> RTC_RST_REFDIV_RTC_W<16> {
        RTC_RST_REFDIV_RTC_W::new(self)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_rst_ctrl_misc(&mut self) -> RTC_RST_CTRL_MISC_W<19> {
        RTC_RST_CTRL_MISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc_rst_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_rst_ctrl](index.html) module"]
pub struct RTC_RST_CTRL_SPEC;
impl crate::RegisterSpec for RTC_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_rst_ctrl::R](R) reader structure"]
impl crate::Readable for RTC_RST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_rst_ctrl::W](W) writer structure"]
impl crate::Writable for RTC_RST_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_rst_ctrl to value 0x5094_3c00"]
impl crate::Resettable for RTC_RST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x5094_3c00;
}
