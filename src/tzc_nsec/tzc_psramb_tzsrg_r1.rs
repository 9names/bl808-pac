#[doc = "Register `tzc_psramb_tzsrg_r1` reader"]
pub struct R(crate::R<TZC_PSRAMB_TZSRG_R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PSRAMB_TZSRG_R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PSRAMB_TZSRG_R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PSRAMB_TZSRG_R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_psramb_tzsrg_r1` writer"]
pub struct W(crate::W<TZC_PSRAMB_TZSRG_R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_PSRAMB_TZSRG_R1_SPEC>;
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
impl From<crate::W<TZC_PSRAMB_TZSRG_R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_PSRAMB_TZSRG_R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_psramb_tzsrg_r1_end` reader - "]
pub type TZC_PSRAMB_TZSRG_R1_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_psramb_tzsrg_r1_start` reader - "]
pub type TZC_PSRAMB_TZSRG_R1_START_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_end(&self) -> TZC_PSRAMB_TZSRG_R1_END_R {
        TZC_PSRAMB_TZSRG_R1_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_start(&self) -> TZC_PSRAMB_TZSRG_R1_START_R {
        TZC_PSRAMB_TZSRG_R1_START_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "tzc_psramb_tzsrg_r1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_psramb_tzsrg_r1](index.html) module"]
pub struct TZC_PSRAMB_TZSRG_R1_SPEC;
impl crate::RegisterSpec for TZC_PSRAMB_TZSRG_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_psramb_tzsrg_r1::R](R) reader structure"]
impl crate::Readable for TZC_PSRAMB_TZSRG_R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_psramb_tzsrg_r1::W](W) writer structure"]
impl crate::Writable for TZC_PSRAMB_TZSRG_R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_psramb_tzsrg_r1 to value 0xffff"]
impl crate::Resettable for TZC_PSRAMB_TZSRG_R1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
