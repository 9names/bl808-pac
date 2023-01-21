#[doc = "Register `spi_cfg0` reader"]
pub struct R(crate::R<SPI_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_cfg0` writer"]
pub struct W(crate::W<SPI_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG0_SPEC>;
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
impl From<crate::W<SPI_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_clk_div` reader - "]
pub type SPI_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_clk_div` writer - "]
pub type SPI_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_clk_en` reader - "]
pub type SPI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_clk_en` writer - "]
pub type SPI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG0_SPEC, bool, O>;
#[doc = "Field `spi_clk_sel` reader - "]
pub type SPI_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `spi_clk_sel` writer - "]
pub type SPI_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_swap_set` reader - "]
pub type SPI_SWAP_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_swap_set` writer - "]
pub type SPI_SWAP_SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_20_31` reader - "]
pub type RESERVED_20_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_clk_sel(&self) -> SPI_CLK_SEL_R {
        SPI_CLK_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn spi_swap_set(&self) -> SPI_SWAP_SET_R {
        SPI_SWAP_SET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn reserved_20_31(&self) -> RESERVED_20_31_R {
        RESERVED_20_31_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W<0> {
        SPI_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W<8> {
        SPI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_sel(&mut self) -> SPI_CLK_SEL_W<9> {
        SPI_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn spi_swap_set(&mut self) -> SPI_SWAP_SET_W<16> {
        SPI_SWAP_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg0](index.html) module"]
pub struct SPI_CFG0_SPEC;
impl crate::RegisterSpec for SPI_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cfg0::R](R) reader structure"]
impl crate::Readable for SPI_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cfg0::W](W) writer structure"]
impl crate::Writable for SPI_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_cfg0 to value 0x0103"]
impl crate::Resettable for SPI_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103;
}
