#[doc = "Register `sd_force_event_auto_cmd12_error` reader"]
pub struct R(crate::R<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_force_event_auto_cmd12_error` writer"]
pub struct W(crate::W<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>;
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
impl From<crate::W<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `f_acmd12_nexe_err` writer - "]
pub type F_ACMD12_NEXE_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `f_acmd_to_err` writer - "]
pub type F_ACMD_TO_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `f_acmd_crc_err` writer - "]
pub type F_ACMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `f__acmd_ebit_err` writer - "]
pub type F__ACMD_EBIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `f_acmd_index_err` writer - "]
pub type F_ACMD_INDEX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `reserved_6_5` reader - "]
pub type RESERVED_6_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `f_acmd12_issue_err` writer - "]
pub type F_ACMD12_ISSUE_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC, bool, O>;
#[doc = "Field `reserved_15_8` reader - "]
pub type RESERVED_15_8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reserved_6_5(&self) -> RESERVED_6_5_R {
        RESERVED_6_5_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reserved_15_8(&self) -> RESERVED_15_8_R {
        RESERVED_15_8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd12_nexe_err(&mut self) -> F_ACMD12_NEXE_ERR_W<0> {
        F_ACMD12_NEXE_ERR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd_to_err(&mut self) -> F_ACMD_TO_ERR_W<1> {
        F_ACMD_TO_ERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd_crc_err(&mut self) -> F_ACMD_CRC_ERR_W<2> {
        F_ACMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn f__acmd_ebit_err(&mut self) -> F__ACMD_EBIT_ERR_W<3> {
        F__ACMD_EBIT_ERR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd_index_err(&mut self) -> F_ACMD_INDEX_ERR_W<4> {
        F_ACMD_INDEX_ERR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd12_issue_err(&mut self) -> F_ACMD12_ISSUE_ERR_W<7> {
        F_ACMD12_ISSUE_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Auto cmd12 Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_force_event_auto_cmd12_error](index.html) module"]
pub struct SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC;
impl crate::RegisterSpec for SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_force_event_auto_cmd12_error::R](R) reader structure"]
impl crate::Readable for SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_force_event_auto_cmd12_error::W](W) writer structure"]
impl crate::Writable for SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_force_event_auto_cmd12_error to value 0"]
impl crate::Resettable for SD_FORCE_EVENT_AUTO_CMD12_ERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
