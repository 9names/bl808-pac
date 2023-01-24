#[doc = "Register `cpu_pll_cfg9` reader"]
pub struct R(crate::R<CPU_PLL_CFG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg9` writer"]
pub struct W(crate::W<CPU_PLL_CFG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG9_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_dc_tp_out_en` reader - "]
pub type CPUPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_dc_tp_out_en` writer - "]
pub type CPUPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_cpupll` reader - "]
pub type TEN_CPUPLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_cpupll` writer - "]
pub type TEN_CPUPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `ten_cpupll_sfreg` reader - "]
pub type TEN_CPUPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `ten_cpupll_sfreg` writer - "]
pub type TEN_CPUPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_fin` reader - "]
pub type DTEN_CPUPLL_FIN_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_fin` writer - "]
pub type DTEN_CPUPLL_FIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_cpupll_fref` reader - "]
pub type DTEN_CPUPLL_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_fref` writer - "]
pub type DTEN_CPUPLL_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_cpupll_fsdm` reader - "]
pub type DTEN_CPUPLL_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_fsdm` writer - "]
pub type DTEN_CPUPLL_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_cpupll_div15` reader - "]
pub type DTEN_CPUPLL_DIV15_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_div15` writer - "]
pub type DTEN_CPUPLL_DIV15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `dten_cpupll_div5` reader - "]
pub type DTEN_CPUPLL_DIV5_R = crate::BitReader<bool>;
#[doc = "Field `dten_cpupll_div5` writer - "]
pub type DTEN_CPUPLL_DIV5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG9_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_dc_tp_out_en(&self) -> CPUPLL_DC_TP_OUT_EN_R {
        CPUPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_cpupll(&self) -> TEN_CPUPLL_R {
        TEN_CPUPLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_cpupll_sfreg(&self) -> TEN_CPUPLL_SFREG_R {
        TEN_CPUPLL_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_cpupll_fin(&self) -> DTEN_CPUPLL_FIN_R {
        DTEN_CPUPLL_FIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_cpupll_fref(&self) -> DTEN_CPUPLL_FREF_R {
        DTEN_CPUPLL_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_cpupll_fsdm(&self) -> DTEN_CPUPLL_FSDM_R {
        DTEN_CPUPLL_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_cpupll_div15(&self) -> DTEN_CPUPLL_DIV15_R {
        DTEN_CPUPLL_DIV15_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_cpupll_div5(&self) -> DTEN_CPUPLL_DIV5_R {
        DTEN_CPUPLL_DIV5_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_dc_tp_out_en(&mut self) -> CPUPLL_DC_TP_OUT_EN_W<0> {
        CPUPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ten_cpupll(&mut self) -> TEN_CPUPLL_W<1> {
        TEN_CPUPLL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_cpupll_sfreg(&mut self) -> TEN_CPUPLL_SFREG_W<2> {
        TEN_CPUPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_cpupll_fin(&mut self) -> DTEN_CPUPLL_FIN_W<4> {
        DTEN_CPUPLL_FIN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_cpupll_fref(&mut self) -> DTEN_CPUPLL_FREF_W<5> {
        DTEN_CPUPLL_FREF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten_cpupll_fsdm(&mut self) -> DTEN_CPUPLL_FSDM_W<6> {
        DTEN_CPUPLL_FSDM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dten_cpupll_div15(&mut self) -> DTEN_CPUPLL_DIV15_W<7> {
        DTEN_CPUPLL_DIV15_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dten_cpupll_div5(&mut self) -> DTEN_CPUPLL_DIV5_W<8> {
        DTEN_CPUPLL_DIV5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg9](index.html) module"]
pub struct CPU_PLL_CFG9_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg9::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg9::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg9 to value 0"]
impl crate::Resettable for CPU_PLL_CFG9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
