#[doc = "Register `tzc_xram_tzsrg_r0` reader"]
pub struct R(crate::R<TZC_XRAM_TZSRG_R0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_XRAM_TZSRG_R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_XRAM_TZSRG_R0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_XRAM_TZSRG_R0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_xram_tzsrg_r0` writer"]
pub struct W(crate::W<TZC_XRAM_TZSRG_R0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_XRAM_TZSRG_R0_SPEC>;
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
impl From<crate::W<TZC_XRAM_TZSRG_R0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_XRAM_TZSRG_R0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_xram_tzsrg_r0_end` reader - "]
pub type TZC_XRAM_TZSRG_R0_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_xram_tzsrg_r0_end` writer - "]
pub type TZC_XRAM_TZSRG_R0_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_XRAM_TZSRG_R0_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_end(&self) -> TZC_XRAM_TZSRG_R0_END_R {
        TZC_XRAM_TZSRG_R0_END_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_xram_tzsrg_r0_end(&mut self) -> TZC_XRAM_TZSRG_R0_END_W<0> {
        TZC_XRAM_TZSRG_R0_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_xram_tzsrg_r0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_xram_tzsrg_r0](index.html) module"]
pub struct TZC_XRAM_TZSRG_R0_SPEC;
impl crate::RegisterSpec for TZC_XRAM_TZSRG_R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_xram_tzsrg_r0::R](R) reader structure"]
impl crate::Readable for TZC_XRAM_TZSRG_R0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_xram_tzsrg_r0::W](W) writer structure"]
impl crate::Writable for TZC_XRAM_TZSRG_R0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_xram_tzsrg_r0 to value 0x03ff"]
impl crate::Resettable for TZC_XRAM_TZSRG_R0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
