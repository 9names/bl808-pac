#[doc = "Register `ef_if_ctrl_2` reader"]
pub struct R(crate::R<EF_IF_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_ctrl_2` writer"]
pub struct W(crate::W<EF_IF_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CTRL_2_SPEC>;
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
impl From<crate::W<EF_IF_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CTRL_2_SPEC>) -> Self {
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
#[doc = "ef_if_ctrl_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ctrl_2](index.html) module"]
pub struct EF_IF_CTRL_2_SPEC;
impl crate::RegisterSpec for EF_IF_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_ctrl_2::R](R) reader structure"]
impl crate::Readable for EF_IF_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_ctrl_2::W](W) writer structure"]
impl crate::Writable for EF_IF_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_ctrl_2 to value 0"]
impl crate::Resettable for EF_IF_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
