#[doc = "Register `channel_force_data_4` reader"]
pub struct R(crate::R<CHANNEL_FORCE_DATA_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_FORCE_DATA_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_FORCE_DATA_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_FORCE_DATA_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel_force_data_4` writer"]
pub struct W(crate::W<CHANNEL_FORCE_DATA_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_FORCE_DATA_4_SPEC>;
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
impl From<crate::W<CHANNEL_FORCE_DATA_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_FORCE_DATA_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_force_data_ch8` reader - "]
pub type TOUCH_FORCE_DATA_CH8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_force_data_ch8` writer - "]
pub type TOUCH_FORCE_DATA_CH8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_FORCE_DATA_4_SPEC, u16, u16, 16, O>;
#[doc = "Field `touch_force_data_ch9` reader - "]
pub type TOUCH_FORCE_DATA_CH9_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_force_data_ch9` writer - "]
pub type TOUCH_FORCE_DATA_CH9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_FORCE_DATA_4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn touch_force_data_ch8(&self) -> TOUCH_FORCE_DATA_CH8_R {
        TOUCH_FORCE_DATA_CH8_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn touch_force_data_ch9(&self) -> TOUCH_FORCE_DATA_CH9_R {
        TOUCH_FORCE_DATA_CH9_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_data_ch8(&mut self) -> TOUCH_FORCE_DATA_CH8_W<0> {
        TOUCH_FORCE_DATA_CH8_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_data_ch9(&mut self) -> TOUCH_FORCE_DATA_CH9_W<16> {
        TOUCH_FORCE_DATA_CH9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel_force_data_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_force_data_4](index.html) module"]
pub struct CHANNEL_FORCE_DATA_4_SPEC;
impl crate::RegisterSpec for CHANNEL_FORCE_DATA_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_force_data_4::R](R) reader structure"]
impl crate::Readable for CHANNEL_FORCE_DATA_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_force_data_4::W](W) writer structure"]
impl crate::Writable for CHANNEL_FORCE_DATA_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel_force_data_4 to value 0x0400_0400"]
impl crate::Resettable for CHANNEL_FORCE_DATA_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0400;
}
