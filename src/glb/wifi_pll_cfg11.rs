#[doc = "Register `wifi_pll_cfg11` reader"]
pub struct R(crate::R<WIFI_PLL_CFG11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg11` writer"]
pub struct W(crate::W<WIFI_PLL_CFG11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG11_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usbpll_sdmin` reader - "]
pub type USBPLL_SDMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `usbpll_sdmin` writer - "]
pub type USBPLL_SDMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG11_SPEC, u32, u32, 19, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn usbpll_sdmin(&self) -> USBPLL_SDMIN_R {
        USBPLL_SDMIN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn reserved_19_31(&self) -> RESERVED_19_31_R {
        RESERVED_19_31_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdmin(&mut self) -> USBPLL_SDMIN_W<0> {
        USBPLL_SDMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg11](index.html) module"]
pub struct WIFI_PLL_CFG11_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg11::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg11::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg11 to value 0x0002_8000"]
impl crate::Resettable for WIFI_PLL_CFG11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_8000;
}
