#[doc = "Register `tx_cfg_reg` reader"]
pub struct R(crate::R<TX_CFG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CFG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CFG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CFG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tx_cfg_reg` writer"]
pub struct W(crate::W<TX_CFG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CFG_REG_SPEC>;
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
impl From<crate::W<TX_CFG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CFG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_hold_delay0` reader - "]
pub type TX_HOLD_DELAY0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_hold_delay0` writer - "]
pub type TX_HOLD_DELAY0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CFG_REG_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_15_10` reader - "]
pub type RESERVED_15_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_hold_delay1` reader - "]
pub type TX_HOLD_DELAY1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_hold_delay1` writer - "]
pub type TX_HOLD_DELAY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CFG_REG_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_29_26` reader - "]
pub type RESERVED_29_26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_int_clk_sel` reader - "]
pub type TX_INT_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tx_int_clk_sel` writer - "]
pub type TX_INT_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CFG_REG_SPEC, bool, O>;
#[doc = "Field `tx_mux_sel` reader - "]
pub type TX_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tx_mux_sel` writer - "]
pub type TX_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CFG_REG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_hold_delay0(&self) -> TX_HOLD_DELAY0_R {
        TX_HOLD_DELAY0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_15_10(&self) -> RESERVED_15_10_R {
        RESERVED_15_10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_hold_delay1(&self) -> TX_HOLD_DELAY1_R {
        TX_HOLD_DELAY1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn reserved_29_26(&self) -> RESERVED_29_26_R {
        RESERVED_29_26_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_int_clk_sel(&self) -> TX_INT_CLK_SEL_R {
        TX_INT_CLK_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_mux_sel(&self) -> TX_MUX_SEL_R {
        TX_MUX_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hold_delay0(&mut self) -> TX_HOLD_DELAY0_W<0> {
        TX_HOLD_DELAY0_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hold_delay1(&mut self) -> TX_HOLD_DELAY1_W<16> {
        TX_HOLD_DELAY1_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_int_clk_sel(&mut self) -> TX_INT_CLK_SEL_W<30> {
        TX_INT_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mux_sel(&mut self) -> TX_MUX_SEL_W<31> {
        TX_MUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cfg_reg](index.html) module"]
pub struct TX_CFG_REG_SPEC;
impl crate::RegisterSpec for TX_CFG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_cfg_reg::R](R) reader structure"]
impl crate::Readable for TX_CFG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_cfg_reg::W](W) writer structure"]
impl crate::Writable for TX_CFG_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_cfg_reg to value 0x0029_0070"]
impl crate::Resettable for TX_CFG_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0029_0070;
}
