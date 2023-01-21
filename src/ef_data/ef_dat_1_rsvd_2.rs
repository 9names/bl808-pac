#[doc = "Register `ef_dat_1_rsvd_2` reader"]
pub struct R(crate::R<EF_DAT_1_RSVD_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_DAT_1_RSVD_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_DAT_1_RSVD_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_DAT_1_RSVD_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_dat_1_rsvd_2` writer"]
pub struct W(crate::W<EF_DAT_1_RSVD_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_DAT_1_RSVD_2_SPEC>;
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
impl From<crate::W<EF_DAT_1_RSVD_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_DAT_1_RSVD_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_dat_1_rsvd_2` reader - "]
pub type EF_DAT_1_RSVD_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ef_dat_1_rsvd_2` writer - "]
pub type EF_DAT_1_RSVD_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_DAT_1_RSVD_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dat_1_rsvd_2(&self) -> EF_DAT_1_RSVD_2_R {
        EF_DAT_1_RSVD_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_dat_1_rsvd_2(&mut self) -> EF_DAT_1_RSVD_2_W<0> {
        EF_DAT_1_RSVD_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_dat_1_rsvd_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_dat_1_rsvd_2](index.html) module"]
pub struct EF_DAT_1_RSVD_2_SPEC;
impl crate::RegisterSpec for EF_DAT_1_RSVD_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_dat_1_rsvd_2::R](R) reader structure"]
impl crate::Readable for EF_DAT_1_RSVD_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_dat_1_rsvd_2::W](W) writer structure"]
impl crate::Writable for EF_DAT_1_RSVD_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_dat_1_rsvd_2 to value 0"]
impl crate::Resettable for EF_DAT_1_RSVD_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
