#[doc = "Register `sf_cfg0` reader"]
pub struct R(crate::R<SF_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_cfg0` writer"]
pub struct W(crate::W<SF_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CFG0_SPEC>;
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
impl From<crate::W<SF_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_7` reader - "]
pub type RESERVED_0_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_div` reader - "]
pub type SF_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_div` writer - "]
pub type SF_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SF_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_clk_en` reader - "]
pub type SF_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_en` writer - "]
pub type SF_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CFG0_SPEC, bool, O>;
#[doc = "Field `sf_clk_sel` reader - "]
pub type SF_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_sel` writer - "]
pub type SF_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SF_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf_clk_sel2` reader - "]
pub type SF_CLK_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_sel2` writer - "]
pub type SF_CLK_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SF_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reserved_0_7(&self) -> RESERVED_0_7_R {
        RESERVED_0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SF_CLK_DIV_R {
        SF_CLK_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SF_CLK_EN_R {
        SF_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SF_CLK_SEL_R {
        SF_CLK_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SF_CLK_SEL2_R {
        SF_CLK_SEL2_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_div(&mut self) -> SF_CLK_DIV_W<8> {
        SF_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_en(&mut self) -> SF_CLK_EN_W<11> {
        SF_CLK_EN_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sel(&mut self) -> SF_CLK_SEL_W<12> {
        SF_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sel2(&mut self) -> SF_CLK_SEL2_W<14> {
        SF_CLK_SEL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_cfg0](index.html) module"]
pub struct SF_CFG0_SPEC;
impl crate::RegisterSpec for SF_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_cfg0::R](R) reader structure"]
impl crate::Readable for SF_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_cfg0::W](W) writer structure"]
impl crate::Writable for SF_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_cfg0 to value 0x2b00"]
impl crate::Resettable for SF_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b00;
}
