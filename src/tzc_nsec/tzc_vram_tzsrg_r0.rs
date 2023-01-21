#[doc = "Register `tzc_vram_tzsrg_r0` reader"]
pub struct R(crate::R<TZC_VRAM_TZSRG_R0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_VRAM_TZSRG_R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_VRAM_TZSRG_R0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_VRAM_TZSRG_R0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_vram_tzsrg_r0` writer"]
pub struct W(crate::W<TZC_VRAM_TZSRG_R0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_VRAM_TZSRG_R0_SPEC>;
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
impl From<crate::W<TZC_VRAM_TZSRG_R0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_VRAM_TZSRG_R0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_vram_tzsrg_r0_end` reader - "]
pub type TZC_VRAM_TZSRG_R0_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_vram_tzsrg_r0_start` reader - "]
pub type TZC_VRAM_TZSRG_R0_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_end(&self) -> TZC_VRAM_TZSRG_R0_END_R {
        TZC_VRAM_TZSRG_R0_END_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_start(&self) -> TZC_VRAM_TZSRG_R0_START_R {
        TZC_VRAM_TZSRG_R0_START_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
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
#[doc = "tzc_vram_tzsrg_r0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_vram_tzsrg_r0](index.html) module"]
pub struct TZC_VRAM_TZSRG_R0_SPEC;
impl crate::RegisterSpec for TZC_VRAM_TZSRG_R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_vram_tzsrg_r0::R](R) reader structure"]
impl crate::Readable for TZC_VRAM_TZSRG_R0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_vram_tzsrg_r0::W](W) writer structure"]
impl crate::Writable for TZC_VRAM_TZSRG_R0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_vram_tzsrg_r0 to value 0x03ff"]
impl crate::Resettable for TZC_VRAM_TZSRG_R0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
