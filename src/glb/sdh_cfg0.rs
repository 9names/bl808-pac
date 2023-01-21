#[doc = "Register `sdh_cfg0` reader"]
pub struct R(crate::R<SDH_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDH_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDH_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDH_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdh_cfg0` writer"]
pub struct W(crate::W<SDH_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDH_CFG0_SPEC>;
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
impl From<crate::W<SDH_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDH_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_8` reader - "]
pub type RESERVED_0_8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_sdh_clk_div` reader - "]
pub type REG_SDH_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_sdh_clk_div` writer - "]
pub type REG_SDH_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDH_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_sdh_clk_sel` reader - "]
pub type REG_SDH_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_sdh_clk_sel` writer - "]
pub type REG_SDH_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDH_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_sdh_clk_en` reader - "]
pub type REG_SDH_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_sdh_clk_en` writer - "]
pub type REG_SDH_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDH_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_14_31` reader - "]
pub type RESERVED_14_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reserved_0_8(&self) -> RESERVED_0_8_R {
        RESERVED_0_8_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reg_sdh_clk_div(&self) -> REG_SDH_CLK_DIV_R {
        REG_SDH_CLK_DIV_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_sdh_clk_sel(&self) -> REG_SDH_CLK_SEL_R {
        REG_SDH_CLK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_sdh_clk_en(&self) -> REG_SDH_CLK_EN_R {
        REG_SDH_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn reserved_14_31(&self) -> RESERVED_14_31_R {
        RESERVED_14_31_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_div(&mut self) -> REG_SDH_CLK_DIV_W<9> {
        REG_SDH_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_sel(&mut self) -> REG_SDH_CLK_SEL_W<12> {
        REG_SDH_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_en(&mut self) -> REG_SDH_CLK_EN_W<13> {
        REG_SDH_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdh_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdh_cfg0](index.html) module"]
pub struct SDH_CFG0_SPEC;
impl crate::RegisterSpec for SDH_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdh_cfg0::R](R) reader structure"]
impl crate::Readable for SDH_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdh_cfg0::W](W) writer structure"]
impl crate::Writable for SDH_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdh_cfg0 to value 0x2000"]
impl crate::Resettable for SDH_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
