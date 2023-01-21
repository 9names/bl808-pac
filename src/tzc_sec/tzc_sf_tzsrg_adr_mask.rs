#[doc = "Register `tzc_sf_tzsrg_adr_mask` reader"]
pub struct R(crate::R<TZC_SF_TZSRG_ADR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SF_TZSRG_ADR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SF_TZSRG_ADR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SF_TZSRG_ADR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_sf_tzsrg_adr_mask` writer"]
pub struct W(crate::W<TZC_SF_TZSRG_ADR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SF_TZSRG_ADR_MASK_SPEC>;
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
impl From<crate::W<TZC_SF_TZSRG_ADR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SF_TZSRG_ADR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_sf_tzsrg_adr_mask` reader - "]
pub type TZC_SF_TZSRG_ADR_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` writer - "]
pub type TZC_SF_TZSRG_ADR_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_SF_TZSRG_ADR_MASK_SPEC, u32, u32, 19, O>;
#[doc = "Field `reserved_19_30` reader - "]
pub type RESERVED_19_30_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` reader - "]
pub type TZC_SF_TZSRG_ADR_MASK_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` writer - "]
pub type TZC_SF_TZSRG_ADR_MASK_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SF_TZSRG_ADR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask(&self) -> TZC_SF_TZSRG_ADR_MASK_R {
        TZC_SF_TZSRG_ADR_MASK_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:30"]
    #[inline(always)]
    pub fn reserved_19_30(&self) -> RESERVED_19_30_R {
        RESERVED_19_30_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask_lock(&self) -> TZC_SF_TZSRG_ADR_MASK_LOCK_R {
        TZC_SF_TZSRG_ADR_MASK_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_adr_mask(&mut self) -> TZC_SF_TZSRG_ADR_MASK_W<0> {
        TZC_SF_TZSRG_ADR_MASK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_adr_mask_lock(&mut self) -> TZC_SF_TZSRG_ADR_MASK_LOCK_W<31> {
        TZC_SF_TZSRG_ADR_MASK_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_sf_tzsrg_adr_mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_sf_tzsrg_adr_mask](index.html) module"]
pub struct TZC_SF_TZSRG_ADR_MASK_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_ADR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_sf_tzsrg_adr_mask::R](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_ADR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_sf_tzsrg_adr_mask::W](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_ADR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_adr_mask to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_ADR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
