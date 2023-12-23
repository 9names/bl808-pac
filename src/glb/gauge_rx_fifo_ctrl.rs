#[doc = "Register `gauge_rx_fifo_ctrl` reader"]
pub struct R(crate::R<GAUGE_RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAUGE_RX_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAUGE_RX_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAUGE_RX_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gauge_rx_fifo_ctrl` writer"]
pub struct W(crate::W<GAUGE_RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAUGE_RX_FIFO_CTRL_SPEC>;
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
impl From<crate::W<GAUGE_RX_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAUGE_RX_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gauge_rx_fifo_flush` writer - "]
pub type GAUGE_RX_FIFO_FLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `gauge_rxo_int_en` reader - "]
pub type GAUGE_RXO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxo_int_en` writer - "]
pub type GAUGE_RXO_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `gauge_rxu_int_en` reader - "]
pub type GAUGE_RXU_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxu_int_en` writer - "]
pub type GAUGE_RXU_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `gauge_rxa_int_en` reader - "]
pub type GAUGE_RXA_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxa_int_en` writer - "]
pub type GAUGE_RXA_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `gauge_rx_drq_en` reader - "]
pub type GAUGE_RX_DRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rx_drq_en` writer - "]
pub type GAUGE_RX_DRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `gauge_rx_data_res` reader - "]
pub type GAUGE_RX_DATA_RES_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rx_data_res` writer - "]
pub type GAUGE_RX_DATA_RES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_ch_en` reader - "]
pub type GAUGE_RX_CH_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rx_ch_en` writer - "]
pub type GAUGE_RX_CH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_9_13` reader - "]
pub type RESERVED_9_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_drq_cnt` reader - "]
pub type GAUGE_RX_DRQ_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_drq_cnt` writer - "]
pub type GAUGE_RX_DRQ_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `gauge_rx_trg_level` reader - "]
pub type GAUGE_RX_TRG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_trg_level` writer - "]
pub type GAUGE_RX_TRG_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_data_mode` reader - "]
pub type GAUGE_RX_DATA_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rx_data_mode` writer - "]
pub type GAUGE_RX_DATA_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAUGE_RX_FIFO_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gauge_rxo_int_en(&self) -> GAUGE_RXO_INT_EN_R {
        GAUGE_RXO_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gauge_rxu_int_en(&self) -> GAUGE_RXU_INT_EN_R {
        GAUGE_RXU_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gauge_rxa_int_en(&self) -> GAUGE_RXA_INT_EN_R {
        GAUGE_RXA_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gauge_rx_drq_en(&self) -> GAUGE_RX_DRQ_EN_R {
        GAUGE_RX_DRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_rx_data_res(&self) -> GAUGE_RX_DATA_RES_R {
        GAUGE_RX_DATA_RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gauge_rx_ch_en(&self) -> GAUGE_RX_CH_EN_R {
        GAUGE_RX_CH_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn reserved_9_13(&self) -> RESERVED_9_13_R {
        RESERVED_9_13_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gauge_rx_drq_cnt(&self) -> GAUGE_RX_DRQ_CNT_R {
        GAUGE_RX_DRQ_CNT_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gauge_rx_trg_level(&self) -> GAUGE_RX_TRG_LEVEL_R {
        GAUGE_RX_TRG_LEVEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gauge_rx_data_mode(&self) -> GAUGE_RX_DATA_MODE_R {
        GAUGE_RX_DATA_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_fifo_flush(&mut self) -> GAUGE_RX_FIFO_FLUSH_W<0> {
        GAUGE_RX_FIFO_FLUSH_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rxo_int_en(&mut self) -> GAUGE_RXO_INT_EN_W<1> {
        GAUGE_RXO_INT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rxu_int_en(&mut self) -> GAUGE_RXU_INT_EN_W<2> {
        GAUGE_RXU_INT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rxa_int_en(&mut self) -> GAUGE_RXA_INT_EN_W<3> {
        GAUGE_RXA_INT_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_drq_en(&mut self) -> GAUGE_RX_DRQ_EN_W<4> {
        GAUGE_RX_DRQ_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_data_res(&mut self) -> GAUGE_RX_DATA_RES_W<5> {
        GAUGE_RX_DATA_RES_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_ch_en(&mut self) -> GAUGE_RX_CH_EN_W<8> {
        GAUGE_RX_CH_EN_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_drq_cnt(&mut self) -> GAUGE_RX_DRQ_CNT_W<14> {
        GAUGE_RX_DRQ_CNT_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_trg_level(&mut self) -> GAUGE_RX_TRG_LEVEL_W<16> {
        GAUGE_RX_TRG_LEVEL_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_rx_data_mode(&mut self) -> GAUGE_RX_DATA_MODE_W<24> {
        GAUGE_RX_DATA_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gauge_rx_fifo_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gauge_rx_fifo_ctrl](index.html) module"]
pub struct GAUGE_RX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for GAUGE_RX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gauge_rx_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for GAUGE_RX_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gauge_rx_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for GAUGE_RX_FIFO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gauge_rx_fifo_ctrl to value 0x0301_0000"]
impl crate::Resettable for GAUGE_RX_FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301_0000;
}
