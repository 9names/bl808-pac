#[doc = "Register `psram_rough_delay_ctrl0` reader"]
pub struct R(crate::R<PSRAM_ROUGH_DELAY_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_ROUGH_DELAY_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_ROUGH_DELAY_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_ROUGH_DELAY_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_rough_delay_ctrl0` writer"]
pub struct W(crate::W<PSRAM_ROUGH_DELAY_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_ROUGH_DELAY_CTRL0_SPEC>;
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
impl From<crate::W<PSRAM_ROUGH_DELAY_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_ROUGH_DELAY_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_rough_delay_ctrl0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_rough_delay_ctrl0](index.html) module"]
pub struct PSRAM_ROUGH_DELAY_CTRL0_SPEC;
impl crate::RegisterSpec for PSRAM_ROUGH_DELAY_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_rough_delay_ctrl0::R](R) reader structure"]
impl crate::Readable for PSRAM_ROUGH_DELAY_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_rough_delay_ctrl0::W](W) writer structure"]
impl crate::Writable for PSRAM_ROUGH_DELAY_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_rough_delay_ctrl0 to value 0"]
impl crate::Resettable for PSRAM_ROUGH_DELAY_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
