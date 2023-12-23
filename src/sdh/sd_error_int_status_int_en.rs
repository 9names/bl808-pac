#[doc = "Register `sd_error_int_status_int_en` reader"]
pub struct R(crate::R<SD_ERROR_INT_STATUS_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ERROR_INT_STATUS_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ERROR_INT_STATUS_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ERROR_INT_STATUS_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_error_int_status_int_en` writer"]
pub struct W(crate::W<SD_ERROR_INT_STATUS_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_ERROR_INT_STATUS_INT_EN_SPEC>;
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
impl From<crate::W<SD_ERROR_INT_STATUS_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_ERROR_INT_STATUS_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_timeout_err_int_en` reader - "]
pub type CMD_TIMEOUT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_timeout_err_int_en` writer - "]
pub type CMD_TIMEOUT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `cmd_crc_err_int_en` reader - "]
pub type CMD_CRC_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_crc_err_int_en` writer - "]
pub type CMD_CRC_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `cmd_end_bit_err_int_en` reader - "]
pub type CMD_END_BIT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_end_bit_err_int_en` writer - "]
pub type CMD_END_BIT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `cmd_index_err_int_en` reader - "]
pub type CMD_INDEX_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_index_err_int_en` writer - "]
pub type CMD_INDEX_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `data_timeout_err_int_en` reader - "]
pub type DATA_TIMEOUT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `data_timeout_err_int_en` writer - "]
pub type DATA_TIMEOUT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `rd_data_crc_err_int_en` reader - "]
pub type RD_DATA_CRC_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rd_data_crc_err_int_en` writer - "]
pub type RD_DATA_CRC_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `rd_data_end_bit_err_int_en` reader - "]
pub type RD_DATA_END_BIT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rd_data_end_bit_err_int_en` writer - "]
pub type RD_DATA_END_BIT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `cur_lim_err_int_en` reader - "]
pub type CUR_LIM_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cur_lim_err_int_en` writer - "]
pub type CUR_LIM_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `auto_cmd12_err_int_en` reader - "]
pub type AUTO_CMD12_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd12_err_int_en` writer - "]
pub type AUTO_CMD12_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `adma_err_int_en` reader - "]
pub type ADMA_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `adma_err_int_en` writer - "]
pub type ADMA_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `tune_err_int_en` reader - "]
pub type TUNE_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tune_err_int_en` writer - "]
pub type TUNE_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `spi_err_int_en` reader - "]
pub type SPI_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_err_int_en` writer - "]
pub type SPI_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `axi_resp_err_int_en` reader - "]
pub type AXI_RESP_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `axi_resp_err_int_en` writer - "]
pub type AXI_RESP_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `cpl_timeout_err_int_en` reader - "]
pub type CPL_TIMEOUT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpl_timeout_err_int_en` writer - "]
pub type CPL_TIMEOUT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `crc_status_err_int_en` reader - "]
pub type CRC_STATUS_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `crc_status_err_int_en` writer - "]
pub type CRC_STATUS_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_ERROR_INT_STATUS_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err_int_en(&self) -> CMD_TIMEOUT_ERR_INT_EN_R {
        CMD_TIMEOUT_ERR_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err_int_en(&self) -> CMD_CRC_ERR_INT_EN_R {
        CMD_CRC_ERR_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err_int_en(&self) -> CMD_END_BIT_ERR_INT_EN_R {
        CMD_END_BIT_ERR_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err_int_en(&self) -> CMD_INDEX_ERR_INT_EN_R {
        CMD_INDEX_ERR_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err_int_en(&self) -> DATA_TIMEOUT_ERR_INT_EN_R {
        DATA_TIMEOUT_ERR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err_int_en(&self) -> RD_DATA_CRC_ERR_INT_EN_R {
        RD_DATA_CRC_ERR_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err_int_en(&self) -> RD_DATA_END_BIT_ERR_INT_EN_R {
        RD_DATA_END_BIT_ERR_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_lim_err_int_en(&self) -> CUR_LIM_ERR_INT_EN_R {
        CUR_LIM_ERR_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err_int_en(&self) -> AUTO_CMD12_ERR_INT_EN_R {
        AUTO_CMD12_ERR_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err_int_en(&self) -> ADMA_ERR_INT_EN_R {
        ADMA_ERR_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tune_err_int_en(&self) -> TUNE_ERR_INT_EN_R {
        TUNE_ERR_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err_int_en(&self) -> SPI_ERR_INT_EN_R {
        SPI_ERR_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err_int_en(&self) -> AXI_RESP_ERR_INT_EN_R {
        AXI_RESP_ERR_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err_int_en(&self) -> CPL_TIMEOUT_ERR_INT_EN_R {
        CPL_TIMEOUT_ERR_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err_int_en(&self) -> CRC_STATUS_ERR_INT_EN_R {
        CRC_STATUS_ERR_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout_err_int_en(&mut self) -> CMD_TIMEOUT_ERR_INT_EN_W<0> {
        CMD_TIMEOUT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_int_en(&mut self) -> CMD_CRC_ERR_INT_EN_W<1> {
        CMD_CRC_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_int_en(&mut self) -> CMD_END_BIT_ERR_INT_EN_W<2> {
        CMD_END_BIT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index_err_int_en(&mut self) -> CMD_INDEX_ERR_INT_EN_W<3> {
        CMD_INDEX_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout_err_int_en(&mut self) -> DATA_TIMEOUT_ERR_INT_EN_W<4> {
        DATA_TIMEOUT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rd_data_crc_err_int_en(&mut self) -> RD_DATA_CRC_ERR_INT_EN_W<5> {
        RD_DATA_CRC_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rd_data_end_bit_err_int_en(&mut self) -> RD_DATA_END_BIT_ERR_INT_EN_W<6> {
        RD_DATA_END_BIT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lim_err_int_en(&mut self) -> CUR_LIM_ERR_INT_EN_W<7> {
        CUR_LIM_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd12_err_int_en(&mut self) -> AUTO_CMD12_ERR_INT_EN_W<8> {
        AUTO_CMD12_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_int_en(&mut self) -> ADMA_ERR_INT_EN_W<9> {
        ADMA_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tune_err_int_en(&mut self) -> TUNE_ERR_INT_EN_W<10> {
        TUNE_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn spi_err_int_en(&mut self) -> SPI_ERR_INT_EN_W<12> {
        SPI_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn axi_resp_err_int_en(&mut self) -> AXI_RESP_ERR_INT_EN_W<13> {
        AXI_RESP_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cpl_timeout_err_int_en(&mut self) -> CPL_TIMEOUT_ERR_INT_EN_W<14> {
        CPL_TIMEOUT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn crc_status_err_int_en(&mut self) -> CRC_STATUS_ERR_INT_EN_W<15> {
        CRC_STATUS_ERR_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_error_int_status_int_en](index.html) module"]
pub struct SD_ERROR_INT_STATUS_INT_EN_SPEC;
impl crate::RegisterSpec for SD_ERROR_INT_STATUS_INT_EN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_error_int_status_int_en::R](R) reader structure"]
impl crate::Readable for SD_ERROR_INT_STATUS_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_error_int_status_int_en::W](W) writer structure"]
impl crate::Writable for SD_ERROR_INT_STATUS_INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_error_int_status_int_en to value 0"]
impl crate::Resettable for SD_ERROR_INT_STATUS_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
