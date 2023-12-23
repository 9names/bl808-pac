#[doc = "Register `core_cfg21` reader"]
pub struct R(crate::R<CORE_CFG21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CFG21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CFG21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CFG21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_cfg21` writer"]
pub struct W(crate::W<CORE_CFG21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CFG21_SPEC>;
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
impl From<crate::W<CORE_CFG21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CFG21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `np_int_clr1` writer - "]
pub type NP_INT_CLR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_CFG21_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn np_int_clr1(&mut self) -> NP_INT_CLR1_W<0> {
        NP_INT_CLR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core_cfg21\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_cfg21](index.html) module"]
pub struct CORE_CFG21_SPEC;
impl crate::RegisterSpec for CORE_CFG21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_cfg21::R](R) reader structure"]
impl crate::Readable for CORE_CFG21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_cfg21::W](W) writer structure"]
impl crate::Writable for CORE_CFG21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core_cfg21 to value 0"]
impl crate::Resettable for CORE_CFG21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
