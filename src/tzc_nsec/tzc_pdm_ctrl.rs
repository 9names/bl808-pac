#[doc = "Register `tzc_pdm_ctrl` reader"]
pub struct R(crate::R<TZC_PDM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PDM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PDM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PDM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_pdm_ctrl` writer"]
pub struct W(crate::W<TZC_PDM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_PDM_CTRL_SPEC>;
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
impl From<crate::W<TZC_PDM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_PDM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_31` reader - "]
pub type RESERVED_0_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reserved_0_31(&self) -> RESERVED_0_31_R {
        RESERVED_0_31_R::new(self.bits)
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
#[doc = "tzc_pdm_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pdm_ctrl](index.html) module"]
pub struct TZC_PDM_CTRL_SPEC;
impl crate::RegisterSpec for TZC_PDM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_pdm_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_PDM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_pdm_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_PDM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_pdm_ctrl to value 0"]
impl crate::Resettable for TZC_PDM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
