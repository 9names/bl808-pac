#[doc = "Register `touch_rsvd` reader"]
pub struct R(crate::R<TOUCH_RSVD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_RSVD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_RSVD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_RSVD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch_rsvd` writer"]
pub struct W(crate::W<TOUCH_RSVD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_RSVD_SPEC>;
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
impl From<crate::W<TOUCH_RSVD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_RSVD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_reserved` reader - "]
pub type TOUCH_RESERVED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_reserved` writer - "]
pub type TOUCH_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_RSVD_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_8_31` reader - "]
pub type RESERVED_8_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_reserved(&self) -> TOUCH_RESERVED_R {
        TOUCH_RESERVED_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved_8_31(&self) -> RESERVED_8_31_R {
        RESERVED_8_31_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_reserved(&mut self) -> TOUCH_RESERVED_W<0> {
        TOUCH_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch_rsvd\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_rsvd](index.html) module"]
pub struct TOUCH_RSVD_SPEC;
impl crate::RegisterSpec for TOUCH_RSVD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_rsvd::R](R) reader structure"]
impl crate::Readable for TOUCH_RSVD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_rsvd::W](W) writer structure"]
impl crate::Writable for TOUCH_RSVD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch_rsvd to value 0"]
impl crate::Resettable for TOUCH_RSVD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
