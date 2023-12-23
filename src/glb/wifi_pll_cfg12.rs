#[doc = "Register `wifi_pll_cfg12` reader"]
pub struct R(crate::R<WIFI_PLL_CFG12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg12` writer"]
pub struct W(crate::W<WIFI_PLL_CFG12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG12_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usbpll_ssc_cnt` reader - "]
pub type USBPLL_SSC_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `usbpll_ssc_cnt` writer - "]
pub type USBPLL_SSC_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG12_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn usbpll_ssc_cnt(&self) -> USBPLL_SSC_CNT_R {
        USBPLL_SSC_CNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_cnt(&mut self) -> USBPLL_SSC_CNT_W<0> {
        USBPLL_SSC_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg12](index.html) module"]
pub struct WIFI_PLL_CFG12_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg12::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg12::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg12 to value 0xf0"]
impl crate::Resettable for WIFI_PLL_CFG12_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
