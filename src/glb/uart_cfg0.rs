#[doc = "Register `uart_cfg0` reader"]
pub struct R(crate::R<UART_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_cfg0` writer"]
pub struct W(crate::W<UART_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CFG0_SPEC>;
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
impl From<crate::W<UART_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_clk_div` reader - "]
pub type UART_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_clk_div` writer - "]
pub type UART_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `uart_clk_en` reader - "]
pub type UART_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `uart_clk_en` writer - "]
pub type UART_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_5_6` reader - "]
pub type RESERVED_5_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HBN_UART_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reserved_8_21` reader - "]
pub type RESERVED_8_21_R = crate::FieldReader<u16, u16>;
#[doc = "Field `hbn_uart_clk_sel2` reader - "]
pub type HBN_UART_CLK_SEL2_R = crate::BitReader<bool>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `uart2_io_sel` reader - "]
pub type UART2_IO_SEL_R = crate::BitReader<bool>;
#[doc = "Field `uart2_io_sel` writer - "]
pub type UART2_IO_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_25_31` reader - "]
pub type RESERVED_25_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UART_CLK_DIV_R {
        UART_CLK_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reserved_5_6(&self) -> RESERVED_5_6_R {
        RESERVED_5_6_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:21"]
    #[inline(always)]
    pub fn reserved_8_21(&self) -> RESERVED_8_21_R {
        RESERVED_8_21_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel2(&self) -> HBN_UART_CLK_SEL2_R {
        HBN_UART_CLK_SEL2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart2_io_sel(&self) -> UART2_IO_SEL_R {
        UART2_IO_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn reserved_25_31(&self) -> RESERVED_25_31_R {
        RESERVED_25_31_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_div(&mut self) -> UART_CLK_DIV_W<0> {
        UART_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<4> {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_io_sel(&mut self) -> UART2_IO_SEL_W<24> {
        UART2_IO_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cfg0](index.html) module"]
pub struct UART_CFG0_SPEC;
impl crate::RegisterSpec for UART_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_cfg0::R](R) reader structure"]
impl crate::Readable for UART_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_cfg0::W](W) writer structure"]
impl crate::Writable for UART_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_cfg0 to value 0x17"]
impl crate::Resettable for UART_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x17;
}
