#[doc = "Register `tzc_mm_bmx_s0` reader"]
pub struct R(crate::R<TZC_MM_BMX_S0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_S0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_S0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_S0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_s0` writer"]
pub struct W(crate::W<TZC_MM_BMX_S0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_S0_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_S0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_S0_SPEC>) -> Self {
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
#[doc = "tzc_mm_bmx_s0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_s0](index.html) module"]
pub struct TZC_MM_BMX_S0_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_S0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_s0::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_S0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_s0::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_S0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_s0 to value 0"]
impl crate::Resettable for TZC_MM_BMX_S0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
