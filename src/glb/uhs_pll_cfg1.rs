#[doc = "Register `uhs_pll_cfg1` reader"]
pub struct R(crate::R<UHS_PLL_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg1` writer"]
pub struct W(crate::W<UHS_PLL_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG1_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_even_div_ratio` reader - "]
pub type UHSPLL_EVEN_DIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_even_div_ratio` writer - "]
pub type UHSPLL_EVEN_DIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG1_SPEC, u8, u8, 7, O>;
#[doc = "Field `uhspll_even_div_en` reader - "]
pub type UHSPLL_EVEN_DIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_even_div_en` writer - "]
pub type UHSPLL_EVEN_DIV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG1_SPEC, bool, O>;
#[doc = "Field `uhspll_refdiv_ratio` reader - "]
pub type UHSPLL_REFDIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_refdiv_ratio` writer - "]
pub type UHSPLL_REFDIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_refclk_sel` reader - "]
pub type UHSPLL_REFCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_refclk_sel` writer - "]
pub type UHSPLL_REFCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_19` reader - "]
pub type RESERVED_18_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vg11_sel` reader - "]
pub type UHSPLL_VG11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vg11_sel` writer - "]
pub type UHSPLL_VG11_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_22_23` reader - "]
pub type RESERVED_22_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vg13_sel` reader - "]
pub type UHSPLL_VG13_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_vg13_sel` writer - "]
pub type UHSPLL_VG13_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn uhspll_even_div_ratio(&self) -> UHSPLL_EVEN_DIV_RATIO_R {
        UHSPLL_EVEN_DIV_RATIO_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_even_div_en(&self) -> UHSPLL_EVEN_DIV_EN_R {
        UHSPLL_EVEN_DIV_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uhspll_refdiv_ratio(&self) -> UHSPLL_REFDIV_RATIO_R {
        UHSPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn uhspll_refclk_sel(&self) -> UHSPLL_REFCLK_SEL_R {
        UHSPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reserved_18_19(&self) -> RESERVED_18_19_R {
        RESERVED_18_19_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn uhspll_vg11_sel(&self) -> UHSPLL_VG11_SEL_R {
        UHSPLL_VG11_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reserved_22_23(&self) -> RESERVED_22_23_R {
        RESERVED_22_23_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn uhspll_vg13_sel(&self) -> UHSPLL_VG13_SEL_R {
        UHSPLL_VG13_SEL_R::new(((self.bits >> 24) & 3) as u8)
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
    pub fn uhspll_even_div_ratio(&mut self) -> UHSPLL_EVEN_DIV_RATIO_W<0> {
        UHSPLL_EVEN_DIV_RATIO_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_even_div_en(&mut self) -> UHSPLL_EVEN_DIV_EN_W<7> {
        UHSPLL_EVEN_DIV_EN_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_refdiv_ratio(&mut self) -> UHSPLL_REFDIV_RATIO_W<8> {
        UHSPLL_REFDIV_RATIO_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_refclk_sel(&mut self) -> UHSPLL_REFCLK_SEL_W<16> {
        UHSPLL_REFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_vg11_sel(&mut self) -> UHSPLL_VG11_SEL_W<20> {
        UHSPLL_VG11_SEL_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_vg13_sel(&mut self) -> UHSPLL_VG13_SEL_W<24> {
        UHSPLL_VG13_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg1](index.html) module"]
pub struct UHS_PLL_CFG1_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg1::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg1::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg1 to value 0x0110_0254"]
impl crate::Resettable for UHS_PLL_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0254;
}
