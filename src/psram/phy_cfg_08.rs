#[doc = "Register `phy_cfg_08` reader"]
pub struct R(crate::R<PHY_CFG_08_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_08_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_08_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_08_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_08` writer"]
pub struct W(crate::W<PHY_CFG_08_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_08_SPEC>;
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
impl From<crate::W<PHY_CFG_08_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_08_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dq1_sr` reader - "]
pub type DQ1_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq1_sr` writer - "]
pub type DQ1_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_7` reader - "]
pub type RESERVED_2_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq1_dly_rx` reader - "]
pub type DQ1_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq1_dly_rx` writer - "]
pub type DQ1_DLY_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq1_dly_drv` reader - "]
pub type DQ1_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq1_dly_drv` writer - "]
pub type DQ1_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq0_sr` reader - "]
pub type DQ0_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq0_sr` writer - "]
pub type DQ0_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_23` reader - "]
pub type RESERVED_18_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq0_dly_rx` reader - "]
pub type DQ0_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq0_dly_rx` writer - "]
pub type DQ0_DLY_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq0_dly_drv` reader - "]
pub type DQ0_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq0_dly_drv` writer - "]
pub type DQ0_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_08_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dq1_sr(&self) -> DQ1_SR_R {
        DQ1_SR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn reserved_2_7(&self) -> RESERVED_2_7_R {
        RESERVED_2_7_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dq1_dly_rx(&self) -> DQ1_DLY_RX_R {
        DQ1_DLY_RX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dq1_dly_drv(&self) -> DQ1_DLY_DRV_R {
        DQ1_DLY_DRV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dq0_sr(&self) -> DQ0_SR_R {
        DQ0_SR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn reserved_18_23(&self) -> RESERVED_18_23_R {
        RESERVED_18_23_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dq0_dly_rx(&self) -> DQ0_DLY_RX_R {
        DQ0_DLY_RX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dq0_dly_drv(&self) -> DQ0_DLY_DRV_R {
        DQ0_DLY_DRV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dq1_sr(&mut self) -> DQ1_SR_W<0> {
        DQ1_SR_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dq1_dly_rx(&mut self) -> DQ1_DLY_RX_W<8> {
        DQ1_DLY_RX_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dq1_dly_drv(&mut self) -> DQ1_DLY_DRV_W<12> {
        DQ1_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn dq0_sr(&mut self) -> DQ0_SR_W<16> {
        DQ0_SR_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dq0_dly_rx(&mut self) -> DQ0_DLY_RX_W<24> {
        DQ0_DLY_RX_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn dq0_dly_drv(&mut self) -> DQ0_DLY_DRV_W<28> {
        DQ0_DLY_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_08\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_08](index.html) module"]
pub struct PHY_CFG_08_SPEC;
impl crate::RegisterSpec for PHY_CFG_08_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_08::R](R) reader structure"]
impl crate::Readable for PHY_CFG_08_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_08::W](W) writer structure"]
impl crate::Writable for PHY_CFG_08_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_08 to value 0x8100_8100"]
impl crate::Resettable for PHY_CFG_08_SPEC {
    const RESET_VALUE: Self::Ux = 0x8100_8100;
}
