#[doc = "Register `touch_data_hystersis` reader"]
pub struct R(crate::R<TOUCH_DATA_HYSTERSIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_DATA_HYSTERSIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_DATA_HYSTERSIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_DATA_HYSTERSIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch_data_hystersis` writer"]
pub struct W(crate::W<TOUCH_DATA_HYSTERSIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_DATA_HYSTERSIS_SPEC>;
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
impl From<crate::W<TOUCH_DATA_HYSTERSIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_DATA_HYSTERSIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_data_hys` reader - "]
pub type TOUCH_DATA_HYS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_data_hys` writer - "]
pub type TOUCH_DATA_HYS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_DATA_HYSTERSIS_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn touch_data_hys(&self) -> TOUCH_DATA_HYS_R {
        TOUCH_DATA_HYS_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_data_hys(&mut self) -> TOUCH_DATA_HYS_W<0> {
        TOUCH_DATA_HYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch_data_hystersis\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_data_hystersis](index.html) module"]
pub struct TOUCH_DATA_HYSTERSIS_SPEC;
impl crate::RegisterSpec for TOUCH_DATA_HYSTERSIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_data_hystersis::R](R) reader structure"]
impl crate::Readable for TOUCH_DATA_HYSTERSIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_data_hystersis::W](W) writer structure"]
impl crate::Writable for TOUCH_DATA_HYSTERSIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch_data_hystersis to value 0"]
impl crate::Resettable for TOUCH_DATA_HYSTERSIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
