#[doc = "Register `cpu_pll_cfg1` reader"]
pub struct R(crate::R<CPU_PLL_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg1` writer"]
pub struct W(crate::W<CPU_PLL_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG1_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_postdiv` reader - "]
pub type CPUPLL_POSTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_postdiv` writer - "]
pub type CPUPLL_POSTDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG1_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_refdiv_ratio` reader - "]
pub type CPUPLL_REFDIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_refdiv_ratio` writer - "]
pub type CPUPLL_REFDIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_refclk_sel` reader - "]
pub type CPUPLL_REFCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_refclk_sel` writer - "]
pub type CPUPLL_REFCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_19` reader - "]
pub type RESERVED_18_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_vg11_sel` reader - "]
pub type CPUPLL_VG11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_vg11_sel` writer - "]
pub type CPUPLL_VG11_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_22_23` reader - "]
pub type RESERVED_22_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_vg13_sel` reader - "]
pub type CPUPLL_VG13_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_vg13_sel` writer - "]
pub type CPUPLL_VG13_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn cpupll_postdiv(&self) -> CPUPLL_POSTDIV_R {
        CPUPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cpupll_refdiv_ratio(&self) -> CPUPLL_REFDIV_RATIO_R {
        CPUPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_refclk_sel(&self) -> CPUPLL_REFCLK_SEL_R {
        CPUPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reserved_18_19(&self) -> RESERVED_18_19_R {
        RESERVED_18_19_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn cpupll_vg11_sel(&self) -> CPUPLL_VG11_SEL_R {
        CPUPLL_VG11_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reserved_22_23(&self) -> RESERVED_22_23_R {
        RESERVED_22_23_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cpupll_vg13_sel(&self) -> CPUPLL_VG13_SEL_R {
        CPUPLL_VG13_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_postdiv(&mut self) -> CPUPLL_POSTDIV_W<0> {
        CPUPLL_POSTDIV_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_refdiv_ratio(&mut self) -> CPUPLL_REFDIV_RATIO_W<8> {
        CPUPLL_REFDIV_RATIO_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_refclk_sel(&mut self) -> CPUPLL_REFCLK_SEL_W<16> {
        CPUPLL_REFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_vg11_sel(&mut self) -> CPUPLL_VG11_SEL_W<20> {
        CPUPLL_VG11_SEL_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_vg13_sel(&mut self) -> CPUPLL_VG13_SEL_W<24> {
        CPUPLL_VG13_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg1](index.html) module"]
pub struct CPU_PLL_CFG1_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg1::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg1::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg1 to value 0x0110_0418"]
impl crate::Resettable for CPU_PLL_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0418;
}
