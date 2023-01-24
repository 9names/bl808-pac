#[doc = "Register `wifi_pll_cfg10` reader"]
pub struct R(crate::R<WIFI_PLL_CFG10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg10` writer"]
pub struct W(crate::W<WIFI_PLL_CFG10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG10_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_ssc_start` reader - "]
pub type USBPLL_SSC_START_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_ssc_start` writer - "]
pub type USBPLL_SSC_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn usbpll_ssc_start(&self) -> USBPLL_SSC_START_R {
        USBPLL_SSC_START_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_start(&mut self) -> USBPLL_SSC_START_W<2> {
        USBPLL_SSC_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg10](index.html) module"]
pub struct WIFI_PLL_CFG10_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg10::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg10::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg10 to value 0x04"]
impl crate::Resettable for WIFI_PLL_CFG10_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
