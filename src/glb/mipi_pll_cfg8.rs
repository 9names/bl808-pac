#[doc = "Register `mipi_pll_cfg8` reader"]
pub struct R(crate::R<MIPI_PLL_CFG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_PLL_CFG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_PLL_CFG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_PLL_CFG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mipi_pll_cfg8` writer"]
pub struct W(crate::W<MIPI_PLL_CFG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_PLL_CFG8_SPEC>;
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
impl From<crate::W<MIPI_PLL_CFG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_PLL_CFG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mipipll_dc_tp_out_en` reader - "]
pub type MIPIPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dc_tp_out_en` writer - "]
pub type MIPIPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `mipipll_ten` reader - "]
pub type MIPIPLL_TEN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_ten` writer - "]
pub type MIPIPLL_TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `mipipll_ten_sfreg` reader - "]
pub type MIPIPLL_TEN_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_ten_sfreg` writer - "]
pub type MIPIPLL_TEN_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dten_ckin` reader - "]
pub type MIPIPLL_DTEN_CKIN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dten_ckin` writer - "]
pub type MIPIPLL_DTEN_CKIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `mipipll_dten_fref` reader - "]
pub type MIPIPLL_DTEN_FREF_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dten_fref` writer - "]
pub type MIPIPLL_DTEN_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `mipipll_dten_fsdm` reader - "]
pub type MIPIPLL_DTEN_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dten_fsdm` writer - "]
pub type MIPIPLL_DTEN_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `mipipll_dten_pupll` reader - "]
pub type MIPIPLL_DTEN_PUPLL_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_dten_pupll` writer - "]
pub type MIPIPLL_DTEN_PUPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_dc_tp_out_en(&self) -> MIPIPLL_DC_TP_OUT_EN_R {
        MIPIPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mipipll_ten(&self) -> MIPIPLL_TEN_R {
        MIPIPLL_TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_ten_sfreg(&self) -> MIPIPLL_TEN_SFREG_R {
        MIPIPLL_TEN_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mipipll_dten_ckin(&self) -> MIPIPLL_DTEN_CKIN_R {
        MIPIPLL_DTEN_CKIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_dten_fref(&self) -> MIPIPLL_DTEN_FREF_R {
        MIPIPLL_DTEN_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mipipll_dten_fsdm(&self) -> MIPIPLL_DTEN_FSDM_R {
        MIPIPLL_DTEN_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mipipll_dten_pupll(&self) -> MIPIPLL_DTEN_PUPLL_R {
        MIPIPLL_DTEN_PUPLL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_dc_tp_out_en(&mut self) -> MIPIPLL_DC_TP_OUT_EN_W<0> {
        MIPIPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ten(&mut self) -> MIPIPLL_TEN_W<1> {
        MIPIPLL_TEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ten_sfreg(&mut self) -> MIPIPLL_TEN_SFREG_W<2> {
        MIPIPLL_TEN_SFREG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_dten_ckin(&mut self) -> MIPIPLL_DTEN_CKIN_W<4> {
        MIPIPLL_DTEN_CKIN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_dten_fref(&mut self) -> MIPIPLL_DTEN_FREF_W<5> {
        MIPIPLL_DTEN_FREF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_dten_fsdm(&mut self) -> MIPIPLL_DTEN_FSDM_W<6> {
        MIPIPLL_DTEN_FSDM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_dten_pupll(&mut self) -> MIPIPLL_DTEN_PUPLL_W<7> {
        MIPIPLL_DTEN_PUPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mipi_pll_cfg8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_pll_cfg8](index.html) module"]
pub struct MIPI_PLL_CFG8_SPEC;
impl crate::RegisterSpec for MIPI_PLL_CFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_pll_cfg8::R](R) reader structure"]
impl crate::Readable for MIPI_PLL_CFG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_pll_cfg8::W](W) writer structure"]
impl crate::Writable for MIPI_PLL_CFG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mipi_pll_cfg8 to value 0"]
impl crate::Resettable for MIPI_PLL_CFG8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}