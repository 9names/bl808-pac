#[doc = "Register `eth_cfg0` reader"]
pub struct R(crate::R<ETH_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `eth_cfg0` writer"]
pub struct W(crate::W<ETH_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_CFG0_SPEC>;
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
impl From<crate::W<ETH_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_4` reader - "]
pub type RESERVED_0_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` reader - "]
pub type CFG_SEL_ETH_REF_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` writer - "]
pub type CFG_SEL_ETH_REF_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_CFG0_SPEC, bool, O>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` reader - "]
pub type CFG_INV_ETH_REF_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` writer - "]
pub type CFG_INV_ETH_REF_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_CFG0_SPEC, bool, O>;
#[doc = "Field `cfg_inv_eth_tx_clk` reader - "]
pub type CFG_INV_ETH_TX_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_tx_clk` writer - "]
pub type CFG_INV_ETH_TX_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_8_9` reader - "]
pub type RESERVED_8_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cfg_inv_eth_rx_clk` reader - "]
pub type CFG_INV_ETH_RX_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_rx_clk` writer - "]
pub type CFG_INV_ETH_RX_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reserved_0_4(&self) -> RESERVED_0_4_R {
        RESERVED_0_4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&self) -> CFG_SEL_ETH_REF_CLK_O_R {
        CFG_SEL_ETH_REF_CLK_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&self) -> CFG_INV_ETH_REF_CLK_O_R {
        CFG_INV_ETH_REF_CLK_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&self) -> CFG_INV_ETH_TX_CLK_R {
        CFG_INV_ETH_TX_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reserved_8_9(&self) -> RESERVED_8_9_R {
        RESERVED_8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&self) -> CFG_INV_ETH_RX_CLK_R {
        CFG_INV_ETH_RX_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sel_eth_ref_clk_o(&mut self) -> CFG_SEL_ETH_REF_CLK_O_W<5> {
        CFG_SEL_ETH_REF_CLK_O_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_ref_clk_o(&mut self) -> CFG_INV_ETH_REF_CLK_O_W<6> {
        CFG_INV_ETH_REF_CLK_O_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_tx_clk(&mut self) -> CFG_INV_ETH_TX_CLK_W<7> {
        CFG_INV_ETH_TX_CLK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_rx_clk(&mut self) -> CFG_INV_ETH_RX_CLK_W<10> {
        CFG_INV_ETH_RX_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eth_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_cfg0](index.html) module"]
pub struct ETH_CFG0_SPEC;
impl crate::RegisterSpec for ETH_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_cfg0::R](R) reader structure"]
impl crate::Readable for ETH_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_cfg0::W](W) writer structure"]
impl crate::Writable for ETH_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets eth_cfg0 to value 0x04c0"]
impl crate::Resettable for ETH_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x04c0;
}
