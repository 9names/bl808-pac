#[doc = "Register `touch1` reader"]
pub struct R(crate::R<TOUCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch1` writer"]
pub struct W(crate::W<TOUCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH1_SPEC>;
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
impl From<crate::W<TOUCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_vref_sel` reader - "]
pub type TOUCH_VREF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vref_sel` writer - "]
pub type TOUCH_VREF_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUCH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_vldo_sel` reader - "]
pub type TOUCH_VLDO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vldo_sel` writer - "]
pub type TOUCH_VLDO_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUCH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_comp_hys_sel` reader - "]
pub type TOUCH_COMP_HYS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `touch_comp_hys_sel` writer - "]
pub type TOUCH_COMP_HYS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `touch_current_sel` reader - "]
pub type TOUCH_CURRENT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `touch_current_sel` writer - "]
pub type TOUCH_CURRENT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `reserved_8_15` reader - "]
pub type RESERVED_8_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_clk_sel` reader - "]
pub type TOUCH_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `touch_clk_sel` writer - "]
pub type TOUCH_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `touch_clk_div_ratio` reader - "]
pub type TOUCH_CLK_DIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_clk_div_ratio` writer - "]
pub type TOUCH_CLK_DIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_pcharge_high` reader - "]
pub type TOUCH_PCHARGE_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_pcharge_high` writer - "]
pub type TOUCH_PCHARGE_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_pcharge_low` reader - "]
pub type TOUCH_PCHARGE_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_pcharge_low` writer - "]
pub type TOUCH_PCHARGE_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH1_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_cont_en` reader - "]
pub type TOUCH_CONT_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_cont_en` writer - "]
pub type TOUCH_CONT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `touch_cycle_en` reader - "]
pub type TOUCH_CYCLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_cycle_en` writer - "]
pub type TOUCH_CYCLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `touch_ulp_en` reader - "]
pub type TOUCH_ULP_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_ulp_en` writer - "]
pub type TOUCH_ULP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `reserved_29` reader - "]
pub type RESERVED_29_R = crate::BitReader<bool>;
#[doc = "Field `pu_touch` reader - "]
pub type PU_TOUCH_R = crate::BitReader<bool>;
#[doc = "Field `pu_touch` writer - "]
pub type PU_TOUCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH1_SPEC, bool, O>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn touch_vref_sel(&self) -> TOUCH_VREF_SEL_R {
        TOUCH_VREF_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn touch_vldo_sel(&self) -> TOUCH_VLDO_SEL_R {
        TOUCH_VLDO_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_comp_hys_sel(&self) -> TOUCH_COMP_HYS_SEL_R {
        TOUCH_COMP_HYS_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_current_sel(&self) -> TOUCH_CURRENT_SEL_R {
        TOUCH_CURRENT_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reserved_8_15(&self) -> RESERVED_8_15_R {
        RESERVED_8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn touch_clk_sel(&self) -> TOUCH_CLK_SEL_R {
        TOUCH_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn touch_clk_div_ratio(&self) -> TOUCH_CLK_DIV_RATIO_R {
        TOUCH_CLK_DIV_RATIO_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn touch_pcharge_high(&self) -> TOUCH_PCHARGE_HIGH_R {
        TOUCH_PCHARGE_HIGH_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn touch_pcharge_low(&self) -> TOUCH_PCHARGE_LOW_R {
        TOUCH_PCHARGE_LOW_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn touch_cont_en(&self) -> TOUCH_CONT_EN_R {
        TOUCH_CONT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn touch_cycle_en(&self) -> TOUCH_CYCLE_EN_R {
        TOUCH_CYCLE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn touch_ulp_en(&self) -> TOUCH_ULP_EN_R {
        TOUCH_ULP_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reserved_29(&self) -> RESERVED_29_R {
        RESERVED_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_touch(&self) -> PU_TOUCH_R {
        PU_TOUCH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vref_sel(&mut self) -> TOUCH_VREF_SEL_W<0> {
        TOUCH_VREF_SEL_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vldo_sel(&mut self) -> TOUCH_VLDO_SEL_W<3> {
        TOUCH_VLDO_SEL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_comp_hys_sel(&mut self) -> TOUCH_COMP_HYS_SEL_W<6> {
        TOUCH_COMP_HYS_SEL_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_current_sel(&mut self) -> TOUCH_CURRENT_SEL_W<7> {
        TOUCH_CURRENT_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn touch_clk_sel(&mut self) -> TOUCH_CLK_SEL_W<16> {
        TOUCH_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    #[must_use]
    pub fn touch_clk_div_ratio(&mut self) -> TOUCH_CLK_DIV_RATIO_W<17> {
        TOUCH_CLK_DIV_RATIO_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pcharge_high(&mut self) -> TOUCH_PCHARGE_HIGH_W<20> {
        TOUCH_PCHARGE_HIGH_W::new(self)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pcharge_low(&mut self) -> TOUCH_PCHARGE_LOW_W<23> {
        TOUCH_PCHARGE_LOW_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn touch_cont_en(&mut self) -> TOUCH_CONT_EN_W<26> {
        TOUCH_CONT_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn touch_cycle_en(&mut self) -> TOUCH_CYCLE_EN_W<27> {
        TOUCH_CYCLE_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn touch_ulp_en(&mut self) -> TOUCH_ULP_EN_W<28> {
        TOUCH_ULP_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pu_touch(&mut self) -> PU_TOUCH_W<30> {
        PU_TOUCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch channel, clock, ana config1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch1](index.html) module"]
pub struct TOUCH1_SPEC;
impl crate::RegisterSpec for TOUCH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch1::R](R) reader structure"]
impl crate::Readable for TOUCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch1::W](W) writer structure"]
impl crate::Writable for TOUCH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch1 to value 0x00a3_001b"]
impl crate::Resettable for TOUCH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00a3_001b;
}
