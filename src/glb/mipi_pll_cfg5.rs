#[doc = "Register `mipi_pll_cfg5` reader"]
pub struct R(crate::R<MIPI_PLL_CFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_PLL_CFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_PLL_CFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_PLL_CFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mipi_pll_cfg5` writer"]
pub struct W(crate::W<MIPI_PLL_CFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_PLL_CFG5_SPEC>;
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
impl From<crate::W<MIPI_PLL_CFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_PLL_CFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mipipll_vco_speed` reader - "]
pub type MIPIPLL_VCO_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_vco_speed` writer - "]
pub type MIPIPLL_VCO_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG5_SPEC, u8, u8, 3, O>;
#[doc = "Field `mipipll_vco_vdd_ctrl` reader - "]
pub type MIPIPLL_VCO_VDD_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_vco_vdd_ctrl` writer - "]
pub type MIPIPLL_VCO_VDD_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG5_SPEC, u8, u8, 2, O>;
#[doc = "Field `mipipll_vco_vdd_ctrl_extra` reader - "]
pub type MIPIPLL_VCO_VDD_CTRL_EXTRA_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_vco_vdd_ctrl_extra` writer - "]
pub type MIPIPLL_VCO_VDD_CTRL_EXTRA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG5_SPEC, bool, O>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_vco_postdiv_sel` reader - "]
pub type MIPIPLL_VCO_POSTDIV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_vco_postdiv_sel` writer - "]
pub type MIPIPLL_VCO_POSTDIV_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG5_SPEC, u8, u8, 3, O>;
#[doc = "Field `mipipll_vco_postdiv_clk_en` reader - "]
pub type MIPIPLL_VCO_POSTDIV_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_vco_postdiv_clk_en` writer - "]
pub type MIPIPLL_VCO_POSTDIV_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG5_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mipipll_vco_speed(&self) -> MIPIPLL_VCO_SPEED_R {
        MIPIPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl(&self) -> MIPIPLL_VCO_VDD_CTRL_R {
        MIPIPLL_VCO_VDD_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl_extra(&self) -> MIPIPLL_VCO_VDD_CTRL_EXTRA_R {
        MIPIPLL_VCO_VDD_CTRL_EXTRA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_sel(&self) -> MIPIPLL_VCO_POSTDIV_SEL_R {
        MIPIPLL_VCO_POSTDIV_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_clk_en(&self) -> MIPIPLL_VCO_POSTDIV_CLK_EN_R {
        MIPIPLL_VCO_POSTDIV_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_vco_speed(&mut self) -> MIPIPLL_VCO_SPEED_W<0> {
        MIPIPLL_VCO_SPEED_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_vco_vdd_ctrl(&mut self) -> MIPIPLL_VCO_VDD_CTRL_W<3> {
        MIPIPLL_VCO_VDD_CTRL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_vco_vdd_ctrl_extra(&mut self) -> MIPIPLL_VCO_VDD_CTRL_EXTRA_W<5> {
        MIPIPLL_VCO_VDD_CTRL_EXTRA_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_vco_postdiv_sel(&mut self) -> MIPIPLL_VCO_POSTDIV_SEL_W<7> {
        MIPIPLL_VCO_POSTDIV_SEL_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_vco_postdiv_clk_en(&mut self) -> MIPIPLL_VCO_POSTDIV_CLK_EN_W<10> {
        MIPIPLL_VCO_POSTDIV_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mipi_pll_cfg5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_pll_cfg5](index.html) module"]
pub struct MIPI_PLL_CFG5_SPEC;
impl crate::RegisterSpec for MIPI_PLL_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_pll_cfg5::R](R) reader structure"]
impl crate::Readable for MIPI_PLL_CFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_pll_cfg5::W](W) writer structure"]
impl crate::Writable for MIPI_PLL_CFG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mipi_pll_cfg5 to value 0x0415"]
impl crate::Resettable for MIPI_PLL_CFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0415;
}
