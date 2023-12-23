#[doc = "Register `sd_present_state_1` reader"]
pub struct R(crate::R<SD_PRESENT_STATE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_PRESENT_STATE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_PRESENT_STATE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_PRESENT_STATE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_present_state_1` writer"]
pub struct W(crate::W<SD_PRESENT_STATE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_PRESENT_STATE_1_SPEC>;
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
impl From<crate::W<SD_PRESENT_STATE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_PRESENT_STATE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_inhibit_cmd` reader - "]
pub type CMD_INHIBIT_CMD_R = crate::BitReader<bool>;
#[doc = "Field `cmd_inhibit_dat` reader - "]
pub type CMD_INHIBIT_DAT_R = crate::BitReader<bool>;
#[doc = "Field `_dat_active` reader - "]
pub type _DAT_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `retuning_req` reader - "]
pub type RETUNING_REQ_R = crate::BitReader<bool>;
#[doc = "Field `reserved_7_4` reader - "]
pub type RESERVED_7_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_active` reader - "]
pub type TX_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `rx_active` reader - "]
pub type RX_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `buffer_wr_en` reader - "]
pub type BUFFER_WR_EN_R = crate::BitReader<bool>;
#[doc = "Field `buffer_rd_en` reader - "]
pub type BUFFER_RD_EN_R = crate::BitReader<bool>;
#[doc = "Field `reserved_15_12` reader - "]
pub type RESERVED_15_12_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_inhibit_cmd(&self) -> CMD_INHIBIT_CMD_R {
        CMD_INHIBIT_CMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_inhibit_dat(&self) -> CMD_INHIBIT_DAT_R {
        CMD_INHIBIT_DAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn _dat_active(&self) -> _DAT_ACTIVE_R {
        _DAT_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn retuning_req(&self) -> RETUNING_REQ_R {
        RETUNING_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_7_4(&self) -> RESERVED_7_4_R {
        RESERVED_7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_active(&self) -> TX_ACTIVE_R {
        TX_ACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_active(&self) -> RX_ACTIVE_R {
        RX_ACTIVE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn buffer_wr_en(&self) -> BUFFER_WR_EN_R {
        BUFFER_WR_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn buffer_rd_en(&self) -> BUFFER_RD_EN_R {
        BUFFER_RD_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_15_12(&self) -> RESERVED_15_12_R {
        RESERVED_15_12_R::new(((self.bits >> 12) & 0x0f) as u8)
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
#[doc = "Present State Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_present_state_1](index.html) module"]
pub struct SD_PRESENT_STATE_1_SPEC;
impl crate::RegisterSpec for SD_PRESENT_STATE_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_present_state_1::R](R) reader structure"]
impl crate::Readable for SD_PRESENT_STATE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_present_state_1::W](W) writer structure"]
impl crate::Writable for SD_PRESENT_STATE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_present_state_1 to value 0x0400"]
impl crate::Resettable for SD_PRESENT_STATE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
