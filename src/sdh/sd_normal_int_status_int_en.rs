#[doc = "Register `sd_normal_int_status_int_en` reader"]
pub struct R(crate::R<SD_NORMAL_INT_STATUS_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_NORMAL_INT_STATUS_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_NORMAL_INT_STATUS_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_NORMAL_INT_STATUS_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_normal_int_status_int_en` writer"]
pub struct W(crate::W<SD_NORMAL_INT_STATUS_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_NORMAL_INT_STATUS_INT_EN_SPEC>;
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
impl From<crate::W<SD_NORMAL_INT_STATUS_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_NORMAL_INT_STATUS_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_complete_int_en` reader - "]
pub type CMD_COMPLETE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_complete_int_en` writer - "]
pub type CMD_COMPLETE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `xfer_complete_int_en` reader - "]
pub type XFER_COMPLETE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `xfer_complete_int_en` writer - "]
pub type XFER_COMPLETE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `block_gap_evt_int_en` reader - "]
pub type BLOCK_GAP_EVT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `block_gap_evt_int_en` writer - "]
pub type BLOCK_GAP_EVT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `dma_int_int_en` reader - "]
pub type DMA_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dma_int_int_en` writer - "]
pub type DMA_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `tx_rdy_int_en` reader - "]
pub type TX_RDY_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_rdy_int_en` writer - "]
pub type TX_RDY_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `rx_rdy_int_en` reader - "]
pub type RX_RDY_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_rdy_int_en` writer - "]
pub type RX_RDY_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `card_ins_int_en` reader - "]
pub type CARD_INS_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `card_ins_int_en` writer - "]
pub type CARD_INS_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `card_rem_int_en` reader - "]
pub type CARD_REM_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `card_rem_int_en` writer - "]
pub type CARD_REM_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `card_int_int_en` reader - "]
pub type CARD_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `card_int_int_en` writer - "]
pub type CARD_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `int_a_int_int_en` reader - "]
pub type INT_A_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `int_a_int_int_en` writer - "]
pub type INT_A_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `int_b_int_int_en` reader - "]
pub type INT_B_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `int_b_int_int_en` writer - "]
pub type INT_B_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `int_c_int_int_en` reader - "]
pub type INT_C_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `int_c_int_int_en` writer - "]
pub type INT_C_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `retune_int_int_en` reader - "]
pub type RETUNE_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `retune_int_int_en` writer - "]
pub type RETUNE_INT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_INT_EN_SPEC, bool, O>;
#[doc = "Field `reserved_15_13` reader - "]
pub type RESERVED_15_13_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete_int_en(&self) -> CMD_COMPLETE_INT_EN_R {
        CMD_COMPLETE_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete_int_en(&self) -> XFER_COMPLETE_INT_EN_R {
        XFER_COMPLETE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt_int_en(&self) -> BLOCK_GAP_EVT_INT_EN_R {
        BLOCK_GAP_EVT_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int_int_en(&self) -> DMA_INT_INT_EN_R {
        DMA_INT_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy_int_en(&self) -> TX_RDY_INT_EN_R {
        TX_RDY_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy_int_en(&self) -> RX_RDY_INT_EN_R {
        RX_RDY_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int_en(&self) -> CARD_INS_INT_EN_R {
        CARD_INS_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int_en(&self) -> CARD_REM_INT_EN_R {
        CARD_REM_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int_int_en(&self) -> CARD_INT_INT_EN_R {
        CARD_INT_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a_int_int_en(&self) -> INT_A_INT_INT_EN_R {
        INT_A_INT_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b_int_int_en(&self) -> INT_B_INT_INT_EN_R {
        INT_B_INT_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c_int_int_en(&self) -> INT_C_INT_INT_EN_R {
        INT_C_INT_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retune_int_int_en(&self) -> RETUNE_INT_INT_EN_R {
        RETUNE_INT_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_15_13(&self) -> RESERVED_15_13_R {
        RESERVED_15_13_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_int_en(&mut self) -> CMD_COMPLETE_INT_EN_W<0> {
        CMD_COMPLETE_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_int_en(&mut self) -> XFER_COMPLETE_INT_EN_W<1> {
        XFER_COMPLETE_INT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap_evt_int_en(&mut self) -> BLOCK_GAP_EVT_INT_EN_W<2> {
        BLOCK_GAP_EVT_INT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_int_int_en(&mut self) -> DMA_INT_INT_EN_W<3> {
        DMA_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rdy_int_en(&mut self) -> TX_RDY_INT_EN_W<4> {
        TX_RDY_INT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rdy_int_en(&mut self) -> RX_RDY_INT_EN_W<5> {
        RX_RDY_INT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins_int_en(&mut self) -> CARD_INS_INT_EN_W<6> {
        CARD_INS_INT_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn card_rem_int_en(&mut self) -> CARD_REM_INT_EN_W<7> {
        CARD_REM_INT_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn card_int_int_en(&mut self) -> CARD_INT_INT_EN_W<8> {
        CARD_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn int_a_int_int_en(&mut self) -> INT_A_INT_INT_EN_W<9> {
        INT_A_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn int_b_int_int_en(&mut self) -> INT_B_INT_INT_EN_W<10> {
        INT_B_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn int_c_int_int_en(&mut self) -> INT_C_INT_INT_EN_W<11> {
        INT_C_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn retune_int_int_en(&mut self) -> RETUNE_INT_INT_EN_W<12> {
        RETUNE_INT_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_normal_int_status_int_en](index.html) module"]
pub struct SD_NORMAL_INT_STATUS_INT_EN_SPEC;
impl crate::RegisterSpec for SD_NORMAL_INT_STATUS_INT_EN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_normal_int_status_int_en::R](R) reader structure"]
impl crate::Readable for SD_NORMAL_INT_STATUS_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_normal_int_status_int_en::W](W) writer structure"]
impl crate::Writable for SD_NORMAL_INT_STATUS_INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_normal_int_status_int_en to value 0"]
impl crate::Resettable for SD_NORMAL_INT_STATUS_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
