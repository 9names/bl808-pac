#[doc = "Register `phy_cfg_48` reader"]
pub struct R(crate::R<PHY_CFG_48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_48` writer"]
pub struct W(crate::W<PHY_CFG_48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_48_SPEC>;
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
impl From<crate::W<PHY_CFG_48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmux` reader - "]
pub type TMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux` writer - "]
pub type TMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 3, O>;
#[doc = "Field `chip_en_33` reader - "]
pub type CHIP_EN_33_R = crate::BitReader<bool>;
#[doc = "Field `chip_en_33` writer - "]
pub type CHIP_EN_33_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_48_SPEC, bool, O>;
#[doc = "Field `tmux_uhs_phy_dig` reader - "]
pub type TMUX_UHS_PHY_DIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux_uhs_phy_dig` writer - "]
pub type TMUX_UHS_PHY_DIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `psram_type` reader - "]
pub type PSRAM_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `psram_type` writer - "]
pub type PSRAM_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_10` reader - "]
pub type RESERVED_10_R = crate::BitReader<bool>;
#[doc = "Field `pu_uhs_pw1p8` reader - "]
pub type PU_UHS_PW1P8_R = crate::BitReader<bool>;
#[doc = "Field `pu_uhs_pw1p8` writer - "]
pub type PU_UHS_PW1P8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_48_SPEC, bool, O>;
#[doc = "Field `reg_test_div_sel` reader - "]
pub type REG_TEST_DIV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_test_div_sel` writer - "]
pub type REG_TEST_DIV_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 3, O>;
#[doc = "Field `en_rx_fe_hw` reader - "]
pub type EN_RX_FE_HW_R = crate::BitReader<bool>;
#[doc = "Field `en_rx_fe_hw` writer - "]
pub type EN_RX_FE_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_48_SPEC, bool, O>;
#[doc = "Field `reg_test_mux_sel` reader - "]
pub type REG_TEST_MUX_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_test_mux_sel` writer - "]
pub type REG_TEST_MUX_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 3, O>;
#[doc = "Field `force_fsm` reader - "]
pub type FORCE_FSM_R = crate::BitReader<bool>;
#[doc = "Field `force_fsm` writer - "]
pub type FORCE_FSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_48_SPEC, bool, O>;
#[doc = "Field `en_rx_fe_dly` reader - "]
pub type EN_RX_FE_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `en_rx_fe_dly` writer - "]
pub type EN_RX_FE_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_48_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_24_31` reader - "]
pub type RESERVED_24_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&self) -> TMUX_R {
        TMUX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chip_en_33(&self) -> CHIP_EN_33_R {
        CHIP_EN_33_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmux_uhs_phy_dig(&self) -> TMUX_UHS_PHY_DIG_R {
        TMUX_UHS_PHY_DIG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn psram_type(&self) -> PSRAM_TYPE_R {
        PSRAM_TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reserved_10(&self) -> RESERVED_10_R {
        RESERVED_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_uhs_pw1p8(&self) -> PU_UHS_PW1P8_R {
        PU_UHS_PW1P8_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn reg_test_div_sel(&self) -> REG_TEST_DIV_SEL_R {
        REG_TEST_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn en_rx_fe_hw(&self) -> EN_RX_FE_HW_R {
        EN_RX_FE_HW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn reg_test_mux_sel(&self) -> REG_TEST_MUX_SEL_R {
        REG_TEST_MUX_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn force_fsm(&self) -> FORCE_FSM_R {
        FORCE_FSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn en_rx_fe_dly(&self) -> EN_RX_FE_DLY_R {
        EN_RX_FE_DLY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reserved_24_31(&self) -> RESERVED_24_31_R {
        RESERVED_24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmux(&mut self) -> TMUX_W<0> {
        TMUX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn chip_en_33(&mut self) -> CHIP_EN_33_W<3> {
        CHIP_EN_33_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_uhs_phy_dig(&mut self) -> TMUX_UHS_PHY_DIG_W<4> {
        TMUX_UHS_PHY_DIG_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn psram_type(&mut self) -> PSRAM_TYPE_W<8> {
        PSRAM_TYPE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_uhs_pw1p8(&mut self) -> PU_UHS_PW1P8_W<11> {
        PU_UHS_PW1P8_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_test_div_sel(&mut self) -> REG_TEST_DIV_SEL_W<12> {
        REG_TEST_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn en_rx_fe_hw(&mut self) -> EN_RX_FE_HW_W<15> {
        EN_RX_FE_HW_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_test_mux_sel(&mut self) -> REG_TEST_MUX_SEL_W<16> {
        REG_TEST_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn force_fsm(&mut self) -> FORCE_FSM_W<19> {
        FORCE_FSM_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn en_rx_fe_dly(&mut self) -> EN_RX_FE_DLY_W<20> {
        EN_RX_FE_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_48\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_48](index.html) module"]
pub struct PHY_CFG_48_SPEC;
impl crate::RegisterSpec for PHY_CFG_48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_48::R](R) reader structure"]
impl crate::Readable for PHY_CFG_48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_48::W](W) writer structure"]
impl crate::Writable for PHY_CFG_48_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_48 to value 0x0020_8908"]
impl crate::Resettable for PHY_CFG_48_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_8908;
}
