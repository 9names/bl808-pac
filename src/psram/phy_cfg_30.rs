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
#[doc = "Field `ipp5un_lpddr` reader - "]
pub type IPP5UN_LPDDR_R = crate::BitReader<bool>;
#[doc = "Field `ipp5un_lpddr` writer - "]
pub type IPP5UN_LPDDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_30_SPEC, bool, O>;
#[doc = "Field `en_rx_fe` reader - "]
pub type EN_RX_FE_R = crate::BitReader<bool>;
#[doc = "Field `en_rx_fe` writer - "]
pub type EN_RX_FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_30_SPEC, bool, O>;
#[doc = "Field `en_bias` reader - "]
pub type EN_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `en_bias` writer - "]
pub type EN_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_30_SPEC, bool, O>;
#[doc = "Field `reserved_3_7` reader - "]
pub type RESERVED_3_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1n_dly_rx` reader - "]
pub type DQS1N_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1n_dly_rx` writer - "]
pub type DQS1N_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs1_sr` reader - "]
pub type DQS1_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_sr` writer - "]
pub type DQS1_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 2, O>;
#[doc = "Field `dqs1_sel` reader - "]
pub type DQS1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_sel` writer - "]
pub type DQS1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_16_19` reader - "]
pub type RESERVED_16_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_dly_rx` reader - "]
pub type DQS1_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_dly_rx` writer - "]
pub type DQS1_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs1_dly_drv` reader - "]
pub type DQS1_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_dly_drv` writer - "]
pub type DQS1_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
#[doc = "Field `dqs1_diff_dly_rx` reader - "]
pub type DQS1_DIFF_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dqs1_diff_dly_rx` writer - "]
pub type DQS1_DIFF_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_30_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ipp5un_lpddr(&self) -> IPP5UN_LPDDR_R {
        IPP5UN_LPDDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_rx_fe(&self) -> EN_RX_FE_R {
        EN_RX_FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_bias(&self) -> EN_BIAS_R {
        EN_BIAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn reserved_3_7(&self) -> RESERVED_3_7_R {
        RESERVED_3_7_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dqs1n_dly_rx(&self) -> DQS1N_DLY_RX_R {
        DQS1N_DLY_RX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dqs1_sr(&self) -> DQS1_SR_R {
        DQS1_SR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dqs1_sel(&self) -> DQS1_SEL_R {
        DQS1_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reserved_16_19(&self) -> RESERVED_16_19_R {
        RESERVED_16_19_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dqs1_dly_rx(&self) -> DQS1_DLY_RX_R {
        DQS1_DLY_RX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dqs1_dly_drv(&self) -> DQS1_DLY_DRV_R {
        DQS1_DLY_DRV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dqs1_diff_dly_rx(&self) -> DQS1_DIFF_DLY_RX_R {
        DQS1_DIFF_DLY_RX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ipp5un_lpddr(&mut self) -> IPP5UN_LPDDR_W<0> {
        IPP5UN_LPDDR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn en_rx_fe(&mut self) -> EN_RX_FE_W<1> {
        EN_RX_FE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn en_bias(&mut self) -> EN_BIAS_W<2> {
        EN_BIAS_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1n_dly_rx(&mut self) -> DQS1N_DLY_RX_W<8> {
        DQS1N_DLY_RX_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1_sr(&mut self) -> DQS1_SR_W<12> {
        DQS1_SR_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1_sel(&mut self) -> DQS1_SEL_W<14> {
        DQS1_SEL_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1_dly_rx(&mut self) -> DQS1_DLY_RX_W<20> {
        DQS1_DLY_RX_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1_dly_drv(&mut self) -> DQS1_DLY_DRV_W<24> {
        DQS1_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn dqs1_diff_dly_rx(&mut self) -> DQS1_DIFF_DLY_RX_W<28> {
        DQS1_DIFF_DLY_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_2C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_30](index.html) module"]
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
#[doc = "`reset()` method sets phy_cfg_30 to value 0x3830_0306"]
impl crate::Resettable for PHY_CFG_30_SPEC {
    const RESET_VALUE: Self::Ux = 0x3830_0306;
}
