#[doc = "Register `psram_dummy_reg` reader"]
pub struct R(crate::R<PSRAM_DUMMY_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_DUMMY_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_DUMMY_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_DUMMY_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_dummy_reg` writer"]
pub struct W(crate::W<PSRAM_DUMMY_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_DUMMY_REG_SPEC>;
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
impl From<crate::W<PSRAM_DUMMY_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_DUMMY_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_psram_dummy_reg` reader - "]
pub type REG_PSRAM_DUMMY_REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_psram_dummy_reg` writer - "]
pub type REG_PSRAM_DUMMY_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_DUMMY_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_psram_dummy_reg(&self) -> REG_PSRAM_DUMMY_REG_R {
        REG_PSRAM_DUMMY_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_dummy_reg(&mut self) -> REG_PSRAM_DUMMY_REG_W<0> {
        REG_PSRAM_DUMMY_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_dummy_reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_dummy_reg](index.html) module"]
pub struct PSRAM_DUMMY_REG_SPEC;
impl crate::RegisterSpec for PSRAM_DUMMY_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_dummy_reg::R](R) reader structure"]
impl crate::Readable for PSRAM_DUMMY_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_dummy_reg::W](W) writer structure"]
impl crate::Writable for PSRAM_DUMMY_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_dummy_reg to value 0xffff_0000"]
impl crate::Resettable for PSRAM_DUMMY_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
