#[doc = "Register `rf_cfg0` reader"]
pub struct R(crate::R<RF_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cfg0` writer"]
pub struct W(crate::W<RF_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CFG0_SPEC>;
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
impl From<crate::W<RF_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_8` reader - "]
pub type RESERVED_0_8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cfg_inv_rf2_test_clk_o` reader - "]
pub type CFG_INV_RF2_TEST_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_rf2_test_clk_o` writer - "]
pub type CFG_INV_RF2_TEST_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_10_31` reader - "]
pub type RESERVED_10_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reserved_0_8(&self) -> RESERVED_0_8_R {
        RESERVED_0_8_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf2_test_clk_o(&self) -> CFG_INV_RF2_TEST_CLK_O_R {
        CFG_INV_RF2_TEST_CLK_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn reserved_10_31(&self) -> RESERVED_10_31_R {
        RESERVED_10_31_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_rf2_test_clk_o(&mut self) -> CFG_INV_RF2_TEST_CLK_O_W<9> {
        CFG_INV_RF2_TEST_CLK_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cfg0](index.html) module"]
pub struct RF_CFG0_SPEC;
impl crate::RegisterSpec for RF_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cfg0::R](R) reader structure"]
impl crate::Readable for RF_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cfg0::W](W) writer structure"]
impl crate::Writable for RF_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_cfg0 to value 0x0200"]
impl crate::Resettable for RF_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
