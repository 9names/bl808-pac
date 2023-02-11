#[doc = "Register `phy_cfg_1c` reader"]
pub struct R(crate::R<PHY_CFG_1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_1c` writer"]
pub struct W(crate::W<PHY_CFG_1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_1C_SPEC>;
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
impl From<crate::W<PHY_CFG_1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dq11_sr` reader - "]
pub type DQ11_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq11_sr` writer - "]
pub type DQ11_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_7` reader - "]
pub type RESERVED_2_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq11_dly_rx` reader - "]
pub type DQ11_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq11_dly_rx` writer - "]
pub type DQ11_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq11_dly_drv` reader - "]
pub type DQ11_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq11_dly_drv` writer - "]
pub type DQ11_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq10_sr` reader - "]
pub type DQ10_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq10_sr` writer - "]
pub type DQ10_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_23` reader - "]
pub type RESERVED_18_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq10_dly_rx` reader - "]
pub type DQ10_DLY_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq10_dly_rx` writer - "]
pub type DQ10_DLY_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 4, O>;
#[doc = "Field `dq10_dly_drv` reader - "]
pub type DQ10_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dq10_dly_drv` writer - "]
pub type DQ10_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_1C_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dq11_sr(&self) -> DQ11_SR_R {
        DQ11_SR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn reserved_2_7(&self) -> RESERVED_2_7_R {
        RESERVED_2_7_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dq11_dly_rx(&self) -> DQ11_DLY_RX_R {
        DQ11_DLY_RX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dq11_dly_drv(&self) -> DQ11_DLY_DRV_R {
        DQ11_DLY_DRV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dq10_sr(&self) -> DQ10_SR_R {
        DQ10_SR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn reserved_18_23(&self) -> RESERVED_18_23_R {
        RESERVED_18_23_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dq10_dly_rx(&self) -> DQ10_DLY_RX_R {
        DQ10_DLY_RX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dq10_dly_drv(&self) -> DQ10_DLY_DRV_R {
        DQ10_DLY_DRV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dq11_sr(&mut self) -> DQ11_SR_W<0> {
        DQ11_SR_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dq11_dly_rx(&mut self) -> DQ11_DLY_RX_W<8> {
        DQ11_DLY_RX_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dq11_dly_drv(&mut self) -> DQ11_DLY_DRV_W<12> {
        DQ11_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn dq10_sr(&mut self) -> DQ10_SR_W<16> {
        DQ10_SR_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dq10_dly_rx(&mut self) -> DQ10_DLY_RX_W<24> {
        DQ10_DLY_RX_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn dq10_dly_drv(&mut self) -> DQ10_DLY_DRV_W<28> {
        DQ10_DLY_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_1C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_1c](index.html) module"]
pub struct PHY_CFG_1C_SPEC;
impl crate::RegisterSpec for PHY_CFG_1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_1c::R](R) reader structure"]
impl crate::Readable for PHY_CFG_1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_1c::W](W) writer structure"]
impl crate::Writable for PHY_CFG_1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_1c to value 0x8100_8100"]
impl crate::Resettable for PHY_CFG_1C_SPEC {
    const RESET_VALUE: Self::Ux = 0x8100_8100;
}
