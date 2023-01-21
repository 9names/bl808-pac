#[doc = "Register `cpu_pll_cfg2` reader"]
pub struct R(crate::R<CPU_PLL_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg2` writer"]
pub struct W(crate::W<CPU_PLL_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG2_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_sel_cp_bias` reader - "]
pub type CPUPLL_SEL_CP_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_sel_cp_bias` writer - "]
pub type CPUPLL_SEL_CP_BIAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_icp_5u` reader - "]
pub type CPUPLL_ICP_5U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_icp_5u` writer - "]
pub type CPUPLL_ICP_5U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `cpupll_icp_1u` reader - "]
pub type CPUPLL_ICP_1U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_icp_1u` writer - "]
pub type CPUPLL_ICP_1U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `cpupll_int_frac_sw` reader - "]
pub type CPUPLL_INT_FRAC_SW_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_int_frac_sw` writer - "]
pub type CPUPLL_INT_FRAC_SW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `cpupll_cp_startup_en` reader - "]
pub type CPUPLL_CP_STARTUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_cp_startup_en` writer - "]
pub type CPUPLL_CP_STARTUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `cpupll_cp_opamp_en` reader - "]
pub type CPUPLL_CP_OPAMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_cp_opamp_en` writer - "]
pub type CPUPLL_CP_OPAMP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sel_cp_bias(&self) -> CPUPLL_SEL_CP_BIAS_R {
        CPUPLL_SEL_CP_BIAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_icp_5u(&self) -> CPUPLL_ICP_5U_R {
        CPUPLL_ICP_5U_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cpupll_icp_1u(&self) -> CPUPLL_ICP_1U_R {
        CPUPLL_ICP_1U_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_int_frac_sw(&self) -> CPUPLL_INT_FRAC_SW_R {
        CPUPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpupll_cp_startup_en(&self) -> CPUPLL_CP_STARTUP_EN_R {
        CPUPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cpupll_cp_opamp_en(&self) -> CPUPLL_CP_OPAMP_EN_R {
        CPUPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 1) != 0)
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
    pub fn cpupll_sel_cp_bias(&mut self) -> CPUPLL_SEL_CP_BIAS_W<0> {
        CPUPLL_SEL_CP_BIAS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_icp_5u(&mut self) -> CPUPLL_ICP_5U_W<4> {
        CPUPLL_ICP_5U_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_icp_1u(&mut self) -> CPUPLL_ICP_1U_W<6> {
        CPUPLL_ICP_1U_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_int_frac_sw(&mut self) -> CPUPLL_INT_FRAC_SW_W<8> {
        CPUPLL_INT_FRAC_SW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_cp_startup_en(&mut self) -> CPUPLL_CP_STARTUP_EN_W<9> {
        CPUPLL_CP_STARTUP_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_cp_opamp_en(&mut self) -> CPUPLL_CP_OPAMP_EN_W<10> {
        CPUPLL_CP_OPAMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg2](index.html) module"]
pub struct CPU_PLL_CFG2_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg2::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg2::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg2 to value 0x0741"]
impl crate::Resettable for CPU_PLL_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0741;
}
