#[doc = "Register `wifi_pll_cfg2` reader"]
pub struct R(crate::R<WIFI_PLL_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg2` writer"]
pub struct W(crate::W<WIFI_PLL_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG2_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_sel_cp_bias` reader - "]
pub type WIFIPLL_SEL_CP_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sel_cp_bias` writer - "]
pub type WIFIPLL_SEL_CP_BIAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_icp_5u` reader - "]
pub type WIFIPLL_ICP_5U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_icp_5u` writer - "]
pub type WIFIPLL_ICP_5U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifipll_icp_1u` reader - "]
pub type WIFIPLL_ICP_1U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_icp_1u` writer - "]
pub type WIFIPLL_ICP_1U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifipll_int_frac_sw` reader - "]
pub type WIFIPLL_INT_FRAC_SW_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_int_frac_sw` writer - "]
pub type WIFIPLL_INT_FRAC_SW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `wifipll_cp_startup_en` reader - "]
pub type WIFIPLL_CP_STARTUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_cp_startup_en` writer - "]
pub type WIFIPLL_CP_STARTUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `wifipll_cp_opamp_en` reader - "]
pub type WIFIPLL_CP_OPAMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_cp_opamp_en` writer - "]
pub type WIFIPLL_CP_OPAMP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sel_cp_bias(&self) -> WIFIPLL_SEL_CP_BIAS_R {
        WIFIPLL_SEL_CP_BIAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_icp_5u(&self) -> WIFIPLL_ICP_5U_R {
        WIFIPLL_ICP_5U_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn wifipll_icp_1u(&self) -> WIFIPLL_ICP_1U_R {
        WIFIPLL_ICP_1U_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_int_frac_sw(&self) -> WIFIPLL_INT_FRAC_SW_R {
        WIFIPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_cp_startup_en(&self) -> WIFIPLL_CP_STARTUP_EN_R {
        WIFIPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wifipll_cp_opamp_en(&self) -> WIFIPLL_CP_OPAMP_EN_R {
        WIFIPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sel_cp_bias(&mut self) -> WIFIPLL_SEL_CP_BIAS_W<0> {
        WIFIPLL_SEL_CP_BIAS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_icp_5u(&mut self) -> WIFIPLL_ICP_5U_W<4> {
        WIFIPLL_ICP_5U_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_icp_1u(&mut self) -> WIFIPLL_ICP_1U_W<6> {
        WIFIPLL_ICP_1U_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_int_frac_sw(&mut self) -> WIFIPLL_INT_FRAC_SW_W<8> {
        WIFIPLL_INT_FRAC_SW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cp_startup_en(&mut self) -> WIFIPLL_CP_STARTUP_EN_W<9> {
        WIFIPLL_CP_STARTUP_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cp_opamp_en(&mut self) -> WIFIPLL_CP_OPAMP_EN_W<10> {
        WIFIPLL_CP_OPAMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg2](index.html) module"]
pub struct WIFI_PLL_CFG2_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg2::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg2::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg2 to value 0x0621"]
impl crate::Resettable for WIFI_PLL_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0621;
}
