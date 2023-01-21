#[doc = "Register `audio_pll_cfg9` reader"]
pub struct R(crate::R<AUDIO_PLL_CFG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_PLL_CFG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_PLL_CFG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_PLL_CFG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_pll_cfg9` writer"]
pub struct W(crate::W<AUDIO_PLL_CFG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_PLL_CFG9_SPEC>;
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
impl From<crate::W<AUDIO_PLL_CFG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_PLL_CFG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aupll_dc_tp_out_en` reader - "]
pub type AUPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dc_tp_out_en` writer - "]
pub type AUPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_aupll` reader - "]
pub type TEN_AUPLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_aupll` writer - "]
pub type TEN_AUPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_aupll_sfreg` reader - "]
pub type TEN_AUPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `ten_aupll_sfreg` writer - "]
pub type TEN_AUPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_fin` reader - "]
pub type DTEN_AUPLL_FIN_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_fin` writer - "]
pub type DTEN_AUPLL_FIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_aupll_fref` reader - "]
pub type DTEN_AUPLL_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_fref` writer - "]
pub type DTEN_AUPLL_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_aupll_fsdm` reader - "]
pub type DTEN_AUPLL_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_fsdm` writer - "]
pub type DTEN_AUPLL_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_aupll_div15` reader - "]
pub type DTEN_AUPLL_DIV15_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_div15` writer - "]
pub type DTEN_AUPLL_DIV15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_aupll_div5` reader - "]
pub type DTEN_AUPLL_DIV5_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_div5` writer - "]
pub type DTEN_AUPLL_DIV5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_aupll_postdiv_clk` reader - "]
pub type DTEN_AUPLL_POSTDIV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `dten_aupll_postdiv_clk` writer - "]
pub type DTEN_AUPLL_POSTDIV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dtest_aupll_pulldown` reader - "]
pub type DTEST_AUPLL_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `dtest_aupll_pulldown` writer - "]
pub type DTEST_AUPLL_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_dc_tp_out_en(&self) -> AUPLL_DC_TP_OUT_EN_R {
        AUPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_aupll(&self) -> TEN_AUPLL_R {
        TEN_AUPLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_aupll_sfreg(&self) -> TEN_AUPLL_SFREG_R {
        TEN_AUPLL_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_aupll_fin(&self) -> DTEN_AUPLL_FIN_R {
        DTEN_AUPLL_FIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_aupll_fref(&self) -> DTEN_AUPLL_FREF_R {
        DTEN_AUPLL_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_aupll_fsdm(&self) -> DTEN_AUPLL_FSDM_R {
        DTEN_AUPLL_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_aupll_div15(&self) -> DTEN_AUPLL_DIV15_R {
        DTEN_AUPLL_DIV15_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_aupll_div5(&self) -> DTEN_AUPLL_DIV5_R {
        DTEN_AUPLL_DIV5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_aupll_postdiv_clk(&self) -> DTEN_AUPLL_POSTDIV_CLK_R {
        DTEN_AUPLL_POSTDIV_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dtest_aupll_pulldown(&self) -> DTEST_AUPLL_PULLDOWN_R {
        DTEST_AUPLL_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
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
    pub fn aupll_dc_tp_out_en(&mut self) -> AUPLL_DC_TP_OUT_EN_W<0> {
        AUPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ten_aupll(&mut self) -> TEN_AUPLL_W<1> {
        TEN_AUPLL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_aupll_sfreg(&mut self) -> TEN_AUPLL_SFREG_W<2> {
        TEN_AUPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_fin(&mut self) -> DTEN_AUPLL_FIN_W<4> {
        DTEN_AUPLL_FIN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_fref(&mut self) -> DTEN_AUPLL_FREF_W<5> {
        DTEN_AUPLL_FREF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_fsdm(&mut self) -> DTEN_AUPLL_FSDM_W<6> {
        DTEN_AUPLL_FSDM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_div15(&mut self) -> DTEN_AUPLL_DIV15_W<7> {
        DTEN_AUPLL_DIV15_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_div5(&mut self) -> DTEN_AUPLL_DIV5_W<8> {
        DTEN_AUPLL_DIV5_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dten_aupll_postdiv_clk(&mut self) -> DTEN_AUPLL_POSTDIV_CLK_W<9> {
        DTEN_AUPLL_POSTDIV_CLK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_aupll_pulldown(&mut self) -> DTEST_AUPLL_PULLDOWN_W<10> {
        DTEST_AUPLL_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_pll_cfg9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_pll_cfg9](index.html) module"]
pub struct AUDIO_PLL_CFG9_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_pll_cfg9::R](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_pll_cfg9::W](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg9 to value 0x0400"]
impl crate::Resettable for AUDIO_PLL_CFG9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
