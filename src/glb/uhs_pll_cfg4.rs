#[doc = "Register `uhs_pll_cfg4` reader"]
pub struct R(crate::R<UHS_PLL_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg4` writer"]
pub struct W(crate::W<UHS_PLL_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG4_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_sel_sample_clk` reader - "]
pub type UHSPLL_SEL_SAMPLE_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_sel_sample_clk` writer - "]
pub type UHSPLL_SEL_SAMPLE_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_sel_fb_clk` reader - "]
pub type UHSPLL_SEL_FB_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_sel_fb_clk` writer - "]
pub type UHSPLL_SEL_FB_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_lock_det_en` reader - "]
pub type UHSPLL_LOCK_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_lock_det_en` writer - "]
pub type UHSPLL_LOCK_DET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG4_SPEC, bool, O>;
#[doc = "Field `uhspll_lock_clk_sel` reader - "]
pub type UHSPLL_LOCK_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_lock_clk_sel` writer - "]
pub type UHSPLL_LOCK_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_lock_clk_inv_en` reader - "]
pub type UHSPLL_LOCK_CLK_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_lock_clk_inv_en` writer - "]
pub type UHSPLL_LOCK_CLK_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG4_SPEC, bool, O>;
#[doc = "Field `reserved_13_14` reader - "]
pub type RESERVED_13_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_lock_win_sel` reader - "]
pub type UHSPLL_LOCK_WIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_lock_win_sel` writer - "]
pub type UHSPLL_LOCK_WIN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn uhspll_sel_sample_clk(&self) -> UHSPLL_SEL_SAMPLE_CLK_R {
        UHSPLL_SEL_SAMPLE_CLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_sel_fb_clk(&self) -> UHSPLL_SEL_FB_CLK_R {
        UHSPLL_SEL_FB_CLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_lock_det_en(&self) -> UHSPLL_LOCK_DET_EN_R {
        UHSPLL_LOCK_DET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn uhspll_lock_clk_sel(&self) -> UHSPLL_LOCK_CLK_SEL_R {
        UHSPLL_LOCK_CLK_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhspll_lock_clk_inv_en(&self) -> UHSPLL_LOCK_CLK_INV_EN_R {
        UHSPLL_LOCK_CLK_INV_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn reserved_13_14(&self) -> RESERVED_13_14_R {
        RESERVED_13_14_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn uhspll_lock_win_sel(&self) -> UHSPLL_LOCK_WIN_SEL_R {
        UHSPLL_LOCK_WIN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_sel_sample_clk(&mut self) -> UHSPLL_SEL_SAMPLE_CLK_W<0> {
        UHSPLL_SEL_SAMPLE_CLK_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_sel_fb_clk(&mut self) -> UHSPLL_SEL_FB_CLK_W<4> {
        UHSPLL_SEL_FB_CLK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_lock_det_en(&mut self) -> UHSPLL_LOCK_DET_EN_W<8> {
        UHSPLL_LOCK_DET_EN_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_lock_clk_sel(&mut self) -> UHSPLL_LOCK_CLK_SEL_W<9> {
        UHSPLL_LOCK_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_lock_clk_inv_en(&mut self) -> UHSPLL_LOCK_CLK_INV_EN_W<12> {
        UHSPLL_LOCK_CLK_INV_EN_W::new(self)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_lock_win_sel(&mut self) -> UHSPLL_LOCK_WIN_SEL_W<15> {
        UHSPLL_LOCK_WIN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg4](index.html) module"]
pub struct UHS_PLL_CFG4_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg4::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg4::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg4 to value 0x8311"]
impl crate::Resettable for UHS_PLL_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x8311;
}
