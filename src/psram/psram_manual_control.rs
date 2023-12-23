#[doc = "Register `psram_manual_control` reader"]
pub struct R(crate::R<PSRAM_MANUAL_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_MANUAL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_MANUAL_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_MANUAL_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_manual_control` writer"]
pub struct W(crate::W<PSRAM_MANUAL_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_MANUAL_CONTROL_SPEC>;
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
impl From<crate::W<PSRAM_MANUAL_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_MANUAL_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_wc_sw` reader - "]
pub type REG_WC_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wc_sw` writer - "]
pub type REG_WC_SW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_wc_sw_en` reader - "]
pub type REG_WC_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_wc_sw_en` writer - "]
pub type REG_WC_SW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_state_hold_tick` reader - "]
pub type REG_STATE_HOLD_TICK_R = crate::BitReader<bool>;
#[doc = "Field `reg_state_hold_tick` writer - "]
pub type REG_STATE_HOLD_TICK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_dqs_latch_inv` reader - "]
pub type REG_DQS_LATCH_INV_R = crate::BitReader<bool>;
#[doc = "Field `reg_dqs_latch_inv` writer - "]
pub type REG_DQS_LATCH_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_wb_bl2_mask` reader - "]
pub type REG_WB_BL2_MASK_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_bl2_mask` writer - "]
pub type REG_WB_BL2_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_force_ceb_low` reader - "]
pub type REG_FORCE_CEB_LOW_R = crate::BitReader<bool>;
#[doc = "Field `reg_force_ceb_low` writer - "]
pub type REG_FORCE_CEB_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_force_ceb_high` reader - "]
pub type REG_FORCE_CEB_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `reg_force_ceb_high` writer - "]
pub type REG_FORCE_CEB_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_psram_resetb` reader - "]
pub type REG_PSRAM_RESETB_R = crate::BitReader<bool>;
#[doc = "Field `reg_psram_resetb` writer - "]
pub type REG_PSRAM_RESETB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_ck_edge_nali` reader - "]
pub type REG_CK_EDGE_NALI_R = crate::BitReader<bool>;
#[doc = "Field `reg_ck_edge_nali` writer - "]
pub type REG_CK_EDGE_NALI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL_SPEC, bool, O>;
#[doc = "Field `sts_config_read` reader - "]
pub type STS_CONFIG_READ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn reg_wc_sw(&self) -> REG_WC_SW_R {
        REG_WC_SW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_wc_sw_en(&self) -> REG_WC_SW_EN_R {
        REG_WC_SW_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_state_hold_tick(&self) -> REG_STATE_HOLD_TICK_R {
        REG_STATE_HOLD_TICK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_dqs_latch_inv(&self) -> REG_DQS_LATCH_INV_R {
        REG_DQS_LATCH_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_wb_bl2_mask(&self) -> REG_WB_BL2_MASK_R {
        REG_WB_BL2_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&self) -> REG_FORCE_CEB_LOW_R {
        REG_FORCE_CEB_LOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&self) -> REG_FORCE_CEB_HIGH_R {
        REG_FORCE_CEB_HIGH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_psram_resetb(&self) -> REG_PSRAM_RESETB_R {
        REG_PSRAM_RESETB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_ck_edge_nali(&self) -> REG_CK_EDGE_NALI_R {
        REG_CK_EDGE_NALI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_config_read(&self) -> STS_CONFIG_READ_R {
        STS_CONFIG_READ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wc_sw(&mut self) -> REG_WC_SW_W<0> {
        REG_WC_SW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wc_sw_en(&mut self) -> REG_WC_SW_EN_W<8> {
        REG_WC_SW_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_state_hold_tick(&mut self) -> REG_STATE_HOLD_TICK_W<9> {
        REG_STATE_HOLD_TICK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dqs_latch_inv(&mut self) -> REG_DQS_LATCH_INV_W<10> {
        REG_DQS_LATCH_INV_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_bl2_mask(&mut self) -> REG_WB_BL2_MASK_W<11> {
        REG_WB_BL2_MASK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_force_ceb_low(&mut self) -> REG_FORCE_CEB_LOW_W<12> {
        REG_FORCE_CEB_LOW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_force_ceb_high(&mut self) -> REG_FORCE_CEB_HIGH_W<13> {
        REG_FORCE_CEB_HIGH_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_resetb(&mut self) -> REG_PSRAM_RESETB_W<14> {
        REG_PSRAM_RESETB_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ck_edge_nali(&mut self) -> REG_CK_EDGE_NALI_W<15> {
        REG_CK_EDGE_NALI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_manual_control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_manual_control](index.html) module"]
pub struct PSRAM_MANUAL_CONTROL_SPEC;
impl crate::RegisterSpec for PSRAM_MANUAL_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_manual_control::R](R) reader structure"]
impl crate::Readable for PSRAM_MANUAL_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_manual_control::W](W) writer structure"]
impl crate::Writable for PSRAM_MANUAL_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_manual_control to value 0x4800"]
impl crate::Resettable for PSRAM_MANUAL_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4800;
}
