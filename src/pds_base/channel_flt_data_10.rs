#[doc = "Register `channel_flt_data_10` reader"]
pub struct R(crate::R<CHANNEL_FLT_DATA_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_FLT_DATA_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_FLT_DATA_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_FLT_DATA_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel_flt_data_10` writer"]
pub struct W(crate::W<CHANNEL_FLT_DATA_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_FLT_DATA_10_SPEC>;
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
impl From<crate::W<CHANNEL_FLT_DATA_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_FLT_DATA_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_flt_data_ch10` reader - "]
pub type TOUCH_FLT_DATA_CH10_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn touch_flt_data_ch10(&self) -> TOUCH_FLT_DATA_CH10_R {
        TOUCH_FLT_DATA_CH10_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Channel_FLT_data_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_flt_data_10](index.html) module"]
pub struct CHANNEL_FLT_DATA_10_SPEC;
impl crate::RegisterSpec for CHANNEL_FLT_DATA_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_flt_data_10::R](R) reader structure"]
impl crate::Readable for CHANNEL_FLT_DATA_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_flt_data_10::W](W) writer structure"]
impl crate::Writable for CHANNEL_FLT_DATA_10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel_flt_data_10 to value 0"]
impl crate::Resettable for CHANNEL_FLT_DATA_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
