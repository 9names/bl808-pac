#[doc = "Register `psram_rough_delay_ctrl1` reader"]
pub struct R(crate::R<PSRAM_ROUGH_DELAY_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_ROUGH_DELAY_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_ROUGH_DELAY_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_ROUGH_DELAY_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_rough_delay_ctrl1` writer"]
pub struct W(crate::W<PSRAM_ROUGH_DELAY_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_ROUGH_DELAY_CTRL1_SPEC>;
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
impl From<crate::W<PSRAM_ROUGH_DELAY_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_ROUGH_DELAY_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_rough_sel_o_adq1` reader - "]
pub type REG_ROUGH_SEL_O_ADQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_o_adq1` writer - "]
pub type REG_ROUGH_SEL_O_ADQ1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_rough_sel_o_adq0` reader - "]
pub type REG_ROUGH_SEL_O_ADQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_o_adq0` writer - "]
pub type REG_ROUGH_SEL_O_ADQ0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq1(&self) -> REG_ROUGH_SEL_O_ADQ1_R {
        REG_ROUGH_SEL_O_ADQ1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq0(&self) -> REG_ROUGH_SEL_O_ADQ0_R {
        REG_ROUGH_SEL_O_ADQ0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_o_adq1(&mut self) -> REG_ROUGH_SEL_O_ADQ1_W<0> {
        REG_ROUGH_SEL_O_ADQ1_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_o_adq0(&mut self) -> REG_ROUGH_SEL_O_ADQ0_W<8> {
        REG_ROUGH_SEL_O_ADQ0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_rough_delay_ctrl1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_rough_delay_ctrl1](index.html) module"]
pub struct PSRAM_ROUGH_DELAY_CTRL1_SPEC;
impl crate::RegisterSpec for PSRAM_ROUGH_DELAY_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_rough_delay_ctrl1::R](R) reader structure"]
impl crate::Readable for PSRAM_ROUGH_DELAY_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_rough_delay_ctrl1::W](W) writer structure"]
impl crate::Writable for PSRAM_ROUGH_DELAY_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_rough_delay_ctrl1 to value 0"]
impl crate::Resettable for PSRAM_ROUGH_DELAY_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
