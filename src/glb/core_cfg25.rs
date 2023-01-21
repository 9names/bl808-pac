#[doc = "Register `core_cfg25` reader"]
pub struct R(crate::R<CORE_CFG25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CFG25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CFG25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CFG25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_cfg25` writer"]
pub struct W(crate::W<CORE_CFG25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CFG25_SPEC>;
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
impl From<crate::W<CORE_CFG25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CFG25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_e902_int_bus_1` reader - "]
pub type STS_E902_INT_BUS_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_e902_int_bus_1(&self) -> STS_E902_INT_BUS_1_R {
        STS_E902_INT_BUS_1_R::new(self.bits)
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
#[doc = "core_cfg25\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_cfg25](index.html) module"]
pub struct CORE_CFG25_SPEC;
impl crate::RegisterSpec for CORE_CFG25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_cfg25::R](R) reader structure"]
impl crate::Readable for CORE_CFG25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_cfg25::W](W) writer structure"]
impl crate::Writable for CORE_CFG25_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core_cfg25 to value 0"]
impl crate::Resettable for CORE_CFG25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
