#[doc = "Register `i2c_cfg0` reader"]
pub struct R(crate::R<I2C_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_cfg0` writer"]
pub struct W(crate::W<I2C_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CFG0_SPEC>;
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
impl From<crate::W<I2C_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_15` reader - "]
pub type RESERVED_0_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `i2c_clk_div` reader - "]
pub type I2C_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2c_clk_div` writer - "]
pub type I2C_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_CFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `i2c_clk_en` reader - "]
pub type I2C_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2c_clk_en` writer - "]
pub type I2C_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CFG0_SPEC, bool, O>;
#[doc = "Field `i2c_clk_sel` reader - "]
pub type I2C_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `i2c_clk_sel` writer - "]
pub type I2C_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reserved_0_15(&self) -> RESERVED_0_15_R {
        RESERVED_0_15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c_clk_sel(&self) -> I2C_CLK_SEL_R {
        I2C_CLK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W<16> {
        I2C_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<24> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_sel(&mut self) -> I2C_CLK_SEL_W<25> {
        I2C_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cfg0](index.html) module"]
pub struct I2C_CFG0_SPEC;
impl crate::RegisterSpec for I2C_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_cfg0::R](R) reader structure"]
impl crate::Readable for I2C_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_cfg0::W](W) writer structure"]
impl crate::Writable for I2C_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_cfg0 to value 0x01ff_0000"]
impl crate::Resettable for I2C_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0000;
}