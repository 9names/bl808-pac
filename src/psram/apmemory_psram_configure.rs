#[doc = "Register `apmemory_psram_configure` reader"]
pub struct R(crate::R<APMEMORY_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APMEMORY_PSRAM_CONFIGURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APMEMORY_PSRAM_CONFIGURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APMEMORY_PSRAM_CONFIGURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `apmemory_psram_configure` writer"]
pub struct W(crate::W<APMEMORY_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APMEMORY_PSRAM_CONFIGURE_SPEC>;
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
impl From<crate::W<APMEMORY_PSRAM_CONFIGURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APMEMORY_PSRAM_CONFIGURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_ap_burst_length` reader - "]
pub type REG_AP_BURST_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_burst_length` writer - "]
pub type REG_AP_BURST_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_burst_type` reader - "]
pub type REG_AP_BURST_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_burst_type` writer - "]
pub type REG_AP_BURST_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_ap_rbx` reader - "]
pub type REG_AP_RBX_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_rbx` writer - "]
pub type REG_AP_RBX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_ap_dpd` reader - "]
pub type REG_AP_DPD_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_dpd` writer - "]
pub type REG_AP_DPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_ap_sleep` reader - "]
pub type REG_AP_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_sleep` writer - "]
pub type REG_AP_SLEEP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_ap_pasr` reader - "]
pub type REG_AP_PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_pasr` writer - "]
pub type REG_AP_PASR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_w_latency_code` reader - "]
pub type REG_AP_W_LATENCY_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_w_latency_code` writer - "]
pub type REG_AP_W_LATENCY_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_drive_st` reader - "]
pub type REG_AP_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_drive_st` writer - "]
pub type REG_AP_DRIVE_ST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_ap_rf` reader - "]
pub type REG_AP_RF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_rf` writer - "]
pub type REG_AP_RF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_ap_r_latency_code` reader - "]
pub type REG_AP_R_LATENCY_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_r_latency_code` writer - "]
pub type REG_AP_R_LATENCY_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_r_latency_type` reader - "]
pub type REG_AP_R_LATENCY_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_r_latency_type` writer - "]
pub type REG_AP_R_LATENCY_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_ap_linear_dis` reader - "]
pub type REG_AP_LINEAR_DIS_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_linear_dis` writer - "]
pub type REG_AP_LINEAR_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_glb_reset_pulse` writer - "]
pub type REG_GLB_RESET_PULSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APMEMORY_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_29_31` reader - "]
pub type RESERVED_29_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_ap_burst_length(&self) -> REG_AP_BURST_LENGTH_R {
        REG_AP_BURST_LENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_ap_burst_type(&self) -> REG_AP_BURST_TYPE_R {
        REG_AP_BURST_TYPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_ap_rbx(&self) -> REG_AP_RBX_R {
        REG_AP_RBX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_ap_dpd(&self) -> REG_AP_DPD_R {
        REG_AP_DPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_ap_sleep(&self) -> REG_AP_SLEEP_R {
        REG_AP_SLEEP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_ap_pasr(&self) -> REG_AP_PASR_R {
        REG_AP_PASR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn reg_ap_w_latency_code(&self) -> REG_AP_W_LATENCY_CODE_R {
        REG_AP_W_LATENCY_CODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn reg_ap_drive_st(&self) -> REG_AP_DRIVE_ST_R {
        REG_AP_DRIVE_ST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_ap_rf(&self) -> REG_AP_RF_R {
        REG_AP_RF_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn reg_ap_r_latency_code(&self) -> REG_AP_R_LATENCY_CODE_R {
        REG_AP_R_LATENCY_CODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_ap_r_latency_type(&self) -> REG_AP_R_LATENCY_TYPE_R {
        REG_AP_R_LATENCY_TYPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_ap_linear_dis(&self) -> REG_AP_LINEAR_DIS_R {
        REG_AP_LINEAR_DIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn reserved_29_31(&self) -> RESERVED_29_31_R {
        RESERVED_29_31_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_burst_length(&mut self) -> REG_AP_BURST_LENGTH_W<0> {
        REG_AP_BURST_LENGTH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_burst_type(&mut self) -> REG_AP_BURST_TYPE_W<4> {
        REG_AP_BURST_TYPE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_rbx(&mut self) -> REG_AP_RBX_W<5> {
        REG_AP_RBX_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_dpd(&mut self) -> REG_AP_DPD_W<6> {
        REG_AP_DPD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_sleep(&mut self) -> REG_AP_SLEEP_W<7> {
        REG_AP_SLEEP_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_pasr(&mut self) -> REG_AP_PASR_W<8> {
        REG_AP_PASR_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_w_latency_code(&mut self) -> REG_AP_W_LATENCY_CODE_W<12> {
        REG_AP_W_LATENCY_CODE_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_drive_st(&mut self) -> REG_AP_DRIVE_ST_W<16> {
        REG_AP_DRIVE_ST_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_rf(&mut self) -> REG_AP_RF_W<18> {
        REG_AP_RF_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_r_latency_code(&mut self) -> REG_AP_R_LATENCY_CODE_W<20> {
        REG_AP_R_LATENCY_CODE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_r_latency_type(&mut self) -> REG_AP_R_LATENCY_TYPE_W<24> {
        REG_AP_R_LATENCY_TYPE_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_linear_dis(&mut self) -> REG_AP_LINEAR_DIS_W<25> {
        REG_AP_LINEAR_DIS_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_glb_reset_pulse(&mut self) -> REG_GLB_RESET_PULSE_W<28> {
        REG_GLB_RESET_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "apmemory_psram_configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apmemory_psram_configure](index.html) module"]
pub struct APMEMORY_PSRAM_CONFIGURE_SPEC;
impl crate::RegisterSpec for APMEMORY_PSRAM_CONFIGURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apmemory_psram_configure::R](R) reader structure"]
impl crate::Readable for APMEMORY_PSRAM_CONFIGURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apmemory_psram_configure::W](W) writer structure"]
impl crate::Writable for APMEMORY_PSRAM_CONFIGURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets apmemory_psram_configure to value 0x0021_2011"]
impl crate::Resettable for APMEMORY_PSRAM_CONFIGURE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0021_2011;
}
