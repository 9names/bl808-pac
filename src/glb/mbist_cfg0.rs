#[doc = "Register `mbist_cfg0` reader"]
pub struct R(crate::R<MBIST_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIST_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIST_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mbist_cfg0` writer"]
pub struct W(crate::W<MBIST_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_CFG0_SPEC>;
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
impl From<crate::W<MBIST_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBIST_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mbist_mode` reader - "]
pub type MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `mbist_mode` writer - "]
pub type MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_1_31` reader - "]
pub type RESERVED_1_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mbist_mode(&self) -> MBIST_MODE_R {
        MBIST_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved_1_31(&self) -> RESERVED_1_31_R {
        RESERVED_1_31_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mbist_mode(&mut self) -> MBIST_MODE_W<0> {
        MBIST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mbist_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_cfg0](index.html) module"]
pub struct MBIST_CFG0_SPEC;
impl crate::RegisterSpec for MBIST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_cfg0::R](R) reader structure"]
impl crate::Readable for MBIST_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_cfg0::W](W) writer structure"]
impl crate::Writable for MBIST_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mbist_cfg0 to value 0"]
impl crate::Resettable for MBIST_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
