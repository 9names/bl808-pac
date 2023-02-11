#[doc = "Register `rtc_rst_ctrl2` reader"]
pub struct R(crate::R<RTC_RST_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_RST_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_RST_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_RST_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_rst_ctrl2` writer"]
pub struct W(crate::W<RTC_RST_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_RST_CTRL2_SPEC>;
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
impl From<crate::W<RTC_RST_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_RST_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_resv` reader - "]
pub type RTC_RESV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_resv` writer - "]
pub type RTC_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_RST_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_en_hw_pu_rc32k` reader - "]
pub type REG_EN_HW_PU_RC32K_R = crate::BitReader<bool>;
#[doc = "Field `reg_en_hw_pu_rc32k` writer - "]
pub type REG_EN_HW_PU_RC32K_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTC_RST_CTRL2_SPEC, bool, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_resv(&self) -> RTC_RESV_R {
        RTC_RESV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_en_hw_pu_rc32k(&self) -> REG_EN_HW_PU_RC32K_R {
        REG_EN_HW_PU_RC32K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_resv(&mut self) -> RTC_RESV_W<0> {
        RTC_RESV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_hw_pu_rc32k(&mut self) -> REG_EN_HW_PU_RC32K_W<8> {
        REG_EN_HW_PU_RC32K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc_rst_ctrl2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_rst_ctrl2](index.html) module"]
pub struct RTC_RST_CTRL2_SPEC;
impl crate::RegisterSpec for RTC_RST_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_rst_ctrl2::R](R) reader structure"]
impl crate::Readable for RTC_RST_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_rst_ctrl2::W](W) writer structure"]
impl crate::Writable for RTC_RST_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_rst_ctrl2 to value 0"]
impl crate::Resettable for RTC_RST_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
