#[doc = "Register `adc_cfg0` reader"]
pub struct R(crate::R<ADC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adc_cfg0` writer"]
pub struct W(crate::W<ADC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFG0_SPEC>;
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
impl From<crate::W<ADC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_32m_clk_div` reader - "]
pub type GPADC_32M_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_32m_clk_div` writer - "]
pub type GPADC_32M_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_32m_clk_sel` reader - "]
pub type GPADC_32M_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_32m_clk_sel` writer - "]
pub type GPADC_32M_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFG0_SPEC, bool, O>;
#[doc = "Field `gpadc_32m_div_en` reader - "]
pub type GPADC_32M_DIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_32m_div_en` writer - "]
pub type GPADC_32M_DIV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> GPADC_32M_CLK_DIV_R {
        GPADC_32M_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> GPADC_32M_CLK_SEL_R {
        GPADC_32M_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> GPADC_32M_DIV_EN_R {
        GPADC_32M_DIV_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_clk_div(&mut self) -> GPADC_32M_CLK_DIV_W<0> {
        GPADC_32M_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_clk_sel(&mut self) -> GPADC_32M_CLK_SEL_W<7> {
        GPADC_32M_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_div_en(&mut self) -> GPADC_32M_DIV_EN_W<8> {
        GPADC_32M_DIV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adc_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfg0](index.html) module"]
pub struct ADC_CFG0_SPEC;
impl crate::RegisterSpec for ADC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfg0::R](R) reader structure"]
impl crate::Readable for ADC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfg0::W](W) writer structure"]
impl crate::Writable for ADC_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adc_cfg0 to value 0x0102"]
impl crate::Resettable for ADC_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102;
}
