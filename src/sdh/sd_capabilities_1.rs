#[doc = "Register `sd_capabilities_1` reader"]
pub struct R(crate::R<SD_CAPABILITIES_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CAPABILITIES_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CAPABILITIES_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CAPABILITIES_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_capabilities_1` writer"]
pub struct W(crate::W<SD_CAPABILITIES_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CAPABILITIES_1_SPEC>;
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
impl From<crate::W<SD_CAPABILITIES_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CAPABILITIES_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timeout_freq` reader - "]
pub type TIMEOUT_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `timeout_unit` reader - "]
pub type TIMEOUT_UNIT_R = crate::BitReader<bool>;
#[doc = "Field `base_freq` reader - "]
pub type BASE_FREQ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn timeout_freq(&self) -> TIMEOUT_FREQ_R {
        TIMEOUT_FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timeout_unit(&self) -> TIMEOUT_UNIT_R {
        TIMEOUT_UNIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn base_freq(&self) -> BASE_FREQ_R {
        BASE_FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_capabilities_1](index.html) module"]
pub struct SD_CAPABILITIES_1_SPEC;
impl crate::RegisterSpec for SD_CAPABILITIES_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_capabilities_1::R](R) reader structure"]
impl crate::Readable for SD_CAPABILITIES_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_capabilities_1::W](W) writer structure"]
impl crate::Writable for SD_CAPABILITIES_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_capabilities_1 to value 0x80"]
impl crate::Resettable for SD_CAPABILITIES_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
