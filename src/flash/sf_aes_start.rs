#[doc = "Register `sf_aes_start` reader"]
pub struct R(crate::R<SF_AES_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_AES_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_AES_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes_start` writer"]
pub struct W(crate::W<SF_AES_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_START_SPEC>;
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
impl From<crate::W<SF_AES_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_AES_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_region_start` reader - "]
pub type SF_AES_REGION_START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sf_aes_region_start` writer - "]
pub type SF_AES_REGION_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_AES_START_SPEC, u32, u32, 19, O>;
#[doc = "Field `reserved_19_28` reader - "]
pub type RESERVED_19_28_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn sf_aes_region_start(&self) -> SF_AES_REGION_START_R {
        SF_AES_REGION_START_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn reserved_19_28(&self) -> RESERVED_19_28_R {
        RESERVED_19_28_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_region_start(&mut self) -> SF_AES_REGION_START_W<0> {
        SF_AES_REGION_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes_start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_start](index.html) module"]
pub struct SF_AES_START_SPEC;
impl crate::RegisterSpec for SF_AES_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes_start::R](R) reader structure"]
impl crate::Readable for SF_AES_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes_start::W](W) writer structure"]
impl crate::Writable for SF_AES_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_aes_start to value 0"]
impl crate::Resettable for SF_AES_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
