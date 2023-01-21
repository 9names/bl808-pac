#[doc = "Register `gpio_cfg143` reader"]
pub struct R(crate::R<GPIO_CFG143_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG143_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG143_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG143_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg143` writer"]
pub struct W(crate::W<GPIO_CFG143_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG143_SPEC>;
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
impl From<crate::W<GPIO_CFG143_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG143_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_gpio_dma_tx_en` reader - "]
pub type CR_GPIO_DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_dma_tx_en` writer - "]
pub type CR_GPIO_DMA_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `cr_gpio_dma_out_sel_latch` reader - "]
pub type CR_GPIO_DMA_OUT_SEL_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_dma_out_sel_latch` writer - "]
pub type CR_GPIO_DMA_OUT_SEL_LATCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_dma_tx_en(&self) -> CR_GPIO_DMA_TX_EN_R {
        CR_GPIO_DMA_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_gpio_dma_out_sel_latch(&self) -> CR_GPIO_DMA_OUT_SEL_LATCH_R {
        CR_GPIO_DMA_OUT_SEL_LATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_dma_tx_en(&mut self) -> CR_GPIO_DMA_TX_EN_W<0> {
        CR_GPIO_DMA_TX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_dma_out_sel_latch(&mut self) -> CR_GPIO_DMA_OUT_SEL_LATCH_W<1> {
        CR_GPIO_DMA_OUT_SEL_LATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg143\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg143](index.html) module"]
pub struct GPIO_CFG143_SPEC;
impl crate::RegisterSpec for GPIO_CFG143_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg143::R](R) reader structure"]
impl crate::Readable for GPIO_CFG143_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg143::W](W) writer structure"]
impl crate::Writable for GPIO_CFG143_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg143 to value 0"]
impl crate::Resettable for GPIO_CFG143_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
