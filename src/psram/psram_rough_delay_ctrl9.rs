#[doc = "Register `psram_rough_delay_ctrl9` reader"]
pub struct R(crate::R<PSRAM_ROUGH_DELAY_CTRL9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_ROUGH_DELAY_CTRL9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_ROUGH_DELAY_CTRL9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_ROUGH_DELAY_CTRL9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_rough_delay_ctrl9` writer"]
pub struct W(crate::W<PSRAM_ROUGH_DELAY_CTRL9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_ROUGH_DELAY_CTRL9_SPEC>;
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
impl From<crate::W<PSRAM_ROUGH_DELAY_CTRL9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_ROUGH_DELAY_CTRL9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_rough_sel_i_adq13` reader - "]
pub type REG_ROUGH_SEL_I_ADQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_i_adq13` writer - "]
pub type REG_ROUGH_SEL_I_ADQ13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL9_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_rough_sel_i_adq12` reader - "]
pub type REG_ROUGH_SEL_I_ADQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_i_adq12` writer - "]
pub type REG_ROUGH_SEL_I_ADQ12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL9_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_rough_sel_i_adq11` reader - "]
pub type REG_ROUGH_SEL_I_ADQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_i_adq11` writer - "]
pub type REG_ROUGH_SEL_I_ADQ11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL9_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_rough_sel_i_adq10` reader - "]
pub type REG_ROUGH_SEL_I_ADQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_rough_sel_i_adq10` writer - "]
pub type REG_ROUGH_SEL_I_ADQ10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_ROUGH_DELAY_CTRL9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq13(&self) -> REG_ROUGH_SEL_I_ADQ13_R {
        REG_ROUGH_SEL_I_ADQ13_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq12(&self) -> REG_ROUGH_SEL_I_ADQ12_R {
        REG_ROUGH_SEL_I_ADQ12_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq11(&self) -> REG_ROUGH_SEL_I_ADQ11_R {
        REG_ROUGH_SEL_I_ADQ11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq10(&self) -> REG_ROUGH_SEL_I_ADQ10_R {
        REG_ROUGH_SEL_I_ADQ10_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_i_adq13(&mut self) -> REG_ROUGH_SEL_I_ADQ13_W<0> {
        REG_ROUGH_SEL_I_ADQ13_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_i_adq12(&mut self) -> REG_ROUGH_SEL_I_ADQ12_W<8> {
        REG_ROUGH_SEL_I_ADQ12_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_i_adq11(&mut self) -> REG_ROUGH_SEL_I_ADQ11_W<16> {
        REG_ROUGH_SEL_I_ADQ11_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rough_sel_i_adq10(&mut self) -> REG_ROUGH_SEL_I_ADQ10_W<24> {
        REG_ROUGH_SEL_I_ADQ10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_rough_delay_ctrl9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_rough_delay_ctrl9](index.html) module"]
pub struct PSRAM_ROUGH_DELAY_CTRL9_SPEC;
impl crate::RegisterSpec for PSRAM_ROUGH_DELAY_CTRL9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_rough_delay_ctrl9::R](R) reader structure"]
impl crate::Readable for PSRAM_ROUGH_DELAY_CTRL9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_rough_delay_ctrl9::W](W) writer structure"]
impl crate::Writable for PSRAM_ROUGH_DELAY_CTRL9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_rough_delay_ctrl9 to value 0"]
impl crate::Resettable for PSRAM_ROUGH_DELAY_CTRL9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
