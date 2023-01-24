#[doc = "Register `uhs_pll_cfg5` reader"]
pub struct R(crate::R<UHS_PLL_CFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg5` writer"]
pub struct W(crate::W<UHS_PLL_CFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG5_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_vco_speed` reader - "]
pub type UHSPLL_VCO_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vco_speed` writer - "]
pub type UHSPLL_VCO_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG5_SPEC, u8, u8, 3, O>;
#[doc = "Field `uhspll_vco_vdd_ctrl` reader - "]
pub type UHSPLL_VCO_VDD_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vco_vdd_ctrl` writer - "]
pub type UHSPLL_VCO_VDD_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG5_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhspll_vco_speed(&self) -> UHSPLL_VCO_SPEED_R {
        UHSPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn uhspll_vco_vdd_ctrl(&self) -> UHSPLL_VCO_VDD_CTRL_R {
        UHSPLL_VCO_VDD_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_vco_speed(&mut self) -> UHSPLL_VCO_SPEED_W<0> {
        UHSPLL_VCO_SPEED_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_vco_vdd_ctrl(&mut self) -> UHSPLL_VCO_VDD_CTRL_W<3> {
        UHSPLL_VCO_VDD_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg5](index.html) module"]
pub struct UHS_PLL_CFG5_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg5::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg5::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg5 to value 0x1f"]
impl crate::Resettable for UHS_PLL_CFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
