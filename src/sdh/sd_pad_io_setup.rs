#[doc = "Register `sd_pad_io_setup` reader"]
pub struct R(crate::R<SD_PAD_IO_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_PAD_IO_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_PAD_IO_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_PAD_IO_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_pad_io_setup` writer"]
pub struct W(crate::W<SD_PAD_IO_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_PAD_IO_SETUP_SPEC>;
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
impl From<crate::W<SD_PAD_IO_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_PAD_IO_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `async_io_en` reader - "]
pub type ASYNC_IO_EN_R = crate::BitReader<bool>;
#[doc = "Field `async_io_en` writer - "]
pub type ASYNC_IO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_PAD_IO_SETUP_SPEC, bool, O>;
#[doc = "Field `inand_sel` reader - "]
pub type INAND_SEL_R = crate::BitReader<bool>;
#[doc = "Field `inand_sel` writer - "]
pub type INAND_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_PAD_IO_SETUP_SPEC, bool, O>;
#[doc = "Field `reserved_15_2` reader - "]
pub type RESERVED_15_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `eco_reg` reader - "]
pub type ECO_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `eco_reg` writer - "]
pub type ECO_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_PAD_IO_SETUP_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_31_24` reader - "]
pub type RESERVED_31_24_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn async_io_en(&self) -> ASYNC_IO_EN_R {
        ASYNC_IO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inand_sel(&self) -> INAND_SEL_R {
        INAND_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn reserved_15_2(&self) -> RESERVED_15_2_R {
        RESERVED_15_2_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn eco_reg(&self) -> ECO_REG_R {
        ECO_REG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reserved_31_24(&self) -> RESERVED_31_24_R {
        RESERVED_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn async_io_en(&mut self) -> ASYNC_IO_EN_W<0> {
        ASYNC_IO_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn inand_sel(&mut self) -> INAND_SEL_W<1> {
        INAND_SEL_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn eco_reg(&mut self) -> ECO_REG_W<16> {
        ECO_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD I/O Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_pad_io_setup](index.html) module"]
pub struct SD_PAD_IO_SETUP_SPEC;
impl crate::RegisterSpec for SD_PAD_IO_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_pad_io_setup::R](R) reader structure"]
impl crate::Readable for SD_PAD_IO_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_pad_io_setup::W](W) writer structure"]
impl crate::Writable for SD_PAD_IO_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_pad_io_setup to value 0x02"]
impl crate::Resettable for SD_PAD_IO_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
