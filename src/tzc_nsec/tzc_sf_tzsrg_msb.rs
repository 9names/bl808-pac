#[doc = "Register `tzc_sf_tzsrg_msb` reader"]
pub struct R(crate::R<TZC_SF_TZSRG_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SF_TZSRG_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SF_TZSRG_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SF_TZSRG_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_sf_tzsrg_msb` writer"]
pub struct W(crate::W<TZC_SF_TZSRG_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SF_TZSRG_MSB_SPEC>;
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
impl From<crate::W<TZC_SF_TZSRG_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SF_TZSRG_MSB_SPEC>) -> Self {
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
#[doc = "tzc_sf_tzsrg_msb\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_sf_tzsrg_msb](index.html) module"]
pub struct TZC_SF_TZSRG_MSB_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_sf_tzsrg_msb::R](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_sf_tzsrg_msb::W](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_MSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_msb to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}