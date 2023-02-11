#[doc = "Register `phy_cfg_00` reader"]
pub struct R(crate::R<PHY_CFG_00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_00` writer"]
pub struct W(crate::W<PHY_CFG_00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_00_SPEC>;
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
impl From<crate::W<PHY_CFG_00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dqs_rdy` reader - "]
pub type DQS_RDY_R = crate::BitReader<bool>;
#[doc = "Field `reserved_1_7` reader - "]
pub type RESERVED_1_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ck_sr` reader - "]
pub type CK_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ck_sr` writer - "]
pub type CK_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_00_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_10_14` reader - "]
pub type RESERVED_10_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk0_polarity` reader - "]
pub type CLK0_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `clk0_polarity` writer - "]
pub type CLK0_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_00_SPEC, bool, O>;
#[doc = "Field `ck_dly_drv` reader - "]
pub type CK_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ck_dly_drv` writer - "]
pub type CK_DLY_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_00_SPEC, u8, u8, 4, O>;
#[doc = "Field `cen_sr` reader - "]
pub type CEN_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cen_sr` writer - "]
pub type CEN_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_00_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_22_27` reader - "]
pub type RESERVED_22_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cen_dly_drv` reader - "]
pub type CEN_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cen_dly_drv` writer - "]
pub type CEN_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_00_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dqs_rdy(&self) -> DQS_RDY_R {
        DQS_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn reserved_1_7(&self) -> RESERVED_1_7_R {
        RESERVED_1_7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ck_sr(&self) -> CK_SR_R {
        CK_SR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn reserved_10_14(&self) -> RESERVED_10_14_R {
        RESERVED_10_14_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk0_polarity(&self) -> CLK0_POLARITY_R {
        CLK0_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ck_dly_drv(&self) -> CK_DLY_DRV_R {
        CK_DLY_DRV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn cen_sr(&self) -> CEN_SR_R {
        CEN_SR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn reserved_22_27(&self) -> RESERVED_22_27_R {
        RESERVED_22_27_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cen_dly_drv(&self) -> CEN_DLY_DRV_R {
        CEN_DLY_DRV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ck_sr(&mut self) -> CK_SR_W<8> {
        CK_SR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn clk0_polarity(&mut self) -> CLK0_POLARITY_W<15> {
        CLK0_POLARITY_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn ck_dly_drv(&mut self) -> CK_DLY_DRV_W<16> {
        CK_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn cen_sr(&mut self) -> CEN_SR_W<20> {
        CEN_SR_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn cen_dly_drv(&mut self) -> CEN_DLY_DRV_W<28> {
        CEN_DLY_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_00\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_00](index.html) module"]
pub struct PHY_CFG_00_SPEC;
impl crate::RegisterSpec for PHY_CFG_00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_00::R](R) reader structure"]
impl crate::Readable for PHY_CFG_00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_00::W](W) writer structure"]
impl crate::Writable for PHY_CFG_00_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_00 to value 0x8008_0000"]
impl crate::Resettable for PHY_CFG_00_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008_0000;
}
