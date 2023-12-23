#[doc = "Register `psram_manual_control2` reader"]
pub struct R(crate::R<PSRAM_MANUAL_CONTROL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_MANUAL_CONTROL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_MANUAL_CONTROL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_MANUAL_CONTROL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_manual_control2` writer"]
pub struct W(crate::W<PSRAM_MANUAL_CONTROL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_MANUAL_CONTROL2_SPEC>;
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
impl From<crate::W<PSRAM_MANUAL_CONTROL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_MANUAL_CONTROL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_hold_cycle_sw` reader - "]
pub type REG_HOLD_CYCLE_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_hold_cycle_sw` writer - "]
pub type REG_HOLD_CYCLE_SW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, u8, u8, 7, O>;
#[doc = "Field `reg_hc_sw_en` reader - "]
pub type REG_HC_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_hc_sw_en` writer - "]
pub type REG_HC_SW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, bool, O>;
#[doc = "Field `reg_dqs_rel_val` reader - "]
pub type REG_DQS_REL_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_dqs_rel_val` writer - "]
pub type REG_DQS_REL_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `reg_pwrap_sw_sht_b` reader - "]
pub type REG_PWRAP_SW_SHT_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pwrap_sw_sht_b` writer - "]
pub type REG_PWRAP_SW_SHT_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_20_22` reader - "]
pub type RESERVED_20_22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pwrap_sw_en` reader - "]
pub type REG_PWRAP_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_pwrap_sw_en` writer - "]
pub type REG_PWRAP_SW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, bool, O>;
#[doc = "Field `reg_addr_mask` reader - "]
pub type REG_ADDR_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_addr_mask` writer - "]
pub type REG_ADDR_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn reg_hold_cycle_sw(&self) -> REG_HOLD_CYCLE_SW_R {
        REG_HOLD_CYCLE_SW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_hc_sw_en(&self) -> REG_HC_SW_EN_R {
        REG_HC_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn reg_dqs_rel_val(&self) -> REG_DQS_REL_VAL_R {
        REG_DQS_REL_VAL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_pwrap_sw_sht_b(&self) -> REG_PWRAP_SW_SHT_B_R {
        REG_PWRAP_SW_SHT_B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn reserved_20_22(&self) -> RESERVED_20_22_R {
        RESERVED_20_22_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_pwrap_sw_en(&self) -> REG_PWRAP_SW_EN_R {
        REG_PWRAP_SW_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_addr_mask(&self) -> REG_ADDR_MASK_R {
        REG_ADDR_MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_hold_cycle_sw(&mut self) -> REG_HOLD_CYCLE_SW_W<0> {
        REG_HOLD_CYCLE_SW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_hc_sw_en(&mut self) -> REG_HC_SW_EN_W<7> {
        REG_HC_SW_EN_W::new(self)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dqs_rel_val(&mut self) -> REG_DQS_REL_VAL_W<8> {
        REG_DQS_REL_VAL_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pwrap_sw_sht_b(&mut self) -> REG_PWRAP_SW_SHT_B_W<16> {
        REG_PWRAP_SW_SHT_B_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pwrap_sw_en(&mut self) -> REG_PWRAP_SW_EN_W<23> {
        REG_PWRAP_SW_EN_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_addr_mask(&mut self) -> REG_ADDR_MASK_W<24> {
        REG_ADDR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_manual_control2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_manual_control2](index.html) module"]
pub struct PSRAM_MANUAL_CONTROL2_SPEC;
impl crate::RegisterSpec for PSRAM_MANUAL_CONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_manual_control2::R](R) reader structure"]
impl crate::Readable for PSRAM_MANUAL_CONTROL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_manual_control2::W](W) writer structure"]
impl crate::Writable for PSRAM_MANUAL_CONTROL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_manual_control2 to value 0x1f08_2008"]
impl crate::Resettable for PSRAM_MANUAL_CONTROL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f08_2008;
}
