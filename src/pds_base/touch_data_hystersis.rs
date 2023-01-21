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
#[doc = "Field `touch_sleep_cycle` reader - "]
pub type TOUCH_SLEEP_CYCLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `touch_sleep_cycle` writer - "]
pub type TOUCH_SLEEP_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_DATA_HYSTERSIS_SPEC, u32, u32, 23, O>;
#[doc = "Field `reserved_23_31` reader - "]
pub type RESERVED_23_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn touch_sleep_cycle(&self) -> TOUCH_SLEEP_CYCLE_R {
        TOUCH_SLEEP_CYCLE_R::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn reserved_23_31(&self) -> RESERVED_23_31_R {
        RESERVED_23_31_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_cycle(&mut self) -> TOUCH_SLEEP_CYCLE_W<0> {
        TOUCH_SLEEP_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch_sleep_time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_data_hystersis](index.html) module"]
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
#[doc = "`reset()` method sets touch_data_hystersis to value 0x0007_ffff"]
impl crate::Resettable for TOUCH_DATA_HYSTERSIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_ffff;
}
