#[doc = "Register `phy_cfg_28` reader"]
pub struct R(crate::R<PHY_CFG_28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_28` writer"]
pub struct W(crate::W<PHY_CFG_28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_28_SPEC>;
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
impl From<crate::W<PHY_CFG_28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_7` reader - "]
pub type RESERVED_0_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0n_dly_rx` reader - "]
pub type DQS0N_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0n_dly_rx` writer - "]
pub type DQS0N_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs0_sr` reader - "]
pub type DQS0_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_sr` writer - "]
pub type DQS0_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 2, O>;
#[doc = "Field `dqs0_sel` reader - "]
pub type DQS0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_sel` writer - "]
pub type DQS0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_16_19` reader - "]
pub type RESERVED_16_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_dly_rx` reader - "]
pub type DQS0_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_dly_rx` writer - "]
pub type DQS0_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs0_dly_drv` reader - "]
pub type DQS0_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_dly_drv` writer - "]
pub type DQS0_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs0_diff_dly_rx` reader - "]
pub type DQS0_DIFF_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs0_diff_dly_rx` writer - "]
pub type DQS0_DIFF_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_28_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reserved_0_7(&self) -> RESERVED_0_7_R {
        RESERVED_0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dqs0n_dly_rx(&self) -> DQS0N_DLY_RX_R {
        DQS0N_DLY_RX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dqs0_sr(&self) -> DQS0_SR_R {
        DQS0_SR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dqs0_sel(&self) -> DQS0_SEL_R {
        DQS0_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reserved_16_19(&self) -> RESERVED_16_19_R {
        RESERVED_16_19_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dqs0_dly_rx(&self) -> DQS0_DLY_RX_R {
        DQS0_DLY_RX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dqs0_dly_drv(&self) -> DQS0_DLY_DRV_R {
        DQS0_DLY_DRV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dqs0_diff_dly_rx(&self) -> DQS0_DIFF_DLY_RX_R {
        DQS0_DIFF_DLY_RX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0n_dly_rx(&mut self) -> DQS0N_DLY_RX_W<8> {
        DQS0N_DLY_RX_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0_sr(&mut self) -> DQS0_SR_W<12> {
        DQS0_SR_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0_sel(&mut self) -> DQS0_SEL_W<14> {
        DQS0_SEL_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0_dly_rx(&mut self) -> DQS0_DLY_RX_W<20> {
        DQS0_DLY_RX_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0_dly_drv(&mut self) -> DQS0_DLY_DRV_W<24> {
        DQS0_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn dqs0_diff_dly_rx(&mut self) -> DQS0_DIFF_DLY_RX_W<28> {
        DQS0_DIFF_DLY_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_28](index.html) module"]
pub struct PHY_CFG_28_SPEC;
impl crate::RegisterSpec for PHY_CFG_28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_28::R](R) reader structure"]
impl crate::Readable for PHY_CFG_28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_28::W](W) writer structure"]
impl crate::Writable for PHY_CFG_28_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_28 to value 0x3830_0300"]
impl crate::Resettable for PHY_CFG_28_SPEC {
    const RESET_VALUE: Self::Ux = 0x3830_0300;
}
