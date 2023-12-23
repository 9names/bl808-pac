#[doc = "Register `sd_clock_ctrl` reader"]
pub struct R(crate::R<SD_CLOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CLOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CLOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CLOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_clock_ctrl` writer"]
pub struct W(crate::W<SD_CLOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CLOCK_CTRL_SPEC>;
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
impl From<crate::W<SD_CLOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CLOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `int_clk_en` reader - "]
pub type INT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `int_clk_en` writer - "]
pub type INT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CLOCK_CTRL_SPEC, bool, O>;
#[doc = "Field `int_clk_stable` reader - "]
pub type INT_CLK_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `sd_clk_en` reader - "]
pub type SD_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `sd_clk_en` writer - "]
pub type SD_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CLOCK_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_4_3` reader - "]
pub type RESERVED_4_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_gen_sel` reader - "]
pub type CLK_GEN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `clk_gen_sel` writer - "]
pub type CLK_GEN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CLOCK_CTRL_SPEC, bool, O>;
#[doc = "Field `sd_freq_sel_hi` reader - "]
pub type SD_FREQ_SEL_HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sd_freq_sel_hi` writer - "]
pub type SD_FREQ_SEL_HI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CLOCK_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `sd_freq_sel_lo` reader - "]
pub type SD_FREQ_SEL_LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sd_freq_sel_lo` writer - "]
pub type SD_FREQ_SEL_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CLOCK_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_clk_en(&self) -> INT_CLK_EN_R {
        INT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int_clk_stable(&self) -> INT_CLK_STABLE_R {
        INT_CLK_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn reserved_4_3(&self) -> RESERVED_4_3_R {
        RESERVED_4_3_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_gen_sel(&self) -> CLK_GEN_SEL_R {
        CLK_GEN_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sd_freq_sel_hi(&self) -> SD_FREQ_SEL_HI_R {
        SD_FREQ_SEL_HI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd_freq_sel_lo(&self) -> SD_FREQ_SEL_LO_R {
        SD_FREQ_SEL_LO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_en(&mut self) -> INT_CLK_EN_W<0> {
        INT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W<2> {
        SD_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gen_sel(&mut self) -> CLK_GEN_SEL_W<5> {
        CLK_GEN_SEL_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd_freq_sel_hi(&mut self) -> SD_FREQ_SEL_HI_W<6> {
        SD_FREQ_SEL_HI_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd_freq_sel_lo(&mut self) -> SD_FREQ_SEL_LO_W<8> {
        SD_FREQ_SEL_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_clock_ctrl](index.html) module"]
pub struct SD_CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for SD_CLOCK_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_clock_ctrl::R](R) reader structure"]
impl crate::Readable for SD_CLOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_clock_ctrl::W](W) writer structure"]
impl crate::Writable for SD_CLOCK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_clock_ctrl to value 0"]
impl crate::Resettable for SD_CLOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
