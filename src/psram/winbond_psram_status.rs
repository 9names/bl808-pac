#[doc = "Register `winbond_psram_status` reader"]
pub struct R(crate::R<WINBOND_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINBOND_PSRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINBOND_PSRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINBOND_PSRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `winbond_psram_status` writer"]
pub struct W(crate::W<WINBOND_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINBOND_PSRAM_STATUS_SPEC>;
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
impl From<crate::W<WINBOND_PSRAM_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINBOND_PSRAM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_wb_latency` reader - "]
pub type STS_WB_LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_wb_drive_st` reader - "]
pub type STS_WB_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_wb_hybrid_en` reader - "]
pub type STS_WB_HYBRID_EN_R = crate::BitReader<bool>;
#[doc = "Field `sts_wb_burst_length` reader - "]
pub type STS_WB_BURST_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `sts_wb_fix_latency` reader - "]
pub type STS_WB_FIX_LATENCY_R = crate::BitReader<bool>;
#[doc = "Field `sts_wb_dpd_dis` reader - "]
pub type STS_WB_DPD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_wb_pasr` reader - "]
pub type STS_WB_PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_21_23` reader - "]
pub type RESERVED_21_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_wb_hybrid_slp` reader - "]
pub type STS_WB_HYBRID_SLP_R = crate::BitReader<bool>;
#[doc = "Field `reserved_25_29` reader - "]
pub type RESERVED_25_29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_wb_mclk_type` reader - "]
pub type STS_WB_MCLK_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sts_wb_latency(&self) -> STS_WB_LATENCY_R {
        STS_WB_LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn sts_wb_drive_st(&self) -> STS_WB_DRIVE_ST_R {
        STS_WB_DRIVE_ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_wb_hybrid_en(&self) -> STS_WB_HYBRID_EN_R {
        STS_WB_HYBRID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sts_wb_burst_length(&self) -> STS_WB_BURST_LENGTH_R {
        STS_WB_BURST_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sts_wb_fix_latency(&self) -> STS_WB_FIX_LATENCY_R {
        STS_WB_FIX_LATENCY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sts_wb_dpd_dis(&self) -> STS_WB_DPD_DIS_R {
        STS_WB_DPD_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn sts_wb_pasr(&self) -> STS_WB_PASR_R {
        STS_WB_PASR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn reserved_21_23(&self) -> RESERVED_21_23_R {
        RESERVED_21_23_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_wb_hybrid_slp(&self) -> STS_WB_HYBRID_SLP_R {
        STS_WB_HYBRID_SLP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn reserved_25_29(&self) -> RESERVED_25_29_R {
        RESERVED_25_29_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sts_wb_mclk_type(&self) -> STS_WB_MCLK_TYPE_R {
        STS_WB_MCLK_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "winbond_psram_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winbond_psram_status](index.html) module"]
pub struct WINBOND_PSRAM_STATUS_SPEC;
impl crate::RegisterSpec for WINBOND_PSRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winbond_psram_status::R](R) reader structure"]
impl crate::Readable for WINBOND_PSRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winbond_psram_status::W](W) writer structure"]
impl crate::Writable for WINBOND_PSRAM_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets winbond_psram_status to value 0x4000_3382"]
impl crate::Resettable for WINBOND_PSRAM_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_3382;
}
