#[doc = "Register `phy_cfg_30` reader"]
pub struct R(crate::R<PHY_CFG_30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_30` writer"]
pub struct W(crate::W<PHY_CFG_30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_30_SPEC>;
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
impl From<crate::W<PHY_CFG_30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `phy_wl_dq_dig` reader - "]
pub type PHY_WL_DQ_DIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_wl_dq_dig` writer - "]
pub type PHY_WL_DQ_DIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `phy_wl_dq_ana` reader - "]
pub type PHY_WL_DQ_ANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_wl_dq_ana` writer - "]
pub type PHY_WL_DQ_ANA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `phy_wl_dig` reader - "]
pub type PHY_WL_DIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_wl_dig` writer - "]
pub type PHY_WL_DIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `phy_wl_ana` reader - "]
pub type PHY_WL_ANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_wl_ana` writer - "]
pub type PHY_WL_ANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `phy_rl_dig` reader - "]
pub type PHY_RL_DIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_rl_dig` writer - "]
pub type PHY_RL_DIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
#[doc = "Field `phy_rl_ana` reader - "]
pub type PHY_RL_ANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_rl_ana` writer - "]
pub type PHY_RL_ANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `oe_timer` reader - "]
pub type OE_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `oe_timer` writer - "]
pub type OE_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 2, O>;
#[doc = "Field `vref_mode` reader - "]
pub type VREF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `vref_mode` writer - "]
pub type VREF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_30_SPEC, bool, O>;
#[doc = "Field `oe_ctrl_hw` reader - "]
pub type OE_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `oe_ctrl_hw` writer - "]
pub type OE_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_30_SPEC, bool, O>;
#[doc = "Field `odt_sel` reader - "]
pub type ODT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `odt_sel` writer - "]
pub type ODT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn phy_wl_dq_dig(&self) -> PHY_WL_DQ_DIG_R {
        PHY_WL_DQ_DIG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn phy_wl_dq_ana(&self) -> PHY_WL_DQ_ANA_R {
        PHY_WL_DQ_ANA_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn phy_wl_dig(&self) -> PHY_WL_DIG_R {
        PHY_WL_DIG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn phy_wl_ana(&self) -> PHY_WL_ANA_R {
        PHY_WL_ANA_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn phy_rl_dig(&self) -> PHY_RL_DIG_R {
        PHY_RL_DIG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn phy_rl_ana(&self) -> PHY_RL_ANA_R {
        PHY_RL_ANA_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn oe_timer(&self) -> OE_TIMER_R {
        OE_TIMER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vref_mode(&self) -> VREF_MODE_R {
        VREF_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oe_ctrl_hw(&self) -> OE_CTRL_HW_R {
        OE_CTRL_HW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn odt_sel(&self) -> ODT_SEL_R {
        ODT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wl_dq_dig(&mut self) -> PHY_WL_DQ_DIG_W<0> {
        PHY_WL_DQ_DIG_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wl_dq_ana(&mut self) -> PHY_WL_DQ_ANA_W<4> {
        PHY_WL_DQ_ANA_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wl_dig(&mut self) -> PHY_WL_DIG_W<8> {
        PHY_WL_DIG_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wl_ana(&mut self) -> PHY_WL_ANA_W<12> {
        PHY_WL_ANA_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rl_dig(&mut self) -> PHY_RL_DIG_W<16> {
        PHY_RL_DIG_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rl_ana(&mut self) -> PHY_RL_ANA_W<20> {
        PHY_RL_ANA_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn oe_timer(&mut self) -> OE_TIMER_W<24> {
        OE_TIMER_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn vref_mode(&mut self) -> VREF_MODE_W<26> {
        VREF_MODE_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn oe_ctrl_hw(&mut self) -> OE_CTRL_HW_W<27> {
        OE_CTRL_HW_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn odt_sel(&mut self) -> ODT_SEL_W<28> {
        ODT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_30](index.html) module"]
pub struct PHY_CFG_30_SPEC;
impl crate::RegisterSpec for PHY_CFG_30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_30::R](R) reader structure"]
impl crate::Readable for PHY_CFG_30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_30::W](W) writer structure"]
impl crate::Writable for PHY_CFG_30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_30 to value 0xaa33_1020"]
impl crate::Resettable for PHY_CFG_30_SPEC {
    const RESET_VALUE: Self::Ux = 0xaa33_1020;
}
