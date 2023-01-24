#[doc = "Register `phy_cfg_34` reader"]
pub struct R(crate::R<PHY_CFG_34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_34` writer"]
pub struct W(crate::W<PHY_CFG_34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_34_SPEC>;
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
impl From<crate::W<PHY_CFG_34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_timer_dqs_start` reader - "]
pub type REG_TIMER_DQS_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_timer_dqs_start` writer - "]
pub type REG_TIMER_DQS_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_34_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_timer_dqs_start(&self) -> REG_TIMER_DQS_START_R {
        REG_TIMER_DQS_START_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timer_dqs_start(&mut self) -> REG_TIMER_DQS_START_W<0> {
        REG_TIMER_DQS_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_34\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_34](index.html) module"]
pub struct PHY_CFG_34_SPEC;
impl crate::RegisterSpec for PHY_CFG_34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_34::R](R) reader structure"]
impl crate::Readable for PHY_CFG_34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_34::W](W) writer structure"]
impl crate::Writable for PHY_CFG_34_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_34 to value 0x01"]
impl crate::Resettable for PHY_CFG_34_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
