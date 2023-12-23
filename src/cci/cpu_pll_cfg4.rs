#[doc = "Register `cpu_pll_cfg4` reader"]
pub struct R(crate::R<CPU_PLL_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg4` writer"]
pub struct W(crate::W<CPU_PLL_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG4_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_sel_sample_clk` reader - "]
pub type CPUPLL_SEL_SAMPLE_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_sel_sample_clk` writer - "]
pub type CPUPLL_SEL_SAMPLE_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_sel_fb_clk` reader - "]
pub type CPUPLL_SEL_FB_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_sel_fb_clk` writer - "]
pub type CPUPLL_SEL_FB_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_sdmclk_sel` reader - "]
pub type CPUPLL_SDMCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_sdmclk_sel` writer - "]
pub type CPUPLL_SDMCLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG4_SPEC, bool, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpupll_sel_sample_clk(&self) -> CPUPLL_SEL_SAMPLE_CLK_R {
        CPUPLL_SEL_SAMPLE_CLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_sel_fb_clk(&self) -> CPUPLL_SEL_FB_CLK_R {
        CPUPLL_SEL_FB_CLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_sdmclk_sel(&self) -> CPUPLL_SDMCLK_SEL_R {
        CPUPLL_SDMCLK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_sel_sample_clk(&mut self) -> CPUPLL_SEL_SAMPLE_CLK_W<0> {
        CPUPLL_SEL_SAMPLE_CLK_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_sel_fb_clk(&mut self) -> CPUPLL_SEL_FB_CLK_W<4> {
        CPUPLL_SEL_FB_CLK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_sdmclk_sel(&mut self) -> CPUPLL_SDMCLK_SEL_W<8> {
        CPUPLL_SDMCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg4](index.html) module"]
pub struct CPU_PLL_CFG4_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg4::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg4::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg4 to value 0x0111"]
impl crate::Resettable for CPU_PLL_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0111;
}
