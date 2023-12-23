#[doc = "Register `winbond_psram_configure` reader"]
pub struct R(crate::R<WINBOND_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINBOND_PSRAM_CONFIGURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINBOND_PSRAM_CONFIGURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINBOND_PSRAM_CONFIGURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `winbond_psram_configure` writer"]
pub struct W(crate::W<WINBOND_PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINBOND_PSRAM_CONFIGURE_SPEC>;
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
impl From<crate::W<WINBOND_PSRAM_CONFIGURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINBOND_PSRAM_CONFIGURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_wb_latency` reader - "]
pub type REG_WB_LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_latency` writer - "]
pub type REG_WB_LATENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, u8, u8, 4, O>;
#[doc = "Field `reg_wb_drive_st` reader - "]
pub type REG_WB_DRIVE_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_drive_st` writer - "]
pub type REG_WB_DRIVE_ST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_wb_hybrid_en` reader - "]
pub type REG_WB_HYBRID_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_hybrid_en` writer - "]
pub type REG_WB_HYBRID_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_burst_length` reader - "]
pub type REG_WB_BURST_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_burst_length` writer - "]
pub type REG_WB_BURST_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_fix_latency` reader - "]
pub type REG_WB_FIX_LATENCY_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_fix_latency` writer - "]
pub type REG_WB_FIX_LATENCY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_dpd_dis` reader - "]
pub type REG_WB_DPD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_dpd_dis` writer - "]
pub type REG_WB_DPD_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_pasr` reader - "]
pub type REG_WB_PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_pasr` writer - "]
pub type REG_WB_PASR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_21_23` reader - "]
pub type RESERVED_21_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_hybrid_slp` reader - "]
pub type REG_WB_HYBRID_SLP_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_hybrid_slp` writer - "]
pub type REG_WB_HYBRID_SLP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_linear_dis` reader - "]
pub type REG_WB_LINEAR_DIS_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_linear_dis` writer - "]
pub type REG_WB_LINEAR_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_26_28` reader - "]
pub type RESERVED_26_28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_ipd` reader - "]
pub type REG_WB_IPD_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_ipd` writer - "]
pub type REG_WB_IPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_mclk_type` reader - "]
pub type REG_WB_MCLK_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_mclk_type` writer - "]
pub type REG_WB_MCLK_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_sw_rst` reader - "]
pub type REG_WB_SW_RST_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_sw_rst` writer - "]
pub type REG_WB_SW_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WINBOND_PSRAM_CONFIGURE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_wb_latency(&self) -> REG_WB_LATENCY_R {
        REG_WB_LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn reg_wb_drive_st(&self) -> REG_WB_DRIVE_ST_R {
        REG_WB_DRIVE_ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_wb_hybrid_en(&self) -> REG_WB_HYBRID_EN_R {
        REG_WB_HYBRID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_wb_burst_length(&self) -> REG_WB_BURST_LENGTH_R {
        REG_WB_BURST_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_wb_fix_latency(&self) -> REG_WB_FIX_LATENCY_R {
        REG_WB_FIX_LATENCY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_wb_dpd_dis(&self) -> REG_WB_DPD_DIS_R {
        REG_WB_DPD_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reg_wb_pasr(&self) -> REG_WB_PASR_R {
        REG_WB_PASR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn reserved_21_23(&self) -> RESERVED_21_23_R {
        RESERVED_21_23_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_wb_hybrid_slp(&self) -> REG_WB_HYBRID_SLP_R {
        REG_WB_HYBRID_SLP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_wb_linear_dis(&self) -> REG_WB_LINEAR_DIS_R {
        REG_WB_LINEAR_DIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn reserved_26_28(&self) -> RESERVED_26_28_R {
        RESERVED_26_28_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_wb_ipd(&self) -> REG_WB_IPD_R {
        REG_WB_IPD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_wb_mclk_type(&self) -> REG_WB_MCLK_TYPE_R {
        REG_WB_MCLK_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_wb_sw_rst(&self) -> REG_WB_SW_RST_R {
        REG_WB_SW_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_latency(&mut self) -> REG_WB_LATENCY_W<0> {
        REG_WB_LATENCY_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_drive_st(&mut self) -> REG_WB_DRIVE_ST_W<4> {
        REG_WB_DRIVE_ST_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_hybrid_en(&mut self) -> REG_WB_HYBRID_EN_W<7> {
        REG_WB_HYBRID_EN_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_burst_length(&mut self) -> REG_WB_BURST_LENGTH_W<8> {
        REG_WB_BURST_LENGTH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_fix_latency(&mut self) -> REG_WB_FIX_LATENCY_W<12> {
        REG_WB_FIX_LATENCY_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_dpd_dis(&mut self) -> REG_WB_DPD_DIS_W<13> {
        REG_WB_DPD_DIS_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_pasr(&mut self) -> REG_WB_PASR_W<16> {
        REG_WB_PASR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_hybrid_slp(&mut self) -> REG_WB_HYBRID_SLP_W<24> {
        REG_WB_HYBRID_SLP_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_linear_dis(&mut self) -> REG_WB_LINEAR_DIS_W<25> {
        REG_WB_LINEAR_DIS_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_ipd(&mut self) -> REG_WB_IPD_W<29> {
        REG_WB_IPD_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_mclk_type(&mut self) -> REG_WB_MCLK_TYPE_W<30> {
        REG_WB_MCLK_TYPE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_sw_rst(&mut self) -> REG_WB_SW_RST_W<31> {
        REG_WB_SW_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "winbond_psram_configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winbond_psram_configure](index.html) module"]
pub struct WINBOND_PSRAM_CONFIGURE_SPEC;
impl crate::RegisterSpec for WINBOND_PSRAM_CONFIGURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winbond_psram_configure::R](R) reader structure"]
impl crate::Readable for WINBOND_PSRAM_CONFIGURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winbond_psram_configure::W](W) writer structure"]
impl crate::Writable for WINBOND_PSRAM_CONFIGURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets winbond_psram_configure to value 0x4000_3782"]
impl crate::Resettable for WINBOND_PSRAM_CONFIGURE_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_3782;
}
