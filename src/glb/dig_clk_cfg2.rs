#[doc = "Register `dig_clk_cfg2` reader"]
pub struct R(crate::R<DIG_CLK_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_CLK_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_CLK_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_CLK_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dig_clk_cfg2` writer"]
pub struct W(crate::W<DIG_CLK_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_CLK_CFG2_SPEC>;
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
impl From<crate::W<DIG_CLK_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_CLK_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chip_clk_out_0_sel` reader - "]
pub type CHIP_CLK_OUT_0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_0_sel` writer - "]
pub type CHIP_CLK_OUT_0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `chip_clk_out_1_sel` reader - "]
pub type CHIP_CLK_OUT_1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_1_sel` writer - "]
pub type CHIP_CLK_OUT_1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `chip_clk_out_2_sel` reader - "]
pub type CHIP_CLK_OUT_2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_2_sel` writer - "]
pub type CHIP_CLK_OUT_2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `chip_clk_out_3_sel` reader - "]
pub type CHIP_CLK_OUT_3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_3_sel` writer - "]
pub type CHIP_CLK_OUT_3_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `chip_clk_out_0_en` reader - "]
pub type CHIP_CLK_OUT_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `chip_clk_out_0_en` writer - "]
pub type CHIP_CLK_OUT_0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `chip_clk_out_1_en` reader - "]
pub type CHIP_CLK_OUT_1_EN_R = crate::BitReader<bool>;
#[doc = "Field `chip_clk_out_1_en` writer - "]
pub type CHIP_CLK_OUT_1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `chip_clk_out_2_en` reader - "]
pub type CHIP_CLK_OUT_2_EN_R = crate::BitReader<bool>;
#[doc = "Field `chip_clk_out_2_en` writer - "]
pub type CHIP_CLK_OUT_2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `chip_clk_out_3_en` reader - "]
pub type CHIP_CLK_OUT_3_EN_R = crate::BitReader<bool>;
#[doc = "Field `chip_clk_out_3_en` writer - "]
pub type CHIP_CLK_OUT_3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `gpio_tmr_clk_sel` reader - "]
pub type GPIO_TMR_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpio_tmr_clk_sel` writer - "]
pub type GPIO_TMR_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpio_mm_tmr_clk_sel` reader - "]
pub type GPIO_MM_TMR_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpio_mm_tmr_clk_sel` writer - "]
pub type GPIO_MM_TMR_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&self) -> CHIP_CLK_OUT_0_SEL_R {
        CHIP_CLK_OUT_0_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&self) -> CHIP_CLK_OUT_1_SEL_R {
        CHIP_CLK_OUT_1_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chip_clk_out_2_sel(&self) -> CHIP_CLK_OUT_2_SEL_R {
        CHIP_CLK_OUT_2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chip_clk_out_3_sel(&self) -> CHIP_CLK_OUT_3_SEL_R {
        CHIP_CLK_OUT_3_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chip_clk_out_0_en(&self) -> CHIP_CLK_OUT_0_EN_R {
        CHIP_CLK_OUT_0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn chip_clk_out_1_en(&self) -> CHIP_CLK_OUT_1_EN_R {
        CHIP_CLK_OUT_1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn chip_clk_out_2_en(&self) -> CHIP_CLK_OUT_2_EN_R {
        CHIP_CLK_OUT_2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn chip_clk_out_3_en(&self) -> CHIP_CLK_OUT_3_EN_R {
        CHIP_CLK_OUT_3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gpio_tmr_clk_sel(&self) -> GPIO_TMR_CLK_SEL_R {
        GPIO_TMR_CLK_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gpio_mm_tmr_clk_sel(&self) -> GPIO_MM_TMR_CLK_SEL_R {
        GPIO_MM_TMR_CLK_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_0_sel(&mut self) -> CHIP_CLK_OUT_0_SEL_W<0> {
        CHIP_CLK_OUT_0_SEL_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_1_sel(&mut self) -> CHIP_CLK_OUT_1_SEL_W<2> {
        CHIP_CLK_OUT_1_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_2_sel(&mut self) -> CHIP_CLK_OUT_2_SEL_W<4> {
        CHIP_CLK_OUT_2_SEL_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_3_sel(&mut self) -> CHIP_CLK_OUT_3_SEL_W<6> {
        CHIP_CLK_OUT_3_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_0_en(&mut self) -> CHIP_CLK_OUT_0_EN_W<8> {
        CHIP_CLK_OUT_0_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_1_en(&mut self) -> CHIP_CLK_OUT_1_EN_W<9> {
        CHIP_CLK_OUT_1_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_2_en(&mut self) -> CHIP_CLK_OUT_2_EN_W<10> {
        CHIP_CLK_OUT_2_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_3_en(&mut self) -> CHIP_CLK_OUT_3_EN_W<11> {
        CHIP_CLK_OUT_3_EN_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_tmr_clk_sel(&mut self) -> GPIO_TMR_CLK_SEL_W<12> {
        GPIO_TMR_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_mm_tmr_clk_sel(&mut self) -> GPIO_MM_TMR_CLK_SEL_W<14> {
        GPIO_MM_TMR_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dig_clk_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_clk_cfg2](index.html) module"]
pub struct DIG_CLK_CFG2_SPEC;
impl crate::RegisterSpec for DIG_CLK_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_clk_cfg2::R](R) reader structure"]
impl crate::Readable for DIG_CLK_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_clk_cfg2::W](W) writer structure"]
impl crate::Writable for DIG_CLK_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dig_clk_cfg2 to value 0x0f00"]
impl crate::Resettable for DIG_CLK_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
