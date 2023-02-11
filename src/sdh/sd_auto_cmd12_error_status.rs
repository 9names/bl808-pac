#[doc = "Register `sd_auto_cmd12_error_status` reader"]
pub struct R(crate::R<SD_AUTO_CMD12_ERROR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_AUTO_CMD12_ERROR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_AUTO_CMD12_ERROR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_AUTO_CMD12_ERROR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_auto_cmd12_error_status` writer"]
pub struct W(crate::W<SD_AUTO_CMD12_ERROR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_AUTO_CMD12_ERROR_STATUS_SPEC>;
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
impl From<crate::W<SD_AUTO_CMD12_ERROR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_AUTO_CMD12_ERROR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `auto_cmd12_not_exe` reader - "]
pub type AUTO_CMD12_NOT_EXE_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd12_not_exe` writer - "]
pub type AUTO_CMD12_NOT_EXE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_AUTO_CMD12_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `auto_cmd_timeout_err` reader - "]
pub type AUTO_CMD_TIMEOUT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd_timeout_err` writer - "]
pub type AUTO_CMD_TIMEOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_AUTO_CMD12_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `auto_cmd_crc_err` reader - "]
pub type AUTO_CMD_CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd_crc_err` writer - "]
pub type AUTO_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_AUTO_CMD12_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `auto_cmd_end_bit_err` reader - "]
pub type AUTO_CMD_END_BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd_end_bit_err` writer - "]
pub type AUTO_CMD_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_AUTO_CMD12_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `auto_cmd_index_err` reader - "]
pub type AUTO_CMD_INDEX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd_index_err` writer - "]
pub type AUTO_CMD_INDEX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_AUTO_CMD12_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `reserved_6_5` reader - "]
pub type RESERVED_6_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_not_issued` reader - "]
pub type CMD_NOT_ISSUED_R = crate::BitReader<bool>;
#[doc = "Field `reserved_15_8` reader - "]
pub type RESERVED_15_8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_cmd12_not_exe(&self) -> AUTO_CMD12_NOT_EXE_R {
        AUTO_CMD12_NOT_EXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_cmd_timeout_err(&self) -> AUTO_CMD_TIMEOUT_ERR_R {
        AUTO_CMD_TIMEOUT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(&self) -> AUTO_CMD_CRC_ERR_R {
        AUTO_CMD_CRC_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn auto_cmd_end_bit_err(&self) -> AUTO_CMD_END_BIT_ERR_R {
        AUTO_CMD_END_BIT_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn auto_cmd_index_err(&self) -> AUTO_CMD_INDEX_ERR_R {
        AUTO_CMD_INDEX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reserved_6_5(&self) -> RESERVED_6_5_R {
        RESERVED_6_5_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cmd_not_issued(&self) -> CMD_NOT_ISSUED_R {
        CMD_NOT_ISSUED_R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn auto_cmd12_not_exe(&mut self) -> AUTO_CMD12_NOT_EXE_W<0> {
        AUTO_CMD12_NOT_EXE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_timeout_err(&mut self) -> AUTO_CMD_TIMEOUT_ERR_W<1> {
        AUTO_CMD_TIMEOUT_ERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_crc_err(&mut self) -> AUTO_CMD_CRC_ERR_W<2> {
        AUTO_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_end_bit_err(&mut self) -> AUTO_CMD_END_BIT_ERR_W<3> {
        AUTO_CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_index_err(&mut self) -> AUTO_CMD_INDEX_ERR_W<4> {
        AUTO_CMD_INDEX_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto CMD12 Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_auto_cmd12_error_status](index.html) module"]
pub struct SD_AUTO_CMD12_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for SD_AUTO_CMD12_ERROR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_auto_cmd12_error_status::R](R) reader structure"]
impl crate::Readable for SD_AUTO_CMD12_ERROR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_auto_cmd12_error_status::W](W) writer structure"]
impl crate::Writable for SD_AUTO_CMD12_ERROR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_auto_cmd12_error_status to value 0"]
impl crate::Resettable for SD_AUTO_CMD12_ERROR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
