#[doc = "Register `codec_misc_dummy` reader"]
pub struct R(crate::R<CODEC_MISC_DUMMY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEC_MISC_DUMMY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEC_MISC_DUMMY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEC_MISC_DUMMY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `codec_misc_dummy` writer"]
pub struct W(crate::W<CODEC_MISC_DUMMY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEC_MISC_DUMMY_SPEC>;
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
impl From<crate::W<CODEC_MISC_DUMMY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEC_MISC_DUMMY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dummy_reg` reader - "]
pub type DUMMY_REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dummy_reg` writer - "]
pub type DUMMY_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_MISC_DUMMY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy_reg(&self) -> DUMMY_REG_R {
        DUMMY_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dummy_reg(&mut self) -> DUMMY_REG_W<0> {
        DUMMY_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CODEC_MISC_Dummy\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codec_misc_dummy](index.html) module"]
pub struct CODEC_MISC_DUMMY_SPEC;
impl crate::RegisterSpec for CODEC_MISC_DUMMY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codec_misc_dummy::R](R) reader structure"]
impl crate::Readable for CODEC_MISC_DUMMY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codec_misc_dummy::W](W) writer structure"]
impl crate::Writable for CODEC_MISC_DUMMY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets codec_misc_dummy to value 0xffff_0000"]
impl crate::Resettable for CODEC_MISC_DUMMY_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
