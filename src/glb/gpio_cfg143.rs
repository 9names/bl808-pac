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
#[doc = "Field `gpio_tx_fifo_clr` writer - "]
pub type GPIO_TX_FIFO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `gpio_tx_end_clr` writer - "]
pub type GPIO_TX_END_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `gpio_tx_fifo_overflow` reader - "]
pub type GPIO_TX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `gpio_tx_fifo_underflow` reader - "]
pub type GPIO_TX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_dma_park_value` reader - "]
pub type CR_GPIO_DMA_PARK_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_dma_park_value` writer - "]
pub type CR_GPIO_DMA_PARK_VALUE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `gpio_tx_fifo_cnt` reader - "]
pub type GPIO_TX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_gpio_tx_fifo_th` reader - "]
pub type CR_GPIO_TX_FIFO_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_gpio_tx_fifo_th` writer - "]
pub type CR_GPIO_TX_FIFO_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG143_SPEC, u8, u8, 7, O>;
#[doc = "Field `cr_gpio_tx_end_mask` reader - "]
pub type CR_GPIO_TX_END_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_end_mask` writer - "]
pub type CR_GPIO_TX_END_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `cr_gpio_tx_fifo_mask` reader - "]
pub type CR_GPIO_TX_FIFO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_fifo_mask` writer - "]
pub type CR_GPIO_TX_FIFO_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `cr_gpio_tx_fer_mask` reader - "]
pub type CR_GPIO_TX_FER_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_fer_mask` writer - "]
pub type CR_GPIO_TX_FER_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `r_gpio_tx_end_int` reader - "]
pub type R_GPIO_TX_END_INT_R = crate::BitReader<bool>;
#[doc = "Field `r_gpio_tx_fifo_int` reader - "]
pub type R_GPIO_TX_FIFO_INT_R = crate::BitReader<bool>;
#[doc = "Field `r_gpio_tx_fer_int` reader - "]
pub type R_GPIO_TX_FER_INT_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_end_en` reader - "]
pub type CR_GPIO_TX_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_end_en` writer - "]
pub type CR_GPIO_TX_END_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `cr_gpio_tx_fifo_en` reader - "]
pub type CR_GPIO_TX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_fifo_en` writer - "]
pub type CR_GPIO_TX_FIFO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG143_SPEC, bool, O>;
#[doc = "Field `cr_gpio_tx_fer_en` reader - "]
pub type CR_GPIO_TX_FER_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_fer_en` writer - "]
pub type CR_GPIO_TX_FER_EN_W<'a, const O: u8> =
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
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_tx_fifo_overflow(&self) -> GPIO_TX_FIFO_OVERFLOW_R {
        GPIO_TX_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_tx_fifo_underflow(&self) -> GPIO_TX_FIFO_UNDERFLOW_R {
        GPIO_TX_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_gpio_dma_park_value(&self) -> CR_GPIO_DMA_PARK_VALUE_R {
        CR_GPIO_DMA_PARK_VALUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_tx_fifo_cnt(&self) -> GPIO_TX_FIFO_CNT_R {
        GPIO_TX_FIFO_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_th(&self) -> CR_GPIO_TX_FIFO_TH_R {
        CR_GPIO_TX_FIFO_TH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_gpio_tx_end_mask(&self) -> CR_GPIO_TX_END_MASK_R {
        CR_GPIO_TX_END_MASK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_mask(&self) -> CR_GPIO_TX_FIFO_MASK_R {
        CR_GPIO_TX_FIFO_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_gpio_tx_fer_mask(&self) -> CR_GPIO_TX_FER_MASK_R {
        CR_GPIO_TX_FER_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_gpio_tx_end_int(&self) -> R_GPIO_TX_END_INT_R {
        R_GPIO_TX_END_INT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_gpio_tx_fifo_int(&self) -> R_GPIO_TX_FIFO_INT_R {
        R_GPIO_TX_FIFO_INT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_gpio_tx_fer_int(&self) -> R_GPIO_TX_FER_INT_R {
        R_GPIO_TX_FER_INT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_gpio_tx_end_en(&self) -> CR_GPIO_TX_END_EN_R {
        CR_GPIO_TX_END_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_en(&self) -> CR_GPIO_TX_FIFO_EN_R {
        CR_GPIO_TX_FIFO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_gpio_tx_fer_en(&self) -> CR_GPIO_TX_FER_EN_R {
        CR_GPIO_TX_FER_EN_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_tx_fifo_clr(&mut self) -> GPIO_TX_FIFO_CLR_W<2> {
        GPIO_TX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_tx_end_clr(&mut self) -> GPIO_TX_END_CLR_W<3> {
        GPIO_TX_END_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_dma_park_value(&mut self) -> CR_GPIO_DMA_PARK_VALUE_W<7> {
        CR_GPIO_DMA_PARK_VALUE_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_fifo_th(&mut self) -> CR_GPIO_TX_FIFO_TH_W<16> {
        CR_GPIO_TX_FIFO_TH_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_end_mask(&mut self) -> CR_GPIO_TX_END_MASK_W<23> {
        CR_GPIO_TX_END_MASK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_fifo_mask(&mut self) -> CR_GPIO_TX_FIFO_MASK_W<24> {
        CR_GPIO_TX_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_fer_mask(&mut self) -> CR_GPIO_TX_FER_MASK_W<25> {
        CR_GPIO_TX_FER_MASK_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_end_en(&mut self) -> CR_GPIO_TX_END_EN_W<29> {
        CR_GPIO_TX_END_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_fifo_en(&mut self) -> CR_GPIO_TX_FIFO_EN_W<30> {
        CR_GPIO_TX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_fer_en(&mut self) -> CR_GPIO_TX_FER_EN_W<31> {
        CR_GPIO_TX_FER_EN_W::new(self)
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
#[doc = "`reset()` method sets gpio_cfg143 to value 0xe380_8000"]
impl crate::Resettable for GPIO_CFG143_SPEC {
    const RESET_VALUE: Self::Ux = 0xe380_8000;
}
