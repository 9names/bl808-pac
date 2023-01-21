#[doc = "Register `mipi_pll_cfg7` reader"]
pub struct R(crate::R<MIPI_PLL_CFG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_PLL_CFG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_PLL_CFG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_PLL_CFG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mipi_pll_cfg7` writer"]
pub struct W(crate::W<MIPI_PLL_CFG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_PLL_CFG7_SPEC>;
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
impl From<crate::W<MIPI_PLL_CFG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_PLL_CFG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mipipll_sdm_order_sel` reader - "]
pub type MIPIPLL_SDM_ORDER_SEL_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_sdm_order_sel` writer - "]
pub type MIPIPLL_SDM_ORDER_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG7_SPEC, bool, O>;
#[doc = "Field `mipipll_sdm_dith_sel` reader - "]
pub type MIPIPLL_SDM_DITH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_sdm_dith_sel` writer - "]
pub type MIPIPLL_SDM_DITH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG7_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_3_31` reader - "]
pub type RESERVED_3_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_order_sel(&self) -> MIPIPLL_SDM_ORDER_SEL_R {
        MIPIPLL_SDM_ORDER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mipipll_sdm_dith_sel(&self) -> MIPIPLL_SDM_DITH_SEL_R {
        MIPIPLL_SDM_DITH_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn reserved_3_31(&self) -> RESERVED_3_31_R {
        RESERVED_3_31_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_sdm_order_sel(&mut self) -> MIPIPLL_SDM_ORDER_SEL_W<0> {
        MIPIPLL_SDM_ORDER_SEL_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_sdm_dith_sel(&mut self) -> MIPIPLL_SDM_DITH_SEL_W<1> {
        MIPIPLL_SDM_DITH_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mipi_pll_cfg7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_pll_cfg7](index.html) module"]
pub struct MIPI_PLL_CFG7_SPEC;
impl crate::RegisterSpec for MIPI_PLL_CFG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_pll_cfg7::R](R) reader structure"]
impl crate::Readable for MIPI_PLL_CFG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_pll_cfg7::W](W) writer structure"]
impl crate::Writable for MIPI_PLL_CFG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mipi_pll_cfg7 to value 0x01"]
impl crate::Resettable for MIPI_PLL_CFG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
