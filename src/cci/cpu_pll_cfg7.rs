#[doc = "Register `cpu_pll_cfg7` reader"]
pub struct R(crate::R<CPU_PLL_CFG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg7` writer"]
pub struct W(crate::W<CPU_PLL_CFG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG7_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_sdm_order_sel` reader - "]
pub type CPUPLL_SDM_ORDER_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_sdm_order_sel` writer - "]
pub type CPUPLL_SDM_ORDER_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG7_SPEC, bool, O>;
#[doc = "Field `reserved_1_15` reader - "]
pub type RESERVED_1_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cpupll_sdm_sig_dith_sel` reader - "]
pub type CPUPLL_SDM_SIG_DITH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_sdm_sig_dith_sel` writer - "]
pub type CPUPLL_SDM_SIG_DITH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG7_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_31` reader - "]
pub type RESERVED_18_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_order_sel(&self) -> CPUPLL_SDM_ORDER_SEL_R {
        CPUPLL_SDM_ORDER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn reserved_1_15(&self) -> RESERVED_1_15_R {
        RESERVED_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_sdm_sig_dith_sel(&self) -> CPUPLL_SDM_SIG_DITH_SEL_R {
        CPUPLL_SDM_SIG_DITH_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn reserved_18_31(&self) -> RESERVED_18_31_R {
        RESERVED_18_31_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_sdm_order_sel(&mut self) -> CPUPLL_SDM_ORDER_SEL_W<0> {
        CPUPLL_SDM_ORDER_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_sdm_sig_dith_sel(&mut self) -> CPUPLL_SDM_SIG_DITH_SEL_W<16> {
        CPUPLL_SDM_SIG_DITH_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg7](index.html) module"]
pub struct CPU_PLL_CFG7_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg7::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg7::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg7 to value 0x01"]
impl crate::Resettable for CPU_PLL_CFG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
