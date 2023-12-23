#[doc = "Register `channel_vth_data_2` reader"]
pub struct R(crate::R<CHANNEL_VTH_DATA_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_VTH_DATA_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_VTH_DATA_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_VTH_DATA_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel_vth_data_2` writer"]
pub struct W(crate::W<CHANNEL_VTH_DATA_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_VTH_DATA_2_SPEC>;
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
impl From<crate::W<CHANNEL_VTH_DATA_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_VTH_DATA_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_vth_data_ch8` reader - "]
pub type TOUCH_VTH_DATA_CH8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vth_data_ch8` writer - "]
pub type TOUCH_VTH_DATA_CH8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_VTH_DATA_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `touch_vth_data_ch9` reader - "]
pub type TOUCH_VTH_DATA_CH9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vth_data_ch9` writer - "]
pub type TOUCH_VTH_DATA_CH9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_VTH_DATA_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `touch_vth_data_ch10` reader - "]
pub type TOUCH_VTH_DATA_CH10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vth_data_ch10` writer - "]
pub type TOUCH_VTH_DATA_CH10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_VTH_DATA_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `touch_vth_data_ch11` reader - "]
pub type TOUCH_VTH_DATA_CH11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vth_data_ch11` writer - "]
pub type TOUCH_VTH_DATA_CH11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_VTH_DATA_2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch8(&self) -> TOUCH_VTH_DATA_CH8_R {
        TOUCH_VTH_DATA_CH8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch9(&self) -> TOUCH_VTH_DATA_CH9_R {
        TOUCH_VTH_DATA_CH9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch10(&self) -> TOUCH_VTH_DATA_CH10_R {
        TOUCH_VTH_DATA_CH10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch11(&self) -> TOUCH_VTH_DATA_CH11_R {
        TOUCH_VTH_DATA_CH11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vth_data_ch8(&mut self) -> TOUCH_VTH_DATA_CH8_W<0> {
        TOUCH_VTH_DATA_CH8_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vth_data_ch9(&mut self) -> TOUCH_VTH_DATA_CH9_W<8> {
        TOUCH_VTH_DATA_CH9_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vth_data_ch10(&mut self) -> TOUCH_VTH_DATA_CH10_W<16> {
        TOUCH_VTH_DATA_CH10_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vth_data_ch11(&mut self) -> TOUCH_VTH_DATA_CH11_W<24> {
        TOUCH_VTH_DATA_CH11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel_vth_data_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_vth_data_2](index.html) module"]
pub struct CHANNEL_VTH_DATA_2_SPEC;
impl crate::RegisterSpec for CHANNEL_VTH_DATA_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_vth_data_2::R](R) reader structure"]
impl crate::Readable for CHANNEL_VTH_DATA_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_vth_data_2::W](W) writer structure"]
impl crate::Writable for CHANNEL_VTH_DATA_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel_vth_data_2 to value 0x3f3f_3f3f"]
impl crate::Resettable for CHANNEL_VTH_DATA_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f3f_3f3f;
}