#[doc = "Register `audio_cfg1` reader"]
pub struct R(crate::R<AUDIO_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_cfg1` writer"]
pub struct W(crate::W<AUDIO_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_CFG1_SPEC>;
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
impl From<crate::W<AUDIO_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_padc_clk_div` reader - "]
pub type REG_PADC_CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_padc_clk_div` writer - "]
pub type REG_PADC_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_CFG1_SPEC, u16, u16, 10, O>;
#[doc = "Field `reg_padc_clk_en` reader - "]
pub type REG_PADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_padc_clk_en` writer - "]
pub type REG_PADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reg_padc_clk_div(&self) -> REG_PADC_CLK_DIV_R {
        REG_PADC_CLK_DIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_padc_clk_en(&self) -> REG_PADC_CLK_EN_R {
        REG_PADC_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_padc_clk_div(&mut self) -> REG_PADC_CLK_DIV_W<0> {
        REG_PADC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_padc_clk_en(&mut self) -> REG_PADC_CLK_EN_W<10> {
        REG_PADC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_cfg1](index.html) module"]
pub struct AUDIO_CFG1_SPEC;
impl crate::RegisterSpec for AUDIO_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_cfg1::R](R) reader structure"]
impl crate::Readable for AUDIO_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_cfg1::W](W) writer structure"]
impl crate::Writable for AUDIO_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_cfg1 to value 0x0460"]
impl crate::Resettable for AUDIO_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0460;
}
