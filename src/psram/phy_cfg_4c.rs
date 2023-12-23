#[doc = "Register `phy_cfg_4c` reader"]
pub struct R(crate::R<PHY_CFG_4C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CFG_4C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CFG_4C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CFG_4C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `phy_cfg_4c` writer"]
pub struct W(crate::W<PHY_CFG_4C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CFG_4C_SPEC>;
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
impl From<crate::W<PHY_CFG_4C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CFG_4C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tout_uhs_phy_dig` reader - "]
pub type TOUT_UHS_PHY_DIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `odt_sel_dly` reader - "]
pub type ODT_SEL_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `odt_sel_dly` writer - "]
pub type ODT_SEL_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_CFG_4C_SPEC, u8, u8, 4, O>;
#[doc = "Field `odt_sel_hw` reader - "]
pub type ODT_SEL_HW_R = crate::BitReader<bool>;
#[doc = "Field `odt_sel_hw` writer - "]
pub type ODT_SEL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CFG_4C_SPEC, bool, O>;
#[doc = "Field `reserved_21_31` reader - "]
pub type RESERVED_21_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tout_uhs_phy_dig(&self) -> TOUT_UHS_PHY_DIG_R {
        TOUT_UHS_PHY_DIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn odt_sel_dly(&self) -> ODT_SEL_DLY_R {
        ODT_SEL_DLY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn odt_sel_hw(&self) -> ODT_SEL_HW_R {
        ODT_SEL_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn reserved_21_31(&self) -> RESERVED_21_31_R {
        RESERVED_21_31_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn odt_sel_dly(&mut self) -> ODT_SEL_DLY_W<16> {
        ODT_SEL_DLY_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn odt_sel_hw(&mut self) -> ODT_SEL_HW_W<20> {
        ODT_SEL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "phy_cfg_4C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_cfg_4c](index.html) module"]
pub struct PHY_CFG_4C_SPEC;
impl crate::RegisterSpec for PHY_CFG_4C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_cfg_4c::R](R) reader structure"]
impl crate::Readable for PHY_CFG_4C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_cfg_4c::W](W) writer structure"]
impl crate::Writable for PHY_CFG_4C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_cfg_4c to value 0x0013_0000"]
impl crate::Resettable for PHY_CFG_4C_SPEC {
    const RESET_VALUE: Self::Ux = 0x0013_0000;
}
