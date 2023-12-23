#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub struct R(crate::R<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_tx_fifo_status` writer"]
pub struct W(crate::W<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_TX_FIFO_STATUS_SPEC>;
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
impl From<crate::W<GPDAC_TX_FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_empty` reader - "]
pub type TX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_full` reader - "]
pub type TX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `tx_cs` reader - "]
pub type TX_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `txfifordptr` reader - "]
pub type TXFIFORDPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `txfifowrptr` reader - "]
pub type TXFIFOWRPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_10_31` reader - "]
pub type RESERVED_10_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TX_FIFO_FULL_R {
        TX_FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TX_CS_R {
        TX_CS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn txfifordptr(&self) -> TXFIFORDPTR_R {
        TXFIFORDPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn txfifowrptr(&self) -> TXFIFOWRPTR_R {
        TXFIFOWRPTR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn reserved_10_31(&self) -> RESERVED_10_31_R {
        RESERVED_10_31_R::new((self.bits >> 10) & 0x003f_ffff)
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
#[doc = "gpdac_tx_fifo_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_tx_fifo_status](index.html) module"]
pub struct GPDAC_TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for GPDAC_TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_tx_fifo_status::R](R) reader structure"]
impl crate::Readable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_tx_fifo_status::W](W) writer structure"]
impl crate::Writable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0x80"]
impl crate::Resettable for GPDAC_TX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
