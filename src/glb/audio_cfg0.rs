#[doc = "Register `audio_cfg0` reader"]
pub struct R(crate::R<AUDIO_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_cfg0` writer"]
pub struct W(crate::W<AUDIO_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_CFG0_SPEC>;
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
impl From<crate::W<AUDIO_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_audio_pdm_clk_div` reader - "]
pub type REG_AUDIO_PDM_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_audio_pdm_clk_div` writer - "]
pub type REG_AUDIO_PDM_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_pdm_clk_en` reader - "]
pub type REG_AUDIO_PDM_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_pdm_clk_en` writer - "]
pub type REG_AUDIO_PDM_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_audio_adc_clk_div` reader - "]
pub type REG_AUDIO_ADC_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_audio_adc_clk_div` writer - "]
pub type REG_AUDIO_ADC_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_14` reader - "]
pub type RESERVED_14_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_adc_clk_en` reader - "]
pub type REG_AUDIO_ADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_adc_clk_en` writer - "]
pub type REG_AUDIO_ADC_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_audio_dac_clk_div` reader - "]
pub type REG_AUDIO_DAC_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_audio_dac_clk_div` writer - "]
pub type REG_AUDIO_DAC_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_22` reader - "]
pub type RESERVED_22_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_dac_clk_en` reader - "]
pub type REG_AUDIO_DAC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_dac_clk_en` writer - "]
pub type REG_AUDIO_DAC_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_24_30` reader - "]
pub type RESERVED_24_30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_audio_auto_div_en` reader - "]
pub type REG_AUDIO_AUTO_DIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_audio_auto_div_en` writer - "]
pub type REG_AUDIO_AUTO_DIV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_div(&self) -> REG_AUDIO_PDM_CLK_DIV_R {
        REG_AUDIO_PDM_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_en(&self) -> REG_AUDIO_PDM_CLK_EN_R {
        REG_AUDIO_PDM_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_div(&self) -> REG_AUDIO_ADC_CLK_DIV_R {
        REG_AUDIO_ADC_CLK_DIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reserved_14(&self) -> RESERVED_14_R {
        RESERVED_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_en(&self) -> REG_AUDIO_ADC_CLK_EN_R {
        REG_AUDIO_ADC_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_div(&self) -> REG_AUDIO_DAC_CLK_DIV_R {
        REG_AUDIO_DAC_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reserved_22(&self) -> RESERVED_22_R {
        RESERVED_22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_en(&self) -> REG_AUDIO_DAC_CLK_EN_R {
        REG_AUDIO_DAC_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn reserved_24_30(&self) -> RESERVED_24_30_R {
        RESERVED_24_30_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_audio_auto_div_en(&self) -> REG_AUDIO_AUTO_DIV_EN_R {
        REG_AUDIO_AUTO_DIV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_pdm_clk_div(&mut self) -> REG_AUDIO_PDM_CLK_DIV_W<0> {
        REG_AUDIO_PDM_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_pdm_clk_en(&mut self) -> REG_AUDIO_PDM_CLK_EN_W<7> {
        REG_AUDIO_PDM_CLK_EN_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_adc_clk_div(&mut self) -> REG_AUDIO_ADC_CLK_DIV_W<8> {
        REG_AUDIO_ADC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_adc_clk_en(&mut self) -> REG_AUDIO_ADC_CLK_EN_W<15> {
        REG_AUDIO_ADC_CLK_EN_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_dac_clk_div(&mut self) -> REG_AUDIO_DAC_CLK_DIV_W<16> {
        REG_AUDIO_DAC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_dac_clk_en(&mut self) -> REG_AUDIO_DAC_CLK_EN_W<23> {
        REG_AUDIO_DAC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_auto_div_en(&mut self) -> REG_AUDIO_AUTO_DIV_EN_W<31> {
        REG_AUDIO_AUTO_DIV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_cfg0](index.html) module"]
pub struct AUDIO_CFG0_SPEC;
impl crate::RegisterSpec for AUDIO_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_cfg0::R](R) reader structure"]
impl crate::Readable for AUDIO_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_cfg0::W](W) writer structure"]
impl crate::Writable for AUDIO_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_cfg0 to value 0x8083_8383"]
impl crate::Resettable for AUDIO_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8083_8383;
}
