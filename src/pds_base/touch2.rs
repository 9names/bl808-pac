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
#[doc = "Field `touch_channel0_highz_en` reader - "]
pub type TOUCH_CHANNEL0_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel0_highz_en` writer - "]
pub type TOUCH_CHANNEL0_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel1_highz_en` reader - "]
pub type TOUCH_CHANNEL1_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel1_highz_en` writer - "]
pub type TOUCH_CHANNEL1_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel2_highz_en` reader - "]
pub type TOUCH_CHANNEL2_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel2_highz_en` writer - "]
pub type TOUCH_CHANNEL2_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel3_highz_en` reader - "]
pub type TOUCH_CHANNEL3_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel3_highz_en` writer - "]
pub type TOUCH_CHANNEL3_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel4_highz_en` reader - "]
pub type TOUCH_CHANNEL4_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel4_highz_en` writer - "]
pub type TOUCH_CHANNEL4_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel5_highz_en` reader - "]
pub type TOUCH_CHANNEL5_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel5_highz_en` writer - "]
pub type TOUCH_CHANNEL5_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel6_highz_en` reader - "]
pub type TOUCH_CHANNEL6_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel6_highz_en` writer - "]
pub type TOUCH_CHANNEL6_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel7_highz_en` reader - "]
pub type TOUCH_CHANNEL7_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel7_highz_en` writer - "]
pub type TOUCH_CHANNEL7_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel8_highz_en` reader - "]
pub type TOUCH_CHANNEL8_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel8_highz_en` writer - "]
pub type TOUCH_CHANNEL8_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel9_highz_en` reader - "]
pub type TOUCH_CHANNEL9_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel9_highz_en` writer - "]
pub type TOUCH_CHANNEL9_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel10_highz_en` reader - "]
pub type TOUCH_CHANNEL10_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel10_highz_en` writer - "]
pub type TOUCH_CHANNEL10_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `touch_channel11_highz_en` reader - "]
pub type TOUCH_CHANNEL11_HIGHZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel11_highz_en` writer - "]
pub type TOUCH_CHANNEL11_HIGHZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH2_SPEC, bool, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn touch_channel_sel(&self) -> TOUCH_CHANNEL_SEL_R {
        TOUCH_CHANNEL_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_channel0_highz_en(&self) -> TOUCH_CHANNEL0_HIGHZ_EN_R {
        TOUCH_CHANNEL0_HIGHZ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn touch_channel1_highz_en(&self) -> TOUCH_CHANNEL1_HIGHZ_EN_R {
        TOUCH_CHANNEL1_HIGHZ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_channel2_highz_en(&self) -> TOUCH_CHANNEL2_HIGHZ_EN_R {
        TOUCH_CHANNEL2_HIGHZ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_channel3_highz_en(&self) -> TOUCH_CHANNEL3_HIGHZ_EN_R {
        TOUCH_CHANNEL3_HIGHZ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_channel4_highz_en(&self) -> TOUCH_CHANNEL4_HIGHZ_EN_R {
        TOUCH_CHANNEL4_HIGHZ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_channel5_highz_en(&self) -> TOUCH_CHANNEL5_HIGHZ_EN_R {
        TOUCH_CHANNEL5_HIGHZ_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_channel6_highz_en(&self) -> TOUCH_CHANNEL6_HIGHZ_EN_R {
        TOUCH_CHANNEL6_HIGHZ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_channel7_highz_en(&self) -> TOUCH_CHANNEL7_HIGHZ_EN_R {
        TOUCH_CHANNEL7_HIGHZ_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_channel8_highz_en(&self) -> TOUCH_CHANNEL8_HIGHZ_EN_R {
        TOUCH_CHANNEL8_HIGHZ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_channel9_highz_en(&self) -> TOUCH_CHANNEL9_HIGHZ_EN_R {
        TOUCH_CHANNEL9_HIGHZ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_channel10_highz_en(&self) -> TOUCH_CHANNEL10_HIGHZ_EN_R {
        TOUCH_CHANNEL10_HIGHZ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_channel11_highz_en(&self) -> TOUCH_CHANNEL11_HIGHZ_EN_R {
        TOUCH_CHANNEL11_HIGHZ_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel_sel(&mut self) -> TOUCH_CHANNEL_SEL_W<0> {
        TOUCH_CHANNEL_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel0_highz_en(&mut self) -> TOUCH_CHANNEL0_HIGHZ_EN_W<4> {
        TOUCH_CHANNEL0_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel1_highz_en(&mut self) -> TOUCH_CHANNEL1_HIGHZ_EN_W<5> {
        TOUCH_CHANNEL1_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel2_highz_en(&mut self) -> TOUCH_CHANNEL2_HIGHZ_EN_W<6> {
        TOUCH_CHANNEL2_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel3_highz_en(&mut self) -> TOUCH_CHANNEL3_HIGHZ_EN_W<7> {
        TOUCH_CHANNEL3_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel4_highz_en(&mut self) -> TOUCH_CHANNEL4_HIGHZ_EN_W<8> {
        TOUCH_CHANNEL4_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel5_highz_en(&mut self) -> TOUCH_CHANNEL5_HIGHZ_EN_W<9> {
        TOUCH_CHANNEL5_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel6_highz_en(&mut self) -> TOUCH_CHANNEL6_HIGHZ_EN_W<10> {
        TOUCH_CHANNEL6_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel7_highz_en(&mut self) -> TOUCH_CHANNEL7_HIGHZ_EN_W<11> {
        TOUCH_CHANNEL7_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel8_highz_en(&mut self) -> TOUCH_CHANNEL8_HIGHZ_EN_W<12> {
        TOUCH_CHANNEL8_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel9_highz_en(&mut self) -> TOUCH_CHANNEL9_HIGHZ_EN_W<13> {
        TOUCH_CHANNEL9_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel10_highz_en(&mut self) -> TOUCH_CHANNEL10_HIGHZ_EN_W<14> {
        TOUCH_CHANNEL10_HIGHZ_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel11_highz_en(&mut self) -> TOUCH_CHANNEL11_HIGHZ_EN_W<15> {
        TOUCH_CHANNEL11_HIGHZ_EN_W::new(self)
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
#[doc = "`reset()` method sets touch2 to value 0xfff0"]
impl crate::Resettable for TOUCH2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff0;
}
