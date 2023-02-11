#[doc = "Register `phy_cfg_40` reader"]
pub struct R(crate::R<PHY_CFG_40_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_40_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_40_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_40_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_40` writer"]
pub struct W(crate::W<PHY_CFG_40_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_40_SPEC>;
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
impl From<crate::W<PHY_CFG_40_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_40_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vref_sel` reader - "]
pub type VREF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vref_sel` writer - "]
pub type VREF_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 4, O>;
#[doc = "Field `vref_dq_sel` reader - "]
pub type VREF_DQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vref_dq_sel` writer - "]
pub type VREF_DQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 4, O>;
#[doc = "Field `reg_uhs_dmy0` reader - "]
pub type REG_UHS_DMY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_uhs_dmy0` writer - "]
pub type REG_UHS_DMY0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_uhs_dmy1` reader - "]
pub type REG_UHS_DMY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_uhs_dmy1` writer - "]
pub type REG_UHS_DMY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_uhs_phy_ten` reader - "]
pub type REG_UHS_PHY_TEN_R = crate::BitReader<bool>;
#[doc = "Field `reg_uhs_phy_ten` writer - "]
pub type REG_UHS_PHY_TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `soc_en_aon` reader - "]
pub type SOC_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `soc_en_aon` writer - "]
pub type SOC_EN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `ten_uhs_phy` reader - "]
pub type TEN_UHS_PHY_R = crate::BitReader<bool>;
#[doc = "Field `ten_uhs_phy` writer - "]
pub type TEN_UHS_PHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `ten_uhs_phy_dig` reader - "]
pub type TEN_UHS_PHY_DIG_R = crate::BitReader<bool>;
#[doc = "Field `ten_uhs_phy_dig` writer - "]
pub type TEN_UHS_PHY_DIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `reserved_28` reader - "]
pub type RESERVED_28_R = crate::BitReader<bool>;
#[doc = "Field `tx_clktree_gate_hw` reader - "]
pub type TX_CLKTREE_GATE_HW_R = crate::BitReader<bool>;
#[doc = "Field `tx_clktree_gate_hw` writer - "]
pub type TX_CLKTREE_GATE_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `uhs_dc_tp_out_en` reader - "]
pub type UHS_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhs_dc_tp_out_en` writer - "]
pub type UHS_DC_TP_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
#[doc = "Field `uhs_phy_dqs_diff` reader - "]
pub type UHS_PHY_DQS_DIFF_R = crate::BitReader<bool>;
#[doc = "Field `uhs_phy_dqs_diff` writer - "]
pub type UHS_PHY_DQS_DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_40_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VREF_SEL_R {
        VREF_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn vref_dq_sel(&self) -> VREF_DQ_SEL_R {
        VREF_DQ_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_uhs_dmy0(&self) -> REG_UHS_DMY0_R {
        REG_UHS_DMY0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_uhs_dmy1(&self) -> REG_UHS_DMY1_R {
        REG_UHS_DMY1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_uhs_phy_ten(&self) -> REG_UHS_PHY_TEN_R {
        REG_UHS_PHY_TEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn soc_en_aon(&self) -> SOC_EN_AON_R {
        SOC_EN_AON_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_uhs_phy(&self) -> TEN_UHS_PHY_R {
        TEN_UHS_PHY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_uhs_phy_dig(&self) -> TEN_UHS_PHY_DIG_R {
        TEN_UHS_PHY_DIG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reserved_28(&self) -> RESERVED_28_R {
        RESERVED_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tx_clktree_gate_hw(&self) -> TX_CLKTREE_GATE_HW_R {
        TX_CLKTREE_GATE_HW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn uhs_dc_tp_out_en(&self) -> UHS_DC_TP_OUT_EN_R {
        UHS_DC_TP_OUT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn uhs_phy_dqs_diff(&self) -> UHS_PHY_DQS_DIFF_R {
        UHS_PHY_DQS_DIFF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel(&mut self) -> VREF_SEL_W<0> {
        VREF_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn vref_dq_sel(&mut self) -> VREF_DQ_SEL_W<4> {
        VREF_DQ_SEL_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_dmy0(&mut self) -> REG_UHS_DMY0_W<8> {
        REG_UHS_DMY0_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_dmy1(&mut self) -> REG_UHS_DMY1_W<16> {
        REG_UHS_DMY1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uhs_phy_ten(&mut self) -> REG_UHS_PHY_TEN_W<24> {
        REG_UHS_PHY_TEN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn soc_en_aon(&mut self) -> SOC_EN_AON_W<25> {
        SOC_EN_AON_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ten_uhs_phy(&mut self) -> TEN_UHS_PHY_W<26> {
        TEN_UHS_PHY_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ten_uhs_phy_dig(&mut self) -> TEN_UHS_PHY_DIG_W<27> {
        TEN_UHS_PHY_DIG_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clktree_gate_hw(&mut self) -> TX_CLKTREE_GATE_HW_W<29> {
        TX_CLKTREE_GATE_HW_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn uhs_dc_tp_out_en(&mut self) -> UHS_DC_TP_OUT_EN_W<30> {
        UHS_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn uhs_phy_dqs_diff(&mut self) -> UHS_PHY_DQS_DIFF_W<31> {
        UHS_PHY_DQS_DIFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_40\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_40](index.html) module"]
pub struct PHY_CFG_40_SPEC;
impl crate::RegisterSpec for PHY_CFG_40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_40::R](R) reader structure"]
impl crate::Readable for PHY_CFG_40_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_40::W](W) writer structure"]
impl crate::Writable for PHY_CFG_40_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_40 to value 0xa2ff_0000"]
impl crate::Resettable for PHY_CFG_40_SPEC {
    const RESET_VALUE: Self::Ux = 0xa2ff_0000;
}
