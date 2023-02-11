#[doc = "Register `sd_adma_error_status` reader"]
pub struct R(crate::R<SD_ADMA_ERROR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ADMA_ERROR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ADMA_ERROR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ADMA_ERROR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_adma_error_status` writer"]
pub struct W(crate::W<SD_ADMA_ERROR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_ADMA_ERROR_STATUS_SPEC>;
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
impl From<crate::W<SD_ADMA_ERROR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_ADMA_ERROR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adma_state` reader - "]
pub type ADMA_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adma_state` writer - "]
pub type ADMA_STATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_ADMA_ERROR_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adma_len_err` reader - "]
pub type ADMA_LEN_ERR_R = crate::BitReader<bool>;
#[doc = "Field `adma_len_err` writer - "]
pub type ADMA_LEN_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ADMA_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `reserved_15_3` reader - "]
pub type RESERVED_15_3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adma_state(&self) -> ADMA_STATE_R {
        ADMA_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adma_len_err(&self) -> ADMA_LEN_ERR_R {
        ADMA_LEN_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn reserved_15_3(&self) -> RESERVED_15_3_R {
        RESERVED_15_3_R::new((self.bits >> 3) & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn adma_state(&mut self) -> ADMA_STATE_W<0> {
        ADMA_STATE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn adma_len_err(&mut self) -> ADMA_LEN_ERR_W<2> {
        ADMA_LEN_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADMA Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_adma_error_status](index.html) module"]
pub struct SD_ADMA_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for SD_ADMA_ERROR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_adma_error_status::R](R) reader structure"]
impl crate::Readable for SD_ADMA_ERROR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_adma_error_status::W](W) writer structure"]
impl crate::Writable for SD_ADMA_ERROR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_adma_error_status to value 0"]
impl crate::Resettable for SD_ADMA_ERROR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
