#[doc = "Register `sd_spi_mode` reader"]
pub struct R(crate::R<SD_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_spi_mode` writer"]
pub struct W(crate::W<SD_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SPI_MODE_SPEC>;
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
impl From<crate::W<SD_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_en` reader - "]
pub type SPI_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_en` writer - "]
pub type SPI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `reserved_7_1` reader - "]
pub type RESERVED_7_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_err_token` reader - "]
pub type SPI_ERR_TOKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_err_token` writer - "]
pub type SPI_ERR_TOKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_SPI_MODE_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_15_13` reader - "]
pub type RESERVED_15_13_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_en(&self) -> SPI_EN_R {
        SPI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn reserved_7_1(&self) -> RESERVED_7_1_R {
        RESERVED_7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn spi_err_token(&self) -> SPI_ERR_TOKEN_R {
        SPI_ERR_TOKEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_15_13(&self) -> RESERVED_15_13_R {
        RESERVED_15_13_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn spi_en(&mut self) -> SPI_EN_W<0> {
        SPI_EN_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn spi_err_token(&mut self) -> SPI_ERR_TOKEN_W<8> {
        SPI_ERR_TOKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_spi_mode](index.html) module"]
pub struct SD_SPI_MODE_SPEC;
impl crate::RegisterSpec for SD_SPI_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_spi_mode::R](R) reader structure"]
impl crate::Readable for SD_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_spi_mode::W](W) writer structure"]
impl crate::Writable for SD_SPI_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_spi_mode to value 0"]
impl crate::Resettable for SD_SPI_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
