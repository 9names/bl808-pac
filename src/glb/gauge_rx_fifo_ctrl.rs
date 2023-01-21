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
impl W {
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
#[doc = "`reset()` method sets gauge_rx_fifo_ctrl to value 0"]
impl crate::Resettable for GAUGE_RX_FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
