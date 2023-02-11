#[doc = "Register `sd_resp_5` reader"]
pub struct R(crate::R<SD_RESP_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RESP_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RESP_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RESP_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_resp_5` writer"]
pub struct W(crate::W<SD_RESP_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_RESP_5_SPEC>;
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
impl From<crate::W<SD_RESP_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_RESP_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `resp5` reader - "]
pub type RESP5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp5(&self) -> RESP5_R {
        RESP5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_resp_5](index.html) module"]
pub struct SD_RESP_5_SPEC;
impl crate::RegisterSpec for SD_RESP_5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_resp_5::R](R) reader structure"]
impl crate::Readable for SD_RESP_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_resp_5::W](W) writer structure"]
impl crate::Writable for SD_RESP_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_resp_5 to value 0"]
impl crate::Resettable for SD_RESP_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
