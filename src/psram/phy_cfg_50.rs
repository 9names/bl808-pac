#[doc = "Register `phy_cfg_50` reader"]
pub struct R(crate::R<PHY_CFG_50_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_50_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_50_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_50_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_50` writer"]
pub struct W(crate::W<PHY_CFG_50_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_50_SPEC>;
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
impl From<crate::W<PHY_CFG_50_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_50_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dq_oe_up_p_reg` reader - "]
pub type DQ_OE_UP_P_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_up_p_reg` writer - "]
pub type DQ_OE_UP_P_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `dq_oe_up_n_reg` reader - "]
pub type DQ_OE_UP_N_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_up_n_reg` writer - "]
pub type DQ_OE_UP_N_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `dq_oe_mid_p_reg` reader - "]
pub type DQ_OE_MID_P_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_mid_p_reg` writer - "]
pub type DQ_OE_MID_P_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `dq_oe_mid_n_reg` reader - "]
pub type DQ_OE_MID_N_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_mid_n_reg` writer - "]
pub type DQ_OE_MID_N_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `dq_oe_dn_p_reg` reader - "]
pub type DQ_OE_DN_P_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_dn_p_reg` writer - "]
pub type DQ_OE_DN_P_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `dq_oe_dn_n_reg` reader - "]
pub type DQ_OE_DN_N_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq_oe_dn_n_reg` writer - "]
pub type DQ_OE_DN_N_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `phy_wl_cen_ana` reader - "]
pub type PHY_WL_CEN_ANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phy_wl_cen_ana` writer - "]
pub type PHY_WL_CEN_ANA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_50_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_27_31` reader - "]
pub type RESERVED_27_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dq_oe_up_p_reg(&self) -> DQ_OE_UP_P_REG_R {
        DQ_OE_UP_P_REG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dq_oe_up_n_reg(&self) -> DQ_OE_UP_N_REG_R {
        DQ_OE_UP_N_REG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dq_oe_mid_p_reg(&self) -> DQ_OE_MID_P_REG_R {
        DQ_OE_MID_P_REG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dq_oe_mid_n_reg(&self) -> DQ_OE_MID_N_REG_R {
        DQ_OE_MID_N_REG_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dq_oe_dn_p_reg(&self) -> DQ_OE_DN_P_REG_R {
        DQ_OE_DN_P_REG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dq_oe_dn_n_reg(&self) -> DQ_OE_DN_N_REG_R {
        DQ_OE_DN_N_REG_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn phy_wl_cen_ana(&self) -> PHY_WL_CEN_ANA_R {
        PHY_WL_CEN_ANA_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn reserved_27_31(&self) -> RESERVED_27_31_R {
        RESERVED_27_31_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_up_p_reg(&mut self) -> DQ_OE_UP_P_REG_W<0> {
        DQ_OE_UP_P_REG_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_up_n_reg(&mut self) -> DQ_OE_UP_N_REG_W<4> {
        DQ_OE_UP_N_REG_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_mid_p_reg(&mut self) -> DQ_OE_MID_P_REG_W<8> {
        DQ_OE_MID_P_REG_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_mid_n_reg(&mut self) -> DQ_OE_MID_N_REG_W<12> {
        DQ_OE_MID_N_REG_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_dn_p_reg(&mut self) -> DQ_OE_DN_P_REG_W<16> {
        DQ_OE_DN_P_REG_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn dq_oe_dn_n_reg(&mut self) -> DQ_OE_DN_N_REG_W<20> {
        DQ_OE_DN_N_REG_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wl_cen_ana(&mut self) -> PHY_WL_CEN_ANA_W<24> {
        PHY_WL_CEN_ANA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_50\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_50](index.html) module"]
pub struct PHY_CFG_50_SPEC;
impl crate::RegisterSpec for PHY_CFG_50_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_50::R](R) reader structure"]
impl crate::Readable for PHY_CFG_50_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_50::W](W) writer structure"]
impl crate::Writable for PHY_CFG_50_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_50 to value 0x0144_4444"]
impl crate::Resettable for PHY_CFG_50_SPEC {
    const RESET_VALUE: Self::Ux = 0x0144_4444;
}
