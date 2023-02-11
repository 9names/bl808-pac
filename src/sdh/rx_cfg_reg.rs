#[doc = "Register `rx_cfg_reg` reader"]
pub struct R(crate::R<RX_CFG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CFG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CFG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CFG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rx_cfg_reg` writer"]
pub struct W(crate::W<RX_CFG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CFG_REG_SPEC>;
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
impl From<crate::W<RX_CFG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CFG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdclk_sel0` reader - "]
pub type SDCLK_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sdclk_sel0` writer - "]
pub type SDCLK_SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_CFG_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `sdclk_sel1` reader - "]
pub type SDCLK_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sdclk_sel1` writer - "]
pub type SDCLK_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_CFG_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_7_4` reader - "]
pub type RESERVED_7_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sdclk_delay` reader - "]
pub type SDCLK_DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sdclk_delay` writer - "]
pub type SDCLK_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CFG_REG_SPEC, u16, u16, 10, O>;
#[doc = "Field `tuning_dly_inc` reader - "]
pub type TUNING_DLY_INC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tuning_dly_inc` writer - "]
pub type TUNING_DLY_INC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CFG_REG_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_31_28` reader - "]
pub type RESERVED_31_28_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdclk_sel0(&self) -> SDCLK_SEL0_R {
        SDCLK_SEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sdclk_sel1(&self) -> SDCLK_SEL1_R {
        SDCLK_SEL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_7_4(&self) -> RESERVED_7_4_R {
        RESERVED_7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:17"]
    #[inline(always)]
    pub fn sdclk_delay(&self) -> SDCLK_DELAY_R {
        SDCLK_DELAY_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn tuning_dly_inc(&self) -> TUNING_DLY_INC_R {
        TUNING_DLY_INC_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_31_28(&self) -> RESERVED_31_28_R {
        RESERVED_31_28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_sel0(&mut self) -> SDCLK_SEL0_W<0> {
        SDCLK_SEL0_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_sel1(&mut self) -> SDCLK_SEL1_W<2> {
        SDCLK_SEL1_W::new(self)
    }
    #[doc = "Bits 8:17"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_delay(&mut self) -> SDCLK_DELAY_W<8> {
        SDCLK_DELAY_W::new(self)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_dly_inc(&mut self) -> TUNING_DLY_INC_W<18> {
        TUNING_DLY_INC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_cfg_reg](index.html) module"]
pub struct RX_CFG_REG_SPEC;
impl crate::RegisterSpec for RX_CFG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_cfg_reg::R](R) reader structure"]
impl crate::Readable for RX_CFG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_cfg_reg::W](W) writer structure"]
impl crate::Writable for RX_CFG_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rx_cfg_reg to value 0"]
impl crate::Resettable for RX_CFG_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
