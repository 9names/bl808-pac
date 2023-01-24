#[doc = "Register `wifi_pll_cfg13` reader"]
pub struct R(crate::R<WIFI_PLL_CFG13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg13` writer"]
pub struct W(crate::W<WIFI_PLL_CFG13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG13_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_resv` reader - "]
pub type WIFIPLL_RESV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wifipll_resv` writer - "]
pub type WIFIPLL_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG13_SPEC, u16, u16, 16, O>;
#[doc = "Field `reserved_16_20` reader - "]
pub type RESERVED_16_20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_dl_ctrl` reader - "]
pub type USBPLL_DL_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_dl_ctrl` writer - "]
pub type USBPLL_DL_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wifipll_resv(&self) -> WIFIPLL_RESV_R {
        WIFIPLL_RESV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reserved_16_20(&self) -> RESERVED_16_20_R {
        RESERVED_16_20_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_dl_ctrl(&self) -> USBPLL_DL_CTRL_R {
        USBPLL_DL_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_resv(&mut self) -> WIFIPLL_RESV_W<0> {
        WIFIPLL_RESV_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_dl_ctrl(&mut self) -> USBPLL_DL_CTRL_W<21> {
        USBPLL_DL_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg13](index.html) module"]
pub struct WIFI_PLL_CFG13_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg13::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg13::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg13 to value 0"]
impl crate::Resettable for WIFI_PLL_CFG13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
