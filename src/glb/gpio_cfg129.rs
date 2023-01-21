#[doc = "Register `gpio_cfg129` reader"]
pub struct R(crate::R<GPIO_CFG129_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG129_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG129_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG129_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg129` writer"]
pub struct W(crate::W<GPIO_CFG129_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG129_SPEC>;
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
impl From<crate::W<GPIO_CFG129_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG129_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_32_i` reader - "]
pub type REG2_GPIO_32_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_33_i` reader - "]
pub type REG2_GPIO_33_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_34_i` reader - "]
pub type REG2_GPIO_34_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_35_i` reader - "]
pub type REG2_GPIO_35_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_36_i` reader - "]
pub type REG2_GPIO_36_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_37_i` reader - "]
pub type REG2_GPIO_37_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_38_i` reader - "]
pub type REG2_GPIO_38_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_39_i` reader - "]
pub type REG2_GPIO_39_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_40_i` reader - "]
pub type REG2_GPIO_40_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_41_i` reader - "]
pub type REG2_GPIO_41_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_42_i` reader - "]
pub type REG2_GPIO_42_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_43_i` reader - "]
pub type REG2_GPIO_43_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_44_i` reader - "]
pub type REG2_GPIO_44_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_45_i` reader - "]
pub type REG2_GPIO_45_I_R = crate::BitReader<bool>;
#[doc = "Field `reserved_14_31` reader - "]
pub type RESERVED_14_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg2_gpio_32_i(&self) -> REG2_GPIO_32_I_R {
        REG2_GPIO_32_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg2_gpio_33_i(&self) -> REG2_GPIO_33_I_R {
        REG2_GPIO_33_I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg2_gpio_34_i(&self) -> REG2_GPIO_34_I_R {
        REG2_GPIO_34_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg2_gpio_35_i(&self) -> REG2_GPIO_35_I_R {
        REG2_GPIO_35_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg2_gpio_36_i(&self) -> REG2_GPIO_36_I_R {
        REG2_GPIO_36_I_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg2_gpio_37_i(&self) -> REG2_GPIO_37_I_R {
        REG2_GPIO_37_I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg2_gpio_38_i(&self) -> REG2_GPIO_38_I_R {
        REG2_GPIO_38_I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg2_gpio_39_i(&self) -> REG2_GPIO_39_I_R {
        REG2_GPIO_39_I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg2_gpio_40_i(&self) -> REG2_GPIO_40_I_R {
        REG2_GPIO_40_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg2_gpio_41_i(&self) -> REG2_GPIO_41_I_R {
        REG2_GPIO_41_I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg2_gpio_42_i(&self) -> REG2_GPIO_42_I_R {
        REG2_GPIO_42_I_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg2_gpio_43_i(&self) -> REG2_GPIO_43_I_R {
        REG2_GPIO_43_I_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg2_gpio_44_i(&self) -> REG2_GPIO_44_I_R {
        REG2_GPIO_44_I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg2_gpio_45_i(&self) -> REG2_GPIO_45_I_R {
        REG2_GPIO_45_I_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn reserved_14_31(&self) -> RESERVED_14_31_R {
        RESERVED_14_31_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg129\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg129](index.html) module"]
pub struct GPIO_CFG129_SPEC;
impl crate::RegisterSpec for GPIO_CFG129_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg129::R](R) reader structure"]
impl crate::Readable for GPIO_CFG129_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg129::W](W) writer structure"]
impl crate::Writable for GPIO_CFG129_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg129 to value 0"]
impl crate::Resettable for GPIO_CFG129_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
