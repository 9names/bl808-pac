#[doc = "Register `channel_force_data_0` reader"]
pub struct R(crate::R<CHANNEL_FORCE_DATA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_FORCE_DATA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_FORCE_DATA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_FORCE_DATA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel_force_data_0` writer"]
pub struct W(crate::W<CHANNEL_FORCE_DATA_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_FORCE_DATA_0_SPEC>;
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
impl From<crate::W<CHANNEL_FORCE_DATA_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_FORCE_DATA_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_force_data_ch0` reader - "]
pub type TOUCH_FORCE_DATA_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_force_data_ch0` writer - "]
pub type TOUCH_FORCE_DATA_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_FORCE_DATA_0_SPEC, u16, u16, 16, O>;
#[doc = "Field `touch_force_data_ch1` reader - "]
pub type TOUCH_FORCE_DATA_CH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_force_data_ch1` writer - "]
pub type TOUCH_FORCE_DATA_CH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_FORCE_DATA_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn touch_force_data_ch0(&self) -> TOUCH_FORCE_DATA_CH0_R {
        TOUCH_FORCE_DATA_CH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn touch_force_data_ch1(&self) -> TOUCH_FORCE_DATA_CH1_R {
        TOUCH_FORCE_DATA_CH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_data_ch0(&mut self) -> TOUCH_FORCE_DATA_CH0_W<0> {
        TOUCH_FORCE_DATA_CH0_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_data_ch1(&mut self) -> TOUCH_FORCE_DATA_CH1_W<16> {
        TOUCH_FORCE_DATA_CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel_force_data_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_force_data_0](index.html) module"]
pub struct CHANNEL_FORCE_DATA_0_SPEC;
impl crate::RegisterSpec for CHANNEL_FORCE_DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_force_data_0::R](R) reader structure"]
impl crate::Readable for CHANNEL_FORCE_DATA_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_force_data_0::W](W) writer structure"]
impl crate::Writable for CHANNEL_FORCE_DATA_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel_force_data_0 to value 0x0400_0400"]
impl crate::Resettable for CHANNEL_FORCE_DATA_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0400;
}