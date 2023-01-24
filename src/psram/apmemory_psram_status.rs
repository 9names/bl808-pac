#[doc = "Register `apmemory_psram_status` reader"]
pub struct R(crate::R<APMEMORY_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APMEMORY_PSRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APMEMORY_PSRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APMEMORY_PSRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `apmemory_psram_status` writer"]
pub struct W(crate::W<APMEMORY_PSRAM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APMEMORY_PSRAM_STATUS_SPEC>;
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
impl From<crate::W<APMEMORY_PSRAM_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APMEMORY_PSRAM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_ap_burst_length` reader - "]
pub type STS_AP_BURST_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_ap_burst_type` reader - "]
pub type STS_AP_BURST_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_rbx` reader - "]
pub type STS_AP_RBX_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_x16_mode` reader - "]
pub type STS_AP_X16_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_pasr` reader - "]
pub type STS_AP_PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_w_latency_code` reader - "]
pub type STS_AP_W_LATENCY_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_drive_st` reader - "]
pub type STS_AP_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_ap_rf` reader - "]
pub type STS_AP_RF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_ap_r_latency_code` reader - "]
pub type STS_AP_R_LATENCY_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `sts_ap_r_latency_type` reader - "]
pub type STS_AP_R_LATENCY_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_25_31` reader - "]
pub type RESERVED_25_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sts_ap_burst_length(&self) -> STS_AP_BURST_LENGTH_R {
        STS_AP_BURST_LENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_ap_burst_type(&self) -> STS_AP_BURST_TYPE_R {
        STS_AP_BURST_TYPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_ap_rbx(&self) -> STS_AP_RBX_R {
        STS_AP_RBX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_ap_x16_mode(&self) -> STS_AP_X16_MODE_R {
        STS_AP_X16_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sts_ap_pasr(&self) -> STS_AP_PASR_R {
        STS_AP_PASR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sts_ap_w_latency_code(&self) -> STS_AP_W_LATENCY_CODE_R {
        STS_AP_W_LATENCY_CODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sts_ap_drive_st(&self) -> STS_AP_DRIVE_ST_R {
        STS_AP_DRIVE_ST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sts_ap_rf(&self) -> STS_AP_RF_R {
        STS_AP_RF_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sts_ap_r_latency_code(&self) -> STS_AP_R_LATENCY_CODE_R {
        STS_AP_R_LATENCY_CODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_ap_r_latency_type(&self) -> STS_AP_R_LATENCY_TYPE_R {
        STS_AP_R_LATENCY_TYPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn reserved_25_31(&self) -> RESERVED_25_31_R {
        RESERVED_25_31_R::new(((self.bits >> 25) & 0x7f) as u8)
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
#[doc = "apmemory_psram_status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apmemory_psram_status](index.html) module"]
pub struct APMEMORY_PSRAM_STATUS_SPEC;
impl crate::RegisterSpec for APMEMORY_PSRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apmemory_psram_status::R](R) reader structure"]
impl crate::Readable for APMEMORY_PSRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apmemory_psram_status::W](W) writer structure"]
impl crate::Writable for APMEMORY_PSRAM_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets apmemory_psram_status to value 0x0021_2011"]
impl crate::Resettable for APMEMORY_PSRAM_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0021_2011;
}
