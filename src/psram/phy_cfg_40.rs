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
#[doc = "Field `reg_timer_self_refresh1_in` reader - "]
pub type REG_TIMER_SELF_REFRESH1_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_timer_self_refresh1_in` writer - "]
pub type REG_TIMER_SELF_REFRESH1_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_timer_self_refresh1_exit` reader - "]
pub type REG_TIMER_SELF_REFRESH1_EXIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_timer_self_refresh1_exit` writer - "]
pub type REG_TIMER_SELF_REFRESH1_EXIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_timer_global_rst` reader - "]
pub type REG_TIMER_GLOBAL_RST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_timer_global_rst` writer - "]
pub type REG_TIMER_GLOBAL_RST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_40_SPEC, u16, u16, 14, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_in(&self) -> REG_TIMER_SELF_REFRESH1_IN_R {
        REG_TIMER_SELF_REFRESH1_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_exit(&self) -> REG_TIMER_SELF_REFRESH1_EXIT_R {
        REG_TIMER_SELF_REFRESH1_EXIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn reg_timer_global_rst(&self) -> REG_TIMER_GLOBAL_RST_R {
        REG_TIMER_GLOBAL_RST_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timer_self_refresh1_in(&mut self) -> REG_TIMER_SELF_REFRESH1_IN_W<0> {
        REG_TIMER_SELF_REFRESH1_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timer_self_refresh1_exit(&mut self) -> REG_TIMER_SELF_REFRESH1_EXIT_W<8> {
        REG_TIMER_SELF_REFRESH1_EXIT_W::new(self)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timer_global_rst(&mut self) -> REG_TIMER_GLOBAL_RST_W<16> {
        REG_TIMER_GLOBAL_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_3C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_40](index.html) module"]
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
#[doc = "`reset()` method sets phy_cfg_40 to value 0x0272_0808"]
impl crate::Resettable for PHY_CFG_40_SPEC {
    const RESET_VALUE: Self::Ux = 0x0272_0808;
}
