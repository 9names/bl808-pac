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
#[doc = "Field `usbpll_ssc_start_gate_en` reader - "]
pub type USBPLL_SSC_START_GATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_ssc_start_gate_en` writer - "]
pub type USBPLL_SSC_START_GATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `usbpll_ssc_gain` reader - "]
pub type USBPLL_SSC_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_ssc_gain` writer - "]
pub type USBPLL_SSC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG10_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_ssc_en` reader - "]
pub type USBPLL_SSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_ssc_en` writer - "]
pub type USBPLL_SSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `usbpll_sdm_bypass` reader - "]
pub type USBPLL_SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_sdm_bypass` writer - "]
pub type USBPLL_SDM_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `usbpll_sdm_order_sel` reader - "]
pub type USBPLL_SDM_ORDER_SEL_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_sdm_order_sel` writer - "]
pub type USBPLL_SDM_ORDER_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_11_15` reader - "]
pub type RESERVED_11_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_sdm_sig_dith_sel` reader - "]
pub type USBPLL_SDM_SIG_DITH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_sdm_sig_dith_sel` writer - "]
pub type USBPLL_SDM_SIG_DITH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG10_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_19` reader - "]
pub type RESERVED_18_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_div2_en` reader - "]
pub type USBPLL_DIV2_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_div2_en` writer - "]
pub type USBPLL_DIV2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `usbpll_clkout_en` reader - "]
pub type USBPLL_CLKOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_clkout_en` writer - "]
pub type USBPLL_CLKOUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_22_23` reader - "]
pub type RESERVED_22_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_sel_sample_clk` reader - "]
pub type USBPLL_SEL_SAMPLE_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_sel_sample_clk` writer - "]
pub type USBPLL_SEL_SAMPLE_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG10_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_rstb` reader - "]
pub type USBPLL_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_rstb` writer - "]
pub type USBPLL_RSTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `pu_usbpll_mmdiv` reader - "]
pub type PU_USBPLL_MMDIV_R = crate::BitReader<bool>;
#[doc = "Field `pu_usbpll_mmdiv` writer - "]
pub type PU_USBPLL_MMDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbpll_ssc_start_gate_en(&self) -> USBPLL_SSC_START_GATE_EN_R {
        USBPLL_SSC_START_GATE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usbpll_ssc_gain(&self) -> USBPLL_SSC_GAIN_R {
        USBPLL_SSC_GAIN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usbpll_ssc_en(&self) -> USBPLL_SSC_EN_R {
        USBPLL_SSC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn usbpll_sdm_bypass(&self) -> USBPLL_SDM_BYPASS_R {
        USBPLL_SDM_BYPASS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_sdm_order_sel(&self) -> USBPLL_SDM_ORDER_SEL_R {
        USBPLL_SDM_ORDER_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn reserved_11_15(&self) -> RESERVED_11_15_R {
        RESERVED_11_15_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbpll_sdm_sig_dith_sel(&self) -> USBPLL_SDM_SIG_DITH_SEL_R {
        USBPLL_SDM_SIG_DITH_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reserved_18_19(&self) -> RESERVED_18_19_R {
        RESERVED_18_19_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usbpll_div2_en(&self) -> USBPLL_DIV2_EN_R {
        USBPLL_DIV2_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_clkout_en(&self) -> USBPLL_CLKOUT_EN_R {
        USBPLL_CLKOUT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reserved_22_23(&self) -> RESERVED_22_23_R {
        RESERVED_22_23_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn usbpll_sel_sample_clk(&self) -> USBPLL_SEL_SAMPLE_CLK_R {
        USBPLL_SEL_SAMPLE_CLK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usbpll_rstb(&self) -> USBPLL_RSTB_R {
        USBPLL_RSTB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_usbpll_mmdiv(&self) -> PU_USBPLL_MMDIV_R {
        PU_USBPLL_MMDIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_start(&mut self) -> USBPLL_SSC_START_W<2> {
        USBPLL_SSC_START_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_start_gate_en(&mut self) -> USBPLL_SSC_START_GATE_EN_W<3> {
        USBPLL_SSC_START_GATE_EN_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_gain(&mut self) -> USBPLL_SSC_GAIN_W<4> {
        USBPLL_SSC_GAIN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_ssc_en(&mut self) -> USBPLL_SSC_EN_W<8> {
        USBPLL_SSC_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_bypass(&mut self) -> USBPLL_SDM_BYPASS_W<9> {
        USBPLL_SDM_BYPASS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_order_sel(&mut self) -> USBPLL_SDM_ORDER_SEL_W<10> {
        USBPLL_SDM_ORDER_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_sig_dith_sel(&mut self) -> USBPLL_SDM_SIG_DITH_SEL_W<16> {
        USBPLL_SDM_SIG_DITH_SEL_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_div2_en(&mut self) -> USBPLL_DIV2_EN_W<20> {
        USBPLL_DIV2_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_clkout_en(&mut self) -> USBPLL_CLKOUT_EN_W<21> {
        USBPLL_CLKOUT_EN_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sel_sample_clk(&mut self) -> USBPLL_SEL_SAMPLE_CLK_W<24> {
        USBPLL_SEL_SAMPLE_CLK_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_rstb(&mut self) -> USBPLL_RSTB_W<28> {
        USBPLL_RSTB_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pu_usbpll_mmdiv(&mut self) -> PU_USBPLL_MMDIV_W<29> {
        PU_USBPLL_MMDIV_W::new(self)
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
#[doc = "`reset()` method sets wifi_pll_cfg10 to value 0x1130_0434"]
impl crate::Resettable for WIFI_PLL_CFG10_SPEC {
    const RESET_VALUE: Self::Ux = 0x1130_0434;
}
