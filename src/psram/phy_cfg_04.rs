#[doc = "Register `phy_cfg_04` reader"]
pub struct R(crate::R<PHY_CFG_04_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_04_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_04_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_04_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_04` writer"]
pub struct W(crate::W<PHY_CFG_04_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_04_SPEC>;
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
impl From<crate::W<PHY_CFG_04_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_04_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_3` reader - "]
pub type RESERVED_0_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm1_sr` reader - "]
pub type DM1_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm1_sr` writer - "]
pub type DM1_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_04_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_11` reader - "]
pub type RESERVED_6_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm1_dly_drv` reader - "]
pub type DM1_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm1_dly_drv` writer - "]
pub type DM1_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_04_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_16_19` reader - "]
pub type RESERVED_16_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm0_sr` reader - "]
pub type DM0_SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm0_sr` writer - "]
pub type DM0_SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_CFG_04_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_22_27` reader - "]
pub type RESERVED_22_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm0_dly_drv` reader - "]
pub type DM0_DLY_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dm0_dly_drv` writer - "]
pub type DM0_DLY_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_04_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reserved_0_3(&self) -> RESERVED_0_3_R {
        RESERVED_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dm1_sr(&self) -> DM1_SR_R {
        DM1_SR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn reserved_6_11(&self) -> RESERVED_6_11_R {
        RESERVED_6_11_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dm1_dly_drv(&self) -> DM1_DLY_DRV_R {
        DM1_DLY_DRV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reserved_16_19(&self) -> RESERVED_16_19_R {
        RESERVED_16_19_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dm0_sr(&self) -> DM0_SR_R {
        DM0_SR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn reserved_22_27(&self) -> RESERVED_22_27_R {
        RESERVED_22_27_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dm0_dly_drv(&self) -> DM0_DLY_DRV_R {
        DM0_DLY_DRV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_sr(&mut self) -> DM1_SR_W<4> {
        DM1_SR_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_dly_drv(&mut self) -> DM1_DLY_DRV_W<12> {
        DM1_DLY_DRV_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_sr(&mut self) -> DM0_SR_W<20> {
        DM0_SR_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_dly_drv(&mut self) -> DM0_DLY_DRV_W<28> {
        DM0_DLY_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_04\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_04](index.html) module"]
pub struct PHY_CFG_04_SPEC;
impl crate::RegisterSpec for PHY_CFG_04_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_04::R](R) reader structure"]
impl crate::Readable for PHY_CFG_04_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_04::W](W) writer structure"]
impl crate::Writable for PHY_CFG_04_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_04 to value 0x8000_8000"]
impl crate::Resettable for PHY_CFG_04_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_8000;
}
