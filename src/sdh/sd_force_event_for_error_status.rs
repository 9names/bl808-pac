#[doc = "Register `sd_force_event_for_error_status` reader"]
pub struct R(crate::R<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_force_event_for_error_status` writer"]
pub struct W(crate::W<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>;
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
impl From<crate::W<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `f_cmd_to_err` writer - "]
pub type F_CMD_TO_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_cmd_crc_err` writer - "]
pub type F_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_cmd_end_bit_err` writer - "]
pub type F_CMD_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_cmd_index_err` writer - "]
pub type F_CMD_INDEX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_dat_to_err` writer - "]
pub type F_DAT_TO_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_dat_crc_err` writer - "]
pub type F_DAT_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_dat_end_bit_err` writer - "]
pub type F_DAT_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_current_err` writer - "]
pub type F_CURRENT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_acmd12_err` writer - "]
pub type F_ACMD12_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_adma_err` writer - "]
pub type F_ADMA_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `reserved_11_10` reader - "]
pub type RESERVED_11_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `f_spi_err` writer - "]
pub type F_SPI_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_axi_resp_err` writer - "]
pub type F_AXI_RESP_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_cpl_timeout_err` writer - "]
pub type F_CPL_TIMEOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
#[doc = "Field `f_crc_status_err` writer - "]
pub type F_CRC_STATUS_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reserved_11_10(&self) -> RESERVED_11_10_R {
        RESERVED_11_10_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn f_cmd_to_err(&mut self) -> F_CMD_TO_ERR_W<0> {
        F_CMD_TO_ERR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn f_cmd_crc_err(&mut self) -> F_CMD_CRC_ERR_W<1> {
        F_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn f_cmd_end_bit_err(&mut self) -> F_CMD_END_BIT_ERR_W<2> {
        F_CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn f_cmd_index_err(&mut self) -> F_CMD_INDEX_ERR_W<3> {
        F_CMD_INDEX_ERR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn f_dat_to_err(&mut self) -> F_DAT_TO_ERR_W<4> {
        F_DAT_TO_ERR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn f_dat_crc_err(&mut self) -> F_DAT_CRC_ERR_W<5> {
        F_DAT_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn f_dat_end_bit_err(&mut self) -> F_DAT_END_BIT_ERR_W<6> {
        F_DAT_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn f_current_err(&mut self) -> F_CURRENT_ERR_W<7> {
        F_CURRENT_ERR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn f_acmd12_err(&mut self) -> F_ACMD12_ERR_W<8> {
        F_ACMD12_ERR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn f_adma_err(&mut self) -> F_ADMA_ERR_W<9> {
        F_ADMA_ERR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn f_spi_err(&mut self) -> F_SPI_ERR_W<12> {
        F_SPI_ERR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn f_axi_resp_err(&mut self) -> F_AXI_RESP_ERR_W<13> {
        F_AXI_RESP_ERR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn f_cpl_timeout_err(&mut self) -> F_CPL_TIMEOUT_ERR_W<14> {
        F_CPL_TIMEOUT_ERR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn f_crc_status_err(&mut self) -> F_CRC_STATUS_ERR_W<15> {
        F_CRC_STATUS_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event for Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_force_event_for_error_status](index.html) module"]
pub struct SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_force_event_for_error_status::R](R) reader structure"]
impl crate::Readable for SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_force_event_for_error_status::W](W) writer structure"]
impl crate::Writable for SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_force_event_for_error_status to value 0"]
impl crate::Resettable for SD_FORCE_EVENT_FOR_ERROR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
