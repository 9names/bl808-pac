#[doc = "Register `wifi_pll_cfg6` reader"]
pub struct R(crate::R<WIFI_PLL_CFG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg6` writer"]
pub struct W(crate::W<WIFI_PLL_CFG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG6_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_sdmin` reader - "]
pub type WIFIPLL_SDMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `wifipll_sdmin` writer - "]
pub type WIFIPLL_SDMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG6_SPEC, u32, u32, 26, O>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_sdm_bypass` reader - "]
pub type WIFIPLL_SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sdm_bypass` writer - "]
pub type WIFIPLL_SDM_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG6_SPEC, bool, O>;
#[doc = "Field `wifipll_sdm_bypass_hw` reader - "]
pub type WIFIPLL_SDM_BYPASS_HW_R = crate::BitReader<bool>;
#[doc = "Field `reserved_30` reader - "]
pub type RESERVED_30_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sdm_ctrl_hw` reader - "]
pub type WIFIPLL_SDM_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sdm_ctrl_hw` writer - "]
pub type WIFIPLL_SDM_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG6_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn wifipll_sdmin(&self) -> WIFIPLL_SDMIN_R {
        WIFIPLL_SDMIN_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass(&self) -> WIFIPLL_SDM_BYPASS_R {
        WIFIPLL_SDM_BYPASS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass_hw(&self) -> WIFIPLL_SDM_BYPASS_HW_R {
        WIFIPLL_SDM_BYPASS_HW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reserved_30(&self) -> RESERVED_30_R {
        RESERVED_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_sdm_ctrl_hw(&self) -> WIFIPLL_SDM_CTRL_HW_R {
        WIFIPLL_SDM_CTRL_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdmin(&mut self) -> WIFIPLL_SDMIN_W<0> {
        WIFIPLL_SDMIN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_bypass(&mut self) -> WIFIPLL_SDM_BYPASS_W<28> {
        WIFIPLL_SDM_BYPASS_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_ctrl_hw(&mut self) -> WIFIPLL_SDM_CTRL_HW_W<31> {
        WIFIPLL_SDM_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg6](index.html) module"]
pub struct WIFI_PLL_CFG6_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg6::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg6::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg6 to value 0x0180_0000"]
impl crate::Resettable for WIFI_PLL_CFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0180_0000;
}
