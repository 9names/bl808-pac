#[doc = "Register `sd_max_current_1` reader"]
pub struct R(crate::R<SD_MAX_CURRENT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_MAX_CURRENT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_MAX_CURRENT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_MAX_CURRENT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_max_current_1` writer"]
pub struct W(crate::W<SD_MAX_CURRENT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_MAX_CURRENT_1_SPEC>;
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
impl From<crate::W<SD_MAX_CURRENT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_MAX_CURRENT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `max_cur_33` reader - "]
pub type MAX_CUR_33_R = crate::FieldReader<u8, u8>;
#[doc = "Field `max_cur_30` reader - "]
pub type MAX_CUR_30_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn max_cur_33(&self) -> MAX_CUR_33_R {
        MAX_CUR_33_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn max_cur_30(&self) -> MAX_CUR_30_R {
        MAX_CUR_30_R::new(((self.bits >> 8) & 0xff) as u8)
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
#[doc = "Maximum Current Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_max_current_1](index.html) module"]
pub struct SD_MAX_CURRENT_1_SPEC;
impl crate::RegisterSpec for SD_MAX_CURRENT_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_max_current_1::R](R) reader structure"]
impl crate::Readable for SD_MAX_CURRENT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_max_current_1::W](W) writer structure"]
impl crate::Writable for SD_MAX_CURRENT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_max_current_1 to value 0"]
impl crate::Resettable for SD_MAX_CURRENT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
