#[doc = "Register `dig_clk_cfg3` reader"]
pub struct R(crate::R<DIG_CLK_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_CLK_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_CLK_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_CLK_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dig_clk_cfg3` writer"]
pub struct W(crate::W<DIG_CLK_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_CLK_CFG3_SPEC>;
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
impl From<crate::W<DIG_CLK_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_CLK_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_txclkesc_sel` reader - "]
pub type DSI_TXCLKESC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dsi_txclkesc_sel` writer - "]
pub type DSI_TXCLKESC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `csi_txclkesc_sel` reader - "]
pub type CSI_TXCLKESC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `csi_txclkesc_sel` writer - "]
pub type CSI_TXCLKESC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_2_31` reader - "]
pub type RESERVED_2_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dsi_txclkesc_sel(&self) -> DSI_TXCLKESC_SEL_R {
        DSI_TXCLKESC_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn csi_txclkesc_sel(&self) -> CSI_TXCLKESC_SEL_R {
        CSI_TXCLKESC_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn reserved_2_31(&self) -> RESERVED_2_31_R {
        RESERVED_2_31_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_txclkesc_sel(&mut self) -> DSI_TXCLKESC_SEL_W<0> {
        DSI_TXCLKESC_SEL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn csi_txclkesc_sel(&mut self) -> CSI_TXCLKESC_SEL_W<1> {
        CSI_TXCLKESC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dig_clk_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_clk_cfg3](index.html) module"]
pub struct DIG_CLK_CFG3_SPEC;
impl crate::RegisterSpec for DIG_CLK_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_clk_cfg3::R](R) reader structure"]
impl crate::Readable for DIG_CLK_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_clk_cfg3::W](W) writer structure"]
impl crate::Writable for DIG_CLK_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dig_clk_cfg3 to value 0"]
impl crate::Resettable for DIG_CLK_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
