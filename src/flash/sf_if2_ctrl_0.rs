#[doc = "Register `sf_if2_ctrl_0` reader"]
pub struct R(crate::R<SF_IF2_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF2_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF2_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF2_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if2_ctrl_0` writer"]
pub struct W(crate::W<SF_IF2_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF2_CTRL_0_SPEC>;
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
impl From<crate::W<SF_IF2_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF2_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
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
#[doc = "sf_if2_ctrl_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if2_ctrl_0](index.html) module"]
pub struct SF_IF2_CTRL_0_SPEC;
impl crate::RegisterSpec for SF_IF2_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if2_ctrl_0::R](R) reader structure"]
impl crate::Readable for SF_IF2_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if2_ctrl_0::W](W) writer structure"]
impl crate::Writable for SF_IF2_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_if2_ctrl_0 to value 0"]
impl crate::Resettable for SF_IF2_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
