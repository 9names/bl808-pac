#[doc = "Register `channel_lta_data_1` reader"]
pub struct R(crate::R<CHANNEL_LTA_DATA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_LTA_DATA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_LTA_DATA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_LTA_DATA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel_lta_data_1` writer"]
pub struct W(crate::W<CHANNEL_LTA_DATA_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_LTA_DATA_1_SPEC>;
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
impl From<crate::W<CHANNEL_LTA_DATA_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_LTA_DATA_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_lta_data_ch1` reader - "]
pub type TOUCH_LTA_DATA_CH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn touch_lta_data_ch1(&self) -> TOUCH_LTA_DATA_CH1_R {
        TOUCH_LTA_DATA_CH1_R::new((self.bits & 0xffff) as u16)
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
#[doc = "Channel_LTA_data_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_lta_data_1](index.html) module"]
pub struct CHANNEL_LTA_DATA_1_SPEC;
impl crate::RegisterSpec for CHANNEL_LTA_DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_lta_data_1::R](R) reader structure"]
impl crate::Readable for CHANNEL_LTA_DATA_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_lta_data_1::W](W) writer structure"]
impl crate::Writable for CHANNEL_LTA_DATA_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel_lta_data_1 to value 0"]
impl crate::Resettable for CHANNEL_LTA_DATA_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
