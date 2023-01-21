#[doc = "Register `wifi_pll_cfg5` reader"]
pub struct R(crate::R<WIFI_PLL_CFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg5` writer"]
pub struct W(crate::W<WIFI_PLL_CFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG5_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_vco_speed` reader - "]
pub type WIFIPLL_VCO_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_vco_speed` writer - "]
pub type WIFIPLL_VCO_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG5_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_vco_div1_en` reader - "]
pub type WIFIPLL_VCO_DIV1_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_vco_div1_en` writer - "]
pub type WIFIPLL_VCO_DIV1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG5_SPEC, bool, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_vco_div2_en` reader - "]
pub type WIFIPLL_VCO_DIV2_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_vco_div2_en` writer - "]
pub type WIFIPLL_VCO_DIV2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG5_SPEC, bool, O>;
#[doc = "Field `reserved_9_11` reader - "]
pub type RESERVED_9_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_vco_div3_en` reader - "]
pub type WIFIPLL_VCO_DIV3_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_vco_div3_en` writer - "]
pub type WIFIPLL_VCO_DIV3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG5_SPEC, bool, O>;
#[doc = "Field `reserved_13_31` reader - "]
pub type RESERVED_13_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifipll_vco_speed(&self) -> WIFIPLL_VCO_SPEED_R {
        WIFIPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_vco_div1_en(&self) -> WIFIPLL_VCO_DIV1_EN_R {
        WIFIPLL_VCO_DIV1_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_vco_div2_en(&self) -> WIFIPLL_VCO_DIV2_EN_R {
        WIFIPLL_VCO_DIV2_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reserved_9_11(&self) -> RESERVED_9_11_R {
        RESERVED_9_11_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_vco_div3_en(&self) -> WIFIPLL_VCO_DIV3_EN_R {
        WIFIPLL_VCO_DIV3_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn reserved_13_31(&self) -> RESERVED_13_31_R {
        RESERVED_13_31_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_speed(&mut self) -> WIFIPLL_VCO_SPEED_W<0> {
        WIFIPLL_VCO_SPEED_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div1_en(&mut self) -> WIFIPLL_VCO_DIV1_EN_W<4> {
        WIFIPLL_VCO_DIV1_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div2_en(&mut self) -> WIFIPLL_VCO_DIV2_EN_W<8> {
        WIFIPLL_VCO_DIV2_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div3_en(&mut self) -> WIFIPLL_VCO_DIV3_EN_W<12> {
        WIFIPLL_VCO_DIV3_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg5](index.html) module"]
pub struct WIFI_PLL_CFG5_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg5::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg5::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg5 to value 0x1105"]
impl crate::Resettable for WIFI_PLL_CFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x1105;
}
