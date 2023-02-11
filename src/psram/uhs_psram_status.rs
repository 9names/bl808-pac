#[doc = "Register `uhs_psram_status` reader"]
pub struct R(crate::R<UHS_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PSRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PSRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PSRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_psram_status` writer"]
pub struct W(crate::W<UHS_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PSRAM_STATUS_SPEC>;
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
impl From<crate::W<UHS_PSRAM_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PSRAM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_uhs_latency` reader - "]
pub type STS_UHS_LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `sts_uhs_drive_st` reader - "]
pub type STS_UHS_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_uhs_bl_16` reader - "]
pub type STS_UHS_BL_16_R = crate::BitReader<bool>;
#[doc = "Field `sts_uhs_bl_32` reader - "]
pub type STS_UHS_BL_32_R = crate::BitReader<bool>;
#[doc = "Field `sts_uhs_bl_64` reader - "]
pub type STS_UHS_BL_64_R = crate::BitReader<bool>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sts_uhs_latency(&self) -> STS_UHS_LATENCY_R {
        STS_UHS_LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sts_uhs_drive_st(&self) -> STS_UHS_DRIVE_ST_R {
        STS_UHS_DRIVE_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_uhs_bl_16(&self) -> STS_UHS_BL_16_R {
        STS_UHS_BL_16_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_uhs_bl_32(&self) -> STS_UHS_BL_32_R {
        STS_UHS_BL_32_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sts_uhs_bl_64(&self) -> STS_UHS_BL_64_R {
        STS_UHS_BL_64_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_psram_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_psram_status](index.html) module"]
pub struct UHS_PSRAM_STATUS_SPEC;
impl crate::RegisterSpec for UHS_PSRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_psram_status::R](R) reader structure"]
impl crate::Readable for UHS_PSRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_psram_status::W](W) writer structure"]
impl crate::Writable for UHS_PSRAM_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_psram_status to value 0xa5"]
impl crate::Resettable for UHS_PSRAM_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0xa5;
}
