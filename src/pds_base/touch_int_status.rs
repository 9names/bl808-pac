#[doc = "Register `touch_int_status` reader"]
pub struct R(crate::R<TOUCH_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch_int_status` writer"]
pub struct W(crate::W<TOUCH_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_INT_STATUS_SPEC>;
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
impl From<crate::W<TOUCH_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_int_status` reader - "]
pub type TOUCH_INT_STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_end_flag` reader - "]
pub type TOUCH_END_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `reserved_13_31` reader - "]
pub type RESERVED_13_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn touch_int_status(&self) -> TOUCH_INT_STATUS_R {
        TOUCH_INT_STATUS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_end_flag(&self) -> TOUCH_END_FLAG_R {
        TOUCH_END_FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn reserved_13_31(&self) -> RESERVED_13_31_R {
        RESERVED_13_31_R::new((self.bits >> 13) & 0x0007_ffff)
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
#[doc = "touch_int_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_int_status](index.html) module"]
pub struct TOUCH_INT_STATUS_SPEC;
impl crate::RegisterSpec for TOUCH_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_int_status::R](R) reader structure"]
impl crate::Readable for TOUCH_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_int_status::W](W) writer structure"]
impl crate::Writable for TOUCH_INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch_int_status to value 0"]
impl crate::Resettable for TOUCH_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
