#[doc = "Register `emi_cfg0` reader"]
pub struct R(crate::R<EMI_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMI_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMI_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMI_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emi_cfg0` writer"]
pub struct W(crate::W<EMI_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMI_CFG0_SPEC>;
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
impl From<crate::W<EMI_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMI_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_8` reader - "]
pub type RESERVED_0_8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_emi_clk_en` reader - "]
pub type REG_EMI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_emi_clk_en` writer - "]
pub type REG_EMI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMI_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_10_13` reader - "]
pub type RESERVED_10_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_emi_clk_sel` reader - "]
pub type REG_EMI_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_emi_clk_sel` writer - "]
pub type REG_EMI_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMI_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_17_21` reader - "]
pub type RESERVED_17_21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_emi_clk_div` reader - "]
pub type REG_EMI_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_emi_clk_div` writer - "]
pub type REG_EMI_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMI_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_24_31` reader - "]
pub type RESERVED_24_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reserved_0_8(&self) -> RESERVED_0_8_R {
        RESERVED_0_8_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_emi_clk_en(&self) -> REG_EMI_CLK_EN_R {
        REG_EMI_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn reserved_10_13(&self) -> RESERVED_10_13_R {
        RESERVED_10_13_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn reg_emi_clk_sel(&self) -> REG_EMI_CLK_SEL_R {
        REG_EMI_CLK_SEL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn reserved_17_21(&self) -> RESERVED_17_21_R {
        RESERVED_17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reg_emi_clk_div(&self) -> REG_EMI_CLK_DIV_R {
        REG_EMI_CLK_DIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reserved_24_31(&self) -> RESERVED_24_31_R {
        RESERVED_24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emi_clk_en(&mut self) -> REG_EMI_CLK_EN_W<9> {
        REG_EMI_CLK_EN_W::new(self)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emi_clk_sel(&mut self) -> REG_EMI_CLK_SEL_W<14> {
        REG_EMI_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emi_clk_div(&mut self) -> REG_EMI_CLK_DIV_W<22> {
        REG_EMI_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "emi_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emi_cfg0](index.html) module"]
pub struct EMI_CFG0_SPEC;
impl crate::RegisterSpec for EMI_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emi_cfg0::R](R) reader structure"]
impl crate::Readable for EMI_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emi_cfg0::W](W) writer structure"]
impl crate::Writable for EMI_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emi_cfg0 to value 0x0200"]
impl crate::Resettable for EMI_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
