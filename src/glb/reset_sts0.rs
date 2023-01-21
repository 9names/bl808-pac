#[doc = "Register `reset_sts0` reader"]
pub struct R(crate::R<RESET_STS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reset_sts0` writer"]
pub struct W(crate::W<RESET_STS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STS0_SPEC>;
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
impl From<crate::W<RESET_STS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `top_reset_recorder` reader - "]
pub type TOP_RESET_RECORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clr_top_reset_recorder` reader - "]
pub type CLR_TOP_RESET_RECORDER_R = crate::BitReader<bool>;
#[doc = "Field `clr_top_reset_recorder` writer - "]
pub type CLR_TOP_RESET_RECORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESET_STS0_SPEC, bool, O>;
#[doc = "Field `reserved_8_31` reader - "]
pub type RESERVED_8_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn top_reset_recorder(&self) -> TOP_RESET_RECORDER_R {
        TOP_RESET_RECORDER_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clr_top_reset_recorder(&self) -> CLR_TOP_RESET_RECORDER_R {
        CLR_TOP_RESET_RECORDER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved_8_31(&self) -> RESERVED_8_31_R {
        RESERVED_8_31_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clr_top_reset_recorder(&mut self) -> CLR_TOP_RESET_RECORDER_W<7> {
        CLR_TOP_RESET_RECORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset_sts0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_sts0](index.html) module"]
pub struct RESET_STS0_SPEC;
impl crate::RegisterSpec for RESET_STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_sts0::R](R) reader structure"]
impl crate::Readable for RESET_STS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_sts0::W](W) writer structure"]
impl crate::Writable for RESET_STS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reset_sts0 to value 0"]
impl crate::Resettable for RESET_STS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
