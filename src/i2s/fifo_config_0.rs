#[doc = "Register `fifo_config_0` reader"]
pub struct R(crate::R<FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fifo_config_0` writer"]
pub struct W(crate::W<FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONFIG_0_SPEC>;
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
impl From<crate::W<FIFO_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `transmit_dma` reader - Enable signal of transmit DMA interface"]
pub use RECEIVE_DMA_R as TRANSMIT_DMA_R;
#[doc = "Field `transmit_dma` writer - Enable signal of transmit DMA interface"]
pub use RECEIVE_DMA_W as TRANSMIT_DMA_W;
#[doc = "Field `receive_dma` reader - Enable signal of receive DMA interface"]
pub type RECEIVE_DMA_R = crate::BitReader<DMA_ENABLE_A>;
#[doc = "Enable signal of receive DMA interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_ENABLE_A {
    #[doc = "1: Enable DMA interface"]
    ENABLE = 1,
    #[doc = "0: Disable DMA interface"]
    DISABLE = 0,
}
impl From<DMA_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENABLE_A {
        match self.bits {
            true => DMA_ENABLE_A::ENABLE,
            false => DMA_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENABLE_A::DISABLE
    }
}
#[doc = "Field `receive_dma` writer - Enable signal of receive DMA interface"]
pub type RECEIVE_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_0_SPEC, DMA_ENABLE_A, O>;
impl<'a, const O: u8> RECEIVE_DMA_W<'a, O> {
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::ENABLE)
    }
    #[doc = "Disable DMA interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `transmit_clear` writer - Clears transmit FIFO overflow and underflow flags"]
pub use RECEIVE_CLEAR_W as TRANSMIT_CLEAR_W;
#[doc = "Clears receive FIFO overflow and underflow flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG_CLEAR_AW {
    #[doc = "1: Write 1 to clear fifo flags"]
    CLEAR = 1,
}
impl From<FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_clear` writer - Clears receive FIFO overflow and underflow flags"]
pub type RECEIVE_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_0_SPEC, FLAG_CLEAR_AW, O>;
impl<'a, const O: u8> RECEIVE_CLEAR_W<'a, O> {
    #[doc = "Write 1 to clear fifo flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLAG_CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `transmit_overflow` reader - Transmit FIFO overflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_OVERFLOW_R as TRANSMIT_OVERFLOW_R;
#[doc = "Field `transmit_underflow` reader - Transmit FIFO underflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_UNDERFLOW_R as TRANSMIT_UNDERFLOW_R;
#[doc = "Field `receive_overflow` reader - Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`."]
pub type RECEIVE_OVERFLOW_R = crate::BitReader<HAS_OVERFLOW_A>;
#[doc = "Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAS_OVERFLOW_A {
    #[doc = "0: No FIFO buffer overflow"]
    NOT_OVERFLOW = 0,
    #[doc = "1: Has FIFO buffer overflow"]
    OVERFLOW = 1,
}
impl From<HAS_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: HAS_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE_OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAS_OVERFLOW_A {
        match self.bits {
            false => HAS_OVERFLOW_A::NOT_OVERFLOW,
            true => HAS_OVERFLOW_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOW`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        *self == HAS_OVERFLOW_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == HAS_OVERFLOW_A::OVERFLOW
    }
}
#[doc = "Field `receive_underflow` reader - Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`."]
pub type RECEIVE_UNDERFLOW_R = crate::BitReader<HAS_UNDERFLOW_A>;
#[doc = "Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAS_UNDERFLOW_A {
    #[doc = "0: No FIFO buffer underflow"]
    NOT_UNDERFLOW = 0,
    #[doc = "1: Has FIFO buffer underflow"]
    UNDERFLOW = 1,
}
impl From<HAS_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: HAS_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE_UNDERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAS_UNDERFLOW_A {
        match self.bits {
            false => HAS_UNDERFLOW_A::NOT_UNDERFLOW,
            true => HAS_UNDERFLOW_A::UNDERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_not_underflow(&self) -> bool {
        *self == HAS_UNDERFLOW_A::NOT_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == HAS_UNDERFLOW_A::UNDERFLOW
    }
}
#[doc = "Field `merge_left_right` reader - "]
pub type MERGE_LEFT_RIGHT_R = crate::BitReader<bool>;
#[doc = "Field `merge_left_right` writer - "]
pub type MERGE_LEFT_RIGHT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `swap_left_right` reader - "]
pub type SWAP_LEFT_RIGHT_R = crate::BitReader<bool>;
#[doc = "Field `swap_left_right` writer - "]
pub type SWAP_LEFT_RIGHT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `left_justified` reader - "]
pub type LEFT_JUSTIFIED_R = crate::BitReader<bool>;
#[doc = "Field `left_justified` writer - "]
pub type LEFT_JUSTIFIED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CONFIG_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable signal of transmit DMA interface"]
    #[inline(always)]
    pub fn transmit_dma(&self) -> TRANSMIT_DMA_R {
        TRANSMIT_DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable signal of receive DMA interface"]
    #[inline(always)]
    pub fn receive_dma(&self) -> RECEIVE_DMA_R {
        RECEIVE_DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO overflow flag\n\n Can be cleared using `transmit_clear`."]
    #[inline(always)]
    pub fn transmit_overflow(&self) -> TRANSMIT_OVERFLOW_R {
        TRANSMIT_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO underflow flag\n\n Can be cleared using `transmit_clear`."]
    #[inline(always)]
    pub fn transmit_underflow(&self) -> TRANSMIT_UNDERFLOW_R {
        TRANSMIT_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`."]
    #[inline(always)]
    pub fn receive_overflow(&self) -> RECEIVE_OVERFLOW_R {
        RECEIVE_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`."]
    #[inline(always)]
    pub fn receive_underflow(&self) -> RECEIVE_UNDERFLOW_R {
        RECEIVE_UNDERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn merge_left_right(&self) -> MERGE_LEFT_RIGHT_R {
        MERGE_LEFT_RIGHT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swap_left_right(&self) -> SWAP_LEFT_RIGHT_R {
        SWAP_LEFT_RIGHT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn left_justified(&self) -> LEFT_JUSTIFIED_R {
        LEFT_JUSTIFIED_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signal of transmit DMA interface"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_dma(&mut self) -> TRANSMIT_DMA_W<0> {
        TRANSMIT_DMA_W::new(self)
    }
    #[doc = "Bit 1 - Enable signal of receive DMA interface"]
    #[inline(always)]
    #[must_use]
    pub fn receive_dma(&mut self) -> RECEIVE_DMA_W<1> {
        RECEIVE_DMA_W::new(self)
    }
    #[doc = "Bit 2 - Clears transmit FIFO overflow and underflow flags"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_clear(&mut self) -> TRANSMIT_CLEAR_W<2> {
        TRANSMIT_CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Clears receive FIFO overflow and underflow flags"]
    #[inline(always)]
    #[must_use]
    pub fn receive_clear(&mut self) -> RECEIVE_CLEAR_W<3> {
        RECEIVE_CLEAR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn merge_left_right(&mut self) -> MERGE_LEFT_RIGHT_W<8> {
        MERGE_LEFT_RIGHT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn swap_left_right(&mut self) -> SWAP_LEFT_RIGHT_W<9> {
        SWAP_LEFT_RIGHT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn left_justified(&mut self) -> LEFT_JUSTIFIED_W<10> {
        LEFT_JUSTIFIED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_config_0](index.html) module"]
pub struct FIFO_CONFIG_0_SPEC;
impl crate::RegisterSpec for FIFO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_config_0::R](R) reader structure"]
impl crate::Readable for FIFO_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_config_0::W](W) writer structure"]
impl crate::Writable for FIFO_CONFIG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fifo_config_0 to value 0"]
impl crate::Resettable for FIFO_CONFIG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
