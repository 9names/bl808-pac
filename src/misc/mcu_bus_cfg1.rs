#[doc = "Register `mcu_bus_cfg1` reader"]
pub struct R(crate::R<MCU_BUS_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU_BUS_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU_BUS_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU_BUS_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu_bus_cfg1` writer"]
pub struct W(crate::W<MCU_BUS_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU_BUS_CFG1_SPEC>;
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
impl From<crate::W<MCU_BUS_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU_BUS_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mcu1_hqos` reader - "]
pub type REG_MCU1_HQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_hqos` writer - "]
pub type REG_MCU1_HQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_BUS_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mcu1_awqos` reader - "]
pub type REG_MCU1_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_awqos` writer - "]
pub type REG_MCU1_AWQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_BUS_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mcu1_arqos` reader - "]
pub type REG_MCU1_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_arqos` writer - "]
pub type REG_MCU1_ARQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_BUS_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mcu_x2hs_sp_bypass` reader - "]
pub type REG_MCU_X2HS_SP_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_x2hs_sp_bypass` writer - "]
pub type REG_MCU_X2HS_SP_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCU_BUS_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_4_6` reader - "]
pub type RESERVED_4_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_mcu2ext` reader - "]
pub type REG_X_WTHRE_MCU2EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_mcu2ext` writer - "]
pub type REG_X_WTHRE_MCU2EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCU_BUS_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_9_15` reader - "]
pub type RESERVED_9_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mcu_infra_arb_mode` reader - "]
pub type REG_MCU_INFRA_ARB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_infra_arb_mode` writer - "]
pub type REG_MCU_INFRA_ARB_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCU_BUS_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mcu1_hqos(&self) -> REG_MCU1_HQOS_R {
        REG_MCU1_HQOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu1_awqos(&self) -> REG_MCU1_AWQOS_R {
        REG_MCU1_AWQOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_mcu1_arqos(&self) -> REG_MCU1_ARQOS_R {
        REG_MCU1_ARQOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_mcu_x2hs_sp_bypass(&self) -> REG_MCU_X2HS_SP_BYPASS_R {
        REG_MCU_X2HS_SP_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn reserved_4_6(&self) -> RESERVED_4_6_R {
        RESERVED_4_6_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn reg_x_wthre_mcu2ext(&self) -> REG_X_WTHRE_MCU2EXT_R {
        REG_X_WTHRE_MCU2EXT_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn reserved_9_15(&self) -> RESERVED_9_15_R {
        RESERVED_9_15_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_mcu_infra_arb_mode(&self) -> REG_MCU_INFRA_ARB_MODE_R {
        REG_MCU_INFRA_ARB_MODE_R::new(((self.bits >> 16) & 1) != 0)
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
    pub fn reg_mcu1_hqos(&mut self) -> REG_MCU1_HQOS_W<0> {
        REG_MCU1_HQOS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_awqos(&mut self) -> REG_MCU1_AWQOS_W<1> {
        REG_MCU1_AWQOS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_arqos(&mut self) -> REG_MCU1_ARQOS_W<2> {
        REG_MCU1_ARQOS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_x2hs_sp_bypass(&mut self) -> REG_MCU_X2HS_SP_BYPASS_W<3> {
        REG_MCU_X2HS_SP_BYPASS_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_mcu2ext(&mut self) -> REG_X_WTHRE_MCU2EXT_W<7> {
        REG_X_WTHRE_MCU2EXT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_arb_mode(&mut self) -> REG_MCU_INFRA_ARB_MODE_W<16> {
        REG_MCU_INFRA_ARB_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu_bus_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_bus_cfg1](index.html) module"]
pub struct MCU_BUS_CFG1_SPEC;
impl crate::RegisterSpec for MCU_BUS_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu_bus_cfg1::R](R) reader structure"]
impl crate::Readable for MCU_BUS_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu_bus_cfg1::W](W) writer structure"]
impl crate::Writable for MCU_BUS_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_bus_cfg1 to value 0"]
impl crate::Resettable for MCU_BUS_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
