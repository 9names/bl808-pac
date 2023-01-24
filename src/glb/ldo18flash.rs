#[doc = "Register `ldo18flash` reader"]
pub struct R(crate::R<LDO18FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO18FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO18FLASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO18FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo18flash` writer"]
pub struct W(crate::W<LDO18FLASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO18FLASH_SPEC>;
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
impl From<crate::W<LDO18FLASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO18FLASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ldo18flash` reader - "]
pub type PU_LDO18FLASH_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo18flash` writer - "]
pub type PU_LDO18FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_bypass` reader - "]
pub type LDO18FLASH_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_bypass` writer - "]
pub type LDO18FLASH_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_pulldown` reader - "]
pub type LDO18FLASH_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_pulldown` writer - "]
pub type LDO18FLASH_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo18flash(&self) -> PU_LDO18FLASH_R {
        PU_LDO18FLASH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18flash_bypass(&self) -> LDO18FLASH_BYPASS_R {
        LDO18FLASH_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18flash_pulldown(&self) -> LDO18FLASH_PULLDOWN_R {
        LDO18FLASH_PULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo18flash(&mut self) -> PU_LDO18FLASH_W<0> {
        PU_LDO18FLASH_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_bypass(&mut self) -> LDO18FLASH_BYPASS_W<1> {
        LDO18FLASH_BYPASS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_pulldown(&mut self) -> LDO18FLASH_PULLDOWN_W<2> {
        LDO18FLASH_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo18flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo18flash](index.html) module"]
pub struct LDO18FLASH_SPEC;
impl crate::RegisterSpec for LDO18FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo18flash::R](R) reader structure"]
impl crate::Readable for LDO18FLASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo18flash::W](W) writer structure"]
impl crate::Writable for LDO18FLASH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18flash to value 0"]
impl crate::Resettable for LDO18FLASH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
