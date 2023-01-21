#[doc = "Register `cpu_pll_cfg5` reader"]
pub struct R(crate::R<CPU_PLL_CFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg5` writer"]
pub struct W(crate::W<CPU_PLL_CFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG5_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_vco_speed` reader - "]
pub type CPUPLL_VCO_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_vco_speed` writer - "]
pub type CPUPLL_VCO_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG5_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3_31` reader - "]
pub type RESERVED_3_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cpupll_vco_speed(&self) -> CPUPLL_VCO_SPEED_R {
        CPUPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn reserved_3_31(&self) -> RESERVED_3_31_R {
        RESERVED_3_31_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_vco_speed(&mut self) -> CPUPLL_VCO_SPEED_W<0> {
        CPUPLL_VCO_SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg5](index.html) module"]
pub struct CPU_PLL_CFG5_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg5::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg5::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg5 to value 0x03"]
impl crate::Resettable for CPU_PLL_CFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
