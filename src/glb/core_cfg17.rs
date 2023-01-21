#[doc = "Register `core_cfg17` reader"]
pub struct R(crate::R<CORE_CFG17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CFG17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CFG17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CFG17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_cfg17` writer"]
pub struct W(crate::W<CORE_CFG17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CFG17_SPEC>;
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
impl From<crate::W<CORE_CFG17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CFG17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `np_int_sta1` reader - "]
pub type NP_INT_STA1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_sta1(&self) -> NP_INT_STA1_R {
        NP_INT_STA1_R::new(self.bits)
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
#[doc = "core_cfg17\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_cfg17](index.html) module"]
pub struct CORE_CFG17_SPEC;
impl crate::RegisterSpec for CORE_CFG17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_cfg17::R](R) reader structure"]
impl crate::Readable for CORE_CFG17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_cfg17::W](W) writer structure"]
impl crate::Writable for CORE_CFG17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core_cfg17 to value 0"]
impl crate::Resettable for CORE_CFG17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
