#[doc = "Register `mcu_bus_cfg0` reader"]
pub struct R(crate::R<MCU_BUS_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU_BUS_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU_BUS_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU_BUS_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu_bus_cfg0` writer"]
pub struct W(crate::W<MCU_BUS_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU_BUS_CFG0_SPEC>;
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
impl From<crate::W<MCU_BUS_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU_BUS_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mcu_infra_timeout_en` reader - "]
pub type REG_MCU_INFRA_TIMEOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_infra_timeout_en` writer - "]
pub type REG_MCU_INFRA_TIMEOUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCU_BUS_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_mcu_infra_timeout_clr` reader - "]
pub type REG_MCU_INFRA_TIMEOUT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_infra_timeout_clr` writer - "]
pub type REG_MCU_INFRA_TIMEOUT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCU_BUS_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_2_15` reader - "]
pub type RESERVED_2_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_mcu_infra_timeout` reader - "]
pub type STS_MCU_INFRA_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_en(&self) -> REG_MCU_INFRA_TIMEOUT_EN_R {
        REG_MCU_INFRA_TIMEOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_clr(&self) -> REG_MCU_INFRA_TIMEOUT_CLR_R {
        REG_MCU_INFRA_TIMEOUT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn reserved_2_15(&self) -> RESERVED_2_15_R {
        RESERVED_2_15_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_mcu_infra_timeout(&self) -> STS_MCU_INFRA_TIMEOUT_R {
        STS_MCU_INFRA_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_timeout_en(&mut self) -> REG_MCU_INFRA_TIMEOUT_EN_W<0> {
        REG_MCU_INFRA_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_timeout_clr(&mut self) -> REG_MCU_INFRA_TIMEOUT_CLR_W<1> {
        REG_MCU_INFRA_TIMEOUT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu_bus_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_bus_cfg0](index.html) module"]
pub struct MCU_BUS_CFG0_SPEC;
impl crate::RegisterSpec for MCU_BUS_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu_bus_cfg0::R](R) reader structure"]
impl crate::Readable for MCU_BUS_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu_bus_cfg0::W](W) writer structure"]
impl crate::Writable for MCU_BUS_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_bus_cfg0 to value 0"]
impl crate::Resettable for MCU_BUS_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
