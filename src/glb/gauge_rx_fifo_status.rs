#[doc = "Register `gauge_rx_fifo_status` reader"]
pub struct R(crate::R<GAUGE_RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAUGE_RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAUGE_RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAUGE_RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gauge_rx_fifo_status` writer"]
pub struct W(crate::W<GAUGE_RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAUGE_RX_FIFO_STATUS_SPEC>;
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
impl From<crate::W<GAUGE_RX_FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAUGE_RX_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0` reader - "]
pub type RESERVED_0_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxo_int` reader - "]
pub type GAUGE_RXO_INT_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxu_int` reader - "]
pub type GAUGE_RXU_INT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `gauge_rxa_int` reader - "]
pub type GAUGE_RXA_INT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_5_15` reader - "]
pub type RESERVED_5_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gauge_rxa_cnt` reader - "]
pub type GAUGE_RXA_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_rxa` reader - "]
pub type GAUGE_RXA_R = crate::BitReader<bool>;
#[doc = "Field `reserved_25_31` reader - "]
pub type RESERVED_25_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> RESERVED_0_R {
        RESERVED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gauge_rxo_int(&self) -> GAUGE_RXO_INT_R {
        GAUGE_RXO_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gauge_rxu_int(&self) -> GAUGE_RXU_INT_R {
        GAUGE_RXU_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gauge_rxa_int(&self) -> GAUGE_RXA_INT_R {
        GAUGE_RXA_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn reserved_5_15(&self) -> RESERVED_5_15_R {
        RESERVED_5_15_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gauge_rxa_cnt(&self) -> GAUGE_RXA_CNT_R {
        GAUGE_RXA_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gauge_rxa(&self) -> GAUGE_RXA_R {
        GAUGE_RXA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn reserved_25_31(&self) -> RESERVED_25_31_R {
        RESERVED_25_31_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gauge_rx_fifo_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gauge_rx_fifo_status](index.html) module"]
pub struct GAUGE_RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for GAUGE_RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gauge_rx_fifo_status::R](R) reader structure"]
impl crate::Readable for GAUGE_RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gauge_rx_fifo_status::W](W) writer structure"]
impl crate::Writable for GAUGE_RX_FIFO_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gauge_rx_fifo_status to value 0"]
impl crate::Resettable for GAUGE_RX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
