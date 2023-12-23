#[doc = "Register `if_sahb_1` reader"]
pub struct R(crate::R<IF_SAHB_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SAHB_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SAHB_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SAHB_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `if_sahb_1` writer"]
pub struct W(crate::W<IF_SAHB_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SAHB_1_SPEC>;
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
impl From<crate::W<IF_SAHB_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SAHB_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `if_0_cmd_buf_0` reader - "]
pub type IF_0_CMD_BUF_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `if_0_cmd_buf_0` writer - "]
pub type IF_0_CMD_BUF_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn if_0_cmd_buf_0(&self) -> IF_0_CMD_BUF_0_R {
        IF_0_CMD_BUF_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_cmd_buf_0(&mut self) -> IF_0_CMD_BUF_0_W<0> {
        IF_0_CMD_BUF_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_sahb_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_sahb_1](index.html) module"]
pub struct IF_SAHB_1_SPEC;
impl crate::RegisterSpec for IF_SAHB_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_sahb_1::R](R) reader structure"]
impl crate::Readable for IF_SAHB_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_sahb_1::W](W) writer structure"]
impl crate::Writable for IF_SAHB_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets if_sahb_1 to value 0x0300_0000"]
impl crate::Resettable for IF_SAHB_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0000;
}
