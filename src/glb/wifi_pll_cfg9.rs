#[doc = "Register `wifi_pll_cfg9` reader"]
pub struct R(crate::R<WIFI_PLL_CFG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg9` writer"]
pub struct W(crate::W<WIFI_PLL_CFG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG9_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_dc_tp_out_en` reader - "]
pub type WIFIPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dc_tp_out_en` writer - "]
pub type WIFIPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_wifipll` reader - "]
pub type TEN_WIFIPLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_wifipll` writer - "]
pub type TEN_WIFIPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_wifipll_sfreg` reader - "]
pub type TEN_WIFIPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `ten_wifipll_sfreg` writer - "]
pub type TEN_WIFIPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_fin` reader - "]
pub type DTEN_WIFIPLL_FIN_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_fin` writer - "]
pub type DTEN_WIFIPLL_FIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_wifipll_fref` reader - "]
pub type DTEN_WIFIPLL_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_fref` writer - "]
pub type DTEN_WIFIPLL_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_wifipll_fsdm` reader - "]
pub type DTEN_WIFIPLL_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_fsdm` writer - "]
pub type DTEN_WIFIPLL_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_wifipll_div30` reader - "]
pub type DTEN_WIFIPLL_DIV30_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_div30` writer - "]
pub type DTEN_WIFIPLL_DIV30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_wifipll_div10` reader - "]
pub type DTEN_WIFIPLL_DIV10_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_div10` writer - "]
pub type DTEN_WIFIPLL_DIV10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_wifipll_postdiv_clk` reader - "]
pub type DTEN_WIFIPLL_POSTDIV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `dten_wifipll_postdiv_clk` writer - "]
pub type DTEN_WIFIPLL_POSTDIV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `usbpll_dtest_pclk_en` reader - "]
pub type USBPLL_DTEST_PCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_dtest_pclk_en` writer - "]
pub type USBPLL_DTEST_PCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `usbpll_dtest_clkout_en` reader - "]
pub type USBPLL_DTEST_CLKOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_dtest_clkout_en` writer - "]
pub type USBPLL_DTEST_CLKOUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dtest_wifipll_pulldown` reader - "]
pub type DTEST_WIFIPLL_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `dtest_wifipll_pulldown` writer - "]
pub type DTEST_WIFIPLL_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_13_31` reader - "]
pub type RESERVED_13_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_dc_tp_out_en(&self) -> WIFIPLL_DC_TP_OUT_EN_R {
        WIFIPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_wifipll(&self) -> TEN_WIFIPLL_R {
        TEN_WIFIPLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_wifipll_sfreg(&self) -> TEN_WIFIPLL_SFREG_R {
        TEN_WIFIPLL_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_wifipll_fin(&self) -> DTEN_WIFIPLL_FIN_R {
        DTEN_WIFIPLL_FIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_wifipll_fref(&self) -> DTEN_WIFIPLL_FREF_R {
        DTEN_WIFIPLL_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_wifipll_fsdm(&self) -> DTEN_WIFIPLL_FSDM_R {
        DTEN_WIFIPLL_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_wifipll_div30(&self) -> DTEN_WIFIPLL_DIV30_R {
        DTEN_WIFIPLL_DIV30_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_wifipll_div10(&self) -> DTEN_WIFIPLL_DIV10_R {
        DTEN_WIFIPLL_DIV10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_wifipll_postdiv_clk(&self) -> DTEN_WIFIPLL_POSTDIV_CLK_R {
        DTEN_WIFIPLL_POSTDIV_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_dtest_pclk_en(&self) -> USBPLL_DTEST_PCLK_EN_R {
        USBPLL_DTEST_PCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbpll_dtest_clkout_en(&self) -> USBPLL_DTEST_CLKOUT_EN_R {
        USBPLL_DTEST_CLKOUT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dtest_wifipll_pulldown(&self) -> DTEST_WIFIPLL_PULLDOWN_R {
        DTEST_WIFIPLL_PULLDOWN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn reserved_13_31(&self) -> RESERVED_13_31_R {
        RESERVED_13_31_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dc_tp_out_en(&mut self) -> WIFIPLL_DC_TP_OUT_EN_W<0> {
        WIFIPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ten_wifipll(&mut self) -> TEN_WIFIPLL_W<1> {
        TEN_WIFIPLL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_wifipll_sfreg(&mut self) -> TEN_WIFIPLL_SFREG_W<2> {
        TEN_WIFIPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fin(&mut self) -> DTEN_WIFIPLL_FIN_W<4> {
        DTEN_WIFIPLL_FIN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fref(&mut self) -> DTEN_WIFIPLL_FREF_W<5> {
        DTEN_WIFIPLL_FREF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fsdm(&mut self) -> DTEN_WIFIPLL_FSDM_W<6> {
        DTEN_WIFIPLL_FSDM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_div30(&mut self) -> DTEN_WIFIPLL_DIV30_W<7> {
        DTEN_WIFIPLL_DIV30_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_div10(&mut self) -> DTEN_WIFIPLL_DIV10_W<8> {
        DTEN_WIFIPLL_DIV10_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_postdiv_clk(&mut self) -> DTEN_WIFIPLL_POSTDIV_CLK_W<9> {
        DTEN_WIFIPLL_POSTDIV_CLK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_dtest_pclk_en(&mut self) -> USBPLL_DTEST_PCLK_EN_W<10> {
        USBPLL_DTEST_PCLK_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_dtest_clkout_en(&mut self) -> USBPLL_DTEST_CLKOUT_EN_W<11> {
        USBPLL_DTEST_CLKOUT_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_wifipll_pulldown(&mut self) -> DTEST_WIFIPLL_PULLDOWN_W<12> {
        DTEST_WIFIPLL_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg9](index.html) module"]
pub struct WIFI_PLL_CFG9_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg9::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg9::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg9 to value 0x1000"]
impl crate::Resettable for WIFI_PLL_CFG9_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
