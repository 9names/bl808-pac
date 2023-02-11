#[doc = "Register `uhs_psram_configure` reader"]
pub struct R(crate::R<UHS_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PSRAM_CONFIGURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PSRAM_CONFIGURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PSRAM_CONFIGURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_psram_configure` writer"]
pub struct W(crate::W<UHS_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PSRAM_CONFIGURE_SPEC>;
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
impl From<crate::W<UHS_PSRAM_CONFIGURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PSRAM_CONFIGURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_uhs_latency` reader - "]
pub type REG_UHS_LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_uhs_latency` writer - "]
pub type REG_UHS_LATENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `reg_uhs_drive_st` reader - "]
pub type REG_UHS_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_uhs_drive_st` writer - "]
pub type REG_UHS_DRIVE_ST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PSRAM_CONFIGURE_SPEC, u8, u8, 4, O>;
#[doc = "Field `reg_uhs_bl_16` reader - "]
pub type REG_UHS_BL_16_R = crate::BitReader<bool>;
#[doc = "Field `reg_uhs_bl_16` writer - "]
pub type REG_UHS_BL_16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_uhs_bl_32` reader - "]
pub type REG_UHS_BL_32_R = crate::BitReader<bool>;
#[doc = "Field `reg_uhs_bl_32` writer - "]
pub type REG_UHS_BL_32_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_uhs_bl_64` reader - "]
pub type REG_UHS_BL_64_R = crate::BitReader<bool>;
#[doc = "Field `reg_uhs_bl_64` writer - "]
pub type REG_UHS_BL_64_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn reg_uhs_latency(&self) -> REG_UHS_LATENCY_R {
        REG_UHS_LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_uhs_drive_st(&self) -> REG_UHS_DRIVE_ST_R {
        REG_UHS_DRIVE_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_uhs_bl_16(&self) -> REG_UHS_BL_16_R {
        REG_UHS_BL_16_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_uhs_bl_32(&self) -> REG_UHS_BL_32_R {
        REG_UHS_BL_32_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_uhs_bl_64(&self) -> REG_UHS_BL_64_R {
        REG_UHS_BL_64_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_latency(&mut self) -> REG_UHS_LATENCY_W<0> {
        REG_UHS_LATENCY_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_drive_st(&mut self) -> REG_UHS_DRIVE_ST_W<4> {
        REG_UHS_DRIVE_ST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_bl_16(&mut self) -> REG_UHS_BL_16_W<8> {
        REG_UHS_BL_16_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_bl_32(&mut self) -> REG_UHS_BL_32_W<9> {
        REG_UHS_BL_32_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_bl_64(&mut self) -> REG_UHS_BL_64_W<10> {
        REG_UHS_BL_64_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_psram_configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_psram_configure](index.html) module"]
pub struct UHS_PSRAM_CONFIGURE_SPEC;
impl crate::RegisterSpec for UHS_PSRAM_CONFIGURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_psram_configure::R](R) reader structure"]
impl crate::Readable for UHS_PSRAM_CONFIGURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_psram_configure::W](W) writer structure"]
impl crate::Writable for UHS_PSRAM_CONFIGURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_psram_configure to value 0xa5"]
impl crate::Resettable for UHS_PSRAM_CONFIGURE_SPEC {
    const RESET_VALUE: Self::Ux = 0xa5;
}
