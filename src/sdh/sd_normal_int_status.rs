#[doc = "Register `sd_normal_int_status` reader"]
pub struct R(crate::R<SD_NORMAL_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_NORMAL_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_NORMAL_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_NORMAL_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_normal_int_status` writer"]
pub struct W(crate::W<SD_NORMAL_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_NORMAL_INT_STATUS_SPEC>;
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
impl From<crate::W<SD_NORMAL_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_NORMAL_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_complete` reader - "]
pub type CMD_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `cmd_complete` writer - "]
pub type CMD_COMPLETE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `xfer_complete` reader - "]
pub type XFER_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `xfer_complete` writer - "]
pub type XFER_COMPLETE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `block_gap_evt` reader - "]
pub type BLOCK_GAP_EVT_R = crate::BitReader<bool>;
#[doc = "Field `block_gap_evt` writer - "]
pub type BLOCK_GAP_EVT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `dma_int` reader - "]
pub type DMA_INT_R = crate::BitReader<bool>;
#[doc = "Field `dma_int` writer - "]
pub type DMA_INT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `tx_rdy` reader - "]
pub type TX_RDY_R = crate::BitReader<bool>;
#[doc = "Field `tx_rdy` writer - "]
pub type TX_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `rx_rdy` reader - "]
pub type RX_RDY_R = crate::BitReader<bool>;
#[doc = "Field `rx_rdy` writer - "]
pub type RX_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `card_ins_int` reader - "]
pub type CARD_INS_INT_R = crate::BitReader<bool>;
#[doc = "Field `card_ins_int` writer - "]
pub type CARD_INS_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `card_rem_int` reader - "]
pub type CARD_REM_INT_R = crate::BitReader<bool>;
#[doc = "Field `card_rem_int` writer - "]
pub type CARD_REM_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `card_int` reader - "]
pub type CARD_INT_R = crate::BitReader<bool>;
#[doc = "Field `int_a` reader - "]
pub type INT_A_R = crate::BitReader<bool>;
#[doc = "Field `int_a` writer - "]
pub type INT_A_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `int_b` reader - "]
pub type INT_B_R = crate::BitReader<bool>;
#[doc = "Field `int_b` writer - "]
pub type INT_B_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `int_c` reader - "]
pub type INT_C_R = crate::BitReader<bool>;
#[doc = "Field `int_c` writer - "]
pub type INT_C_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `retuning_int` reader - "]
pub type RETUNING_INT_R = crate::BitReader<bool>;
#[doc = "Field `retuning_int` writer - "]
pub type RETUNING_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_NORMAL_INT_STATUS_SPEC, bool, O>;
#[doc = "Field `reserved_14_13` reader - "]
pub type RESERVED_14_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `err_int` reader - "]
pub type ERR_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CMD_COMPLETE_R {
        CMD_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XFER_COMPLETE_R {
        XFER_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt(&self) -> BLOCK_GAP_EVT_R {
        BLOCK_GAP_EVT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int(&self) -> DMA_INT_R {
        DMA_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy(&self) -> TX_RDY_R {
        TX_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RX_RDY_R {
        RX_RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int(&self) -> CARD_INS_INT_R {
        CARD_INS_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int(&self) -> CARD_REM_INT_R {
        CARD_REM_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a(&self) -> INT_A_R {
        INT_A_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b(&self) -> INT_B_R {
        INT_B_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c(&self) -> INT_C_R {
        INT_C_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retuning_int(&self) -> RETUNING_INT_R {
        RETUNING_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn reserved_14_13(&self) -> RESERVED_14_13_R {
        RESERVED_14_13_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CMD_COMPLETE_W<0> {
        CMD_COMPLETE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete(&mut self) -> XFER_COMPLETE_W<1> {
        XFER_COMPLETE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap_evt(&mut self) -> BLOCK_GAP_EVT_W<2> {
        BLOCK_GAP_EVT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_int(&mut self) -> DMA_INT_W<3> {
        DMA_INT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rdy(&mut self) -> TX_RDY_W<4> {
        TX_RDY_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rdy(&mut self) -> RX_RDY_W<5> {
        RX_RDY_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins_int(&mut self) -> CARD_INS_INT_W<6> {
        CARD_INS_INT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn card_rem_int(&mut self) -> CARD_REM_INT_W<7> {
        CARD_REM_INT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn int_a(&mut self) -> INT_A_W<9> {
        INT_A_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn int_b(&mut self) -> INT_B_W<10> {
        INT_B_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn int_c(&mut self) -> INT_C_W<11> {
        INT_C_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn retuning_int(&mut self) -> RETUNING_INT_W<12> {
        RETUNING_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_normal_int_status](index.html) module"]
pub struct SD_NORMAL_INT_STATUS_SPEC;
impl crate::RegisterSpec for SD_NORMAL_INT_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_normal_int_status::R](R) reader structure"]
impl crate::Readable for SD_NORMAL_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_normal_int_status::W](W) writer structure"]
impl crate::Writable for SD_NORMAL_INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_normal_int_status to value 0x10"]
impl crate::Resettable for SD_NORMAL_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
