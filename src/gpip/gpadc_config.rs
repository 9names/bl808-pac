#[doc = "Register `gpadc_config` reader"]
pub struct R(crate::R<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_config` writer"]
pub struct W(crate::W<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_CONFIG_SPEC>;
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
impl From<crate::W<GPADC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_dma_en` reader - "]
pub type GPADC_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_dma_en` writer - "]
pub type GPADC_DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_clr` writer - "]
pub type GPADC_FIFO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_ne` reader - "]
pub type GPADC_FIFO_NE_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_full` reader - "]
pub type GPADC_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy` reader - "]
pub type GPADC_RDY_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_overrun` reader - "]
pub type GPADC_FIFO_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_underrun` reader - "]
pub type GPADC_FIFO_UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy_clr` reader - "]
pub type GPADC_RDY_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy_clr` writer - "]
pub type GPADC_RDY_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_overrun_clr` reader - "]
pub type GPADC_FIFO_OVERRUN_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_overrun_clr` writer - "]
pub type GPADC_FIFO_OVERRUN_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GPADC_DMA_EN_R {
        GPADC_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GPADC_FIFO_NE_R {
        GPADC_FIFO_NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GPADC_FIFO_FULL_R {
        GPADC_FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GPADC_RDY_R {
        GPADC_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GPADC_FIFO_OVERRUN_R {
        GPADC_FIFO_OVERRUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GPADC_FIFO_UNDERRUN_R {
        GPADC_FIFO_UNDERRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GPADC_RDY_CLR_R {
        GPADC_RDY_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GPADC_FIFO_OVERRUN_CLR_R {
        GPADC_FIFO_OVERRUN_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dma_en(&mut self) -> GPADC_DMA_EN_W<0> {
        GPADC_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_clr(&mut self) -> GPADC_FIFO_CLR_W<1> {
        GPADC_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_rdy_clr(&mut self) -> GPADC_RDY_CLR_W<8> {
        GPADC_RDY_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GPADC_FIFO_OVERRUN_CLR_W<9> {
        GPADC_FIFO_OVERRUN_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_config](index.html) module"]
pub struct GPADC_CONFIG_SPEC;
impl crate::RegisterSpec for GPADC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_config::R](R) reader structure"]
impl crate::Readable for GPADC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_config::W](W) writer structure"]
impl crate::Writable for GPADC_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GPADC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
