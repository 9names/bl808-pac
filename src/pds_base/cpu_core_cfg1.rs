#[doc = "Register `cpu_core_cfg1` reader"]
pub struct R(crate::R<CPU_CORE_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CORE_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CORE_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CORE_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_core_cfg1` writer"]
pub struct W(crate::W<CPU_CORE_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CORE_CFG1_SPEC>;
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
impl From<crate::W<CPU_CORE_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CORE_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_3` reader - "]
pub type RESERVED_0_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pll_sel` reader - "]
pub type REG_PLL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pll_sel` writer - "]
pub type REG_PLL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CORE_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mcu1_clk_en` reader - "]
pub type REG_MCU1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_clk_en` writer - "]
pub type REG_MCU1_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CORE_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reserved_0_3(&self) -> RESERVED_0_3_R {
        RESERVED_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> REG_PLL_SEL_R {
        REG_PLL_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu1_clk_en(&self) -> REG_MCU1_CLK_EN_R {
        REG_MCU1_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pll_sel(&mut self) -> REG_PLL_SEL_W<4> {
        REG_PLL_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_clk_en(&mut self) -> REG_MCU1_CLK_EN_W<8> {
        REG_MCU1_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_core_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_core_cfg1](index.html) module"]
pub struct CPU_CORE_CFG1_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_core_cfg1::R](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_core_cfg1::W](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg1 to value 0x0130"]
impl crate::Resettable for CPU_CORE_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0130;
}
