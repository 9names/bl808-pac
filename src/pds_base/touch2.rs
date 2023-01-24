#[doc = "Register `touch2` reader"]
pub struct R(crate::R<TOUCH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch2` writer"]
pub struct W(crate::W<TOUCH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH2_SPEC>;
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
impl From<crate::W<TOUCH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_channel_sel` reader - "]
pub type TOUCH_CHANNEL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_channel_sel` writer - "]
pub type TOUCH_CHANNEL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn touch_channel_sel(&self) -> TOUCH_CHANNEL_SEL_R {
        TOUCH_CHANNEL_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel_sel(&mut self) -> TOUCH_CHANNEL_SEL_W<0> {
        TOUCH_CHANNEL_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch channel, clock, ana config2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch2](index.html) module"]
pub struct TOUCH2_SPEC;
impl crate::RegisterSpec for TOUCH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch2::R](R) reader structure"]
impl crate::Readable for TOUCH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch2::W](W) writer structure"]
impl crate::Writable for TOUCH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch2 to value 0"]
impl crate::Resettable for TOUCH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
