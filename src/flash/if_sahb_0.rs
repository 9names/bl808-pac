#[doc = "Register `if_sahb_0` reader"]
pub struct R(crate::R<IF_SAHB_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SAHB_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SAHB_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SAHB_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `if_sahb_0` writer"]
pub struct W(crate::W<IF_SAHB_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SAHB_0_SPEC>;
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
impl From<crate::W<IF_SAHB_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SAHB_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `if_busy` reader - "]
pub type IF_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `if_0_trig` reader - "]
pub type IF_0_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `if_0_trig` writer - "]
pub type IF_0_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_dat_byte` reader - "]
pub type IF_0_DAT_BYTE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `if_0_dat_byte` writer - "]
pub type IF_0_DAT_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `if_0_dmy_byte` reader - "]
pub type IF_0_DMY_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `if_0_dmy_byte` writer - "]
pub type IF_0_DMY_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `if_0_adr_byte` reader - "]
pub type IF_0_ADR_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `if_0_adr_byte` writer - "]
pub type IF_0_ADR_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `if_0_cmd_byte` reader - "]
pub type IF_0_CMD_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `if_0_cmd_byte` writer - "]
pub type IF_0_CMD_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `if_0_dat_rw` reader - "]
pub type IF_0_DAT_RW_R = crate::BitReader<bool>;
#[doc = "Field `if_0_dat_rw` writer - "]
pub type IF_0_DAT_RW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_dat_en` reader - "]
pub type IF_0_DAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `if_0_dat_en` writer - "]
pub type IF_0_DAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_dmy_en` reader - "]
pub type IF_0_DMY_EN_R = crate::BitReader<bool>;
#[doc = "Field `if_0_dmy_en` writer - "]
pub type IF_0_DMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_adr_en` reader - "]
pub type IF_0_ADR_EN_R = crate::BitReader<bool>;
#[doc = "Field `if_0_adr_en` writer - "]
pub type IF_0_ADR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_cmd_en` reader - "]
pub type IF_0_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `if_0_cmd_en` writer - "]
pub type IF_0_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
#[doc = "Field `if_0_spi_mode` reader - "]
pub type IF_0_SPI_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `if_0_spi_mode` writer - "]
pub type IF_0_SPI_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_SAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `if_0_qpi_mode_en` reader - "]
pub type IF_0_QPI_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `if_0_qpi_mode_en` writer - "]
pub type IF_0_QPI_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SAHB_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn if_busy(&self) -> IF_BUSY_R {
        IF_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn if_0_trig(&self) -> IF_0_TRIG_R {
        IF_0_TRIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn if_0_dat_byte(&self) -> IF_0_DAT_BYTE_R {
        IF_0_DAT_BYTE_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn if_0_dmy_byte(&self) -> IF_0_DMY_BYTE_R {
        IF_0_DMY_BYTE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn if_0_adr_byte(&self) -> IF_0_ADR_BYTE_R {
        IF_0_ADR_BYTE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn if_0_cmd_byte(&self) -> IF_0_CMD_BYTE_R {
        IF_0_CMD_BYTE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn if_0_dat_rw(&self) -> IF_0_DAT_RW_R {
        IF_0_DAT_RW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn if_0_dat_en(&self) -> IF_0_DAT_EN_R {
        IF_0_DAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn if_0_dmy_en(&self) -> IF_0_DMY_EN_R {
        IF_0_DMY_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn if_0_adr_en(&self) -> IF_0_ADR_EN_R {
        IF_0_ADR_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn if_0_cmd_en(&self) -> IF_0_CMD_EN_R {
        IF_0_CMD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn if_0_spi_mode(&self) -> IF_0_SPI_MODE_R {
        IF_0_SPI_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn if_0_qpi_mode_en(&self) -> IF_0_QPI_MODE_EN_R {
        IF_0_QPI_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_trig(&mut self) -> IF_0_TRIG_W<1> {
        IF_0_TRIG_W::new(self)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_dat_byte(&mut self) -> IF_0_DAT_BYTE_W<2> {
        IF_0_DAT_BYTE_W::new(self)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_dmy_byte(&mut self) -> IF_0_DMY_BYTE_W<12> {
        IF_0_DMY_BYTE_W::new(self)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_adr_byte(&mut self) -> IF_0_ADR_BYTE_W<17> {
        IF_0_ADR_BYTE_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_cmd_byte(&mut self) -> IF_0_CMD_BYTE_W<20> {
        IF_0_CMD_BYTE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_dat_rw(&mut self) -> IF_0_DAT_RW_W<23> {
        IF_0_DAT_RW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_dat_en(&mut self) -> IF_0_DAT_EN_W<24> {
        IF_0_DAT_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_dmy_en(&mut self) -> IF_0_DMY_EN_W<25> {
        IF_0_DMY_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_adr_en(&mut self) -> IF_0_ADR_EN_W<26> {
        IF_0_ADR_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_cmd_en(&mut self) -> IF_0_CMD_EN_W<27> {
        IF_0_CMD_EN_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_spi_mode(&mut self) -> IF_0_SPI_MODE_W<28> {
        IF_0_SPI_MODE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn if_0_qpi_mode_en(&mut self) -> IF_0_QPI_MODE_EN_W<31> {
        IF_0_QPI_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_sahb_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_sahb_0](index.html) module"]
pub struct IF_SAHB_0_SPEC;
impl crate::RegisterSpec for IF_SAHB_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_sahb_0::R](R) reader structure"]
impl crate::Readable for IF_SAHB_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_sahb_0::W](W) writer structure"]
impl crate::Writable for IF_SAHB_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets if_sahb_0 to value 0x0d04_03fc"]
impl crate::Resettable for IF_SAHB_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d04_03fc;
}
