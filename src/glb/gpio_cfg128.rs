#[doc = "Register `gpio_cfg128` reader"]
pub struct R(crate::R<GPIO_CFG128_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG128_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG128_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG128_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg128` writer"]
pub struct W(crate::W<GPIO_CFG128_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG128_SPEC>;
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
impl From<crate::W<GPIO_CFG128_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG128_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_0_i` reader - "]
pub type REG2_GPIO_0_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_1_i` reader - "]
pub type REG2_GPIO_1_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_2_i` reader - "]
pub type REG2_GPIO_2_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_3_i` reader - "]
pub type REG2_GPIO_3_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_4_i` reader - "]
pub type REG2_GPIO_4_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_5_i` reader - "]
pub type REG2_GPIO_5_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_6_i` reader - "]
pub type REG2_GPIO_6_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_7_i` reader - "]
pub type REG2_GPIO_7_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_8_i` reader - "]
pub type REG2_GPIO_8_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_9_i` reader - "]
pub type REG2_GPIO_9_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_10_i` reader - "]
pub type REG2_GPIO_10_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_11_i` reader - "]
pub type REG2_GPIO_11_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_12_i` reader - "]
pub type REG2_GPIO_12_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_13_i` reader - "]
pub type REG2_GPIO_13_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_14_i` reader - "]
pub type REG2_GPIO_14_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_15_i` reader - "]
pub type REG2_GPIO_15_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_16_i` reader - "]
pub type REG2_GPIO_16_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_17_i` reader - "]
pub type REG2_GPIO_17_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_18_i` reader - "]
pub type REG2_GPIO_18_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_19_i` reader - "]
pub type REG2_GPIO_19_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_20_i` reader - "]
pub type REG2_GPIO_20_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_21_i` reader - "]
pub type REG2_GPIO_21_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_22_i` reader - "]
pub type REG2_GPIO_22_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_23_i` reader - "]
pub type REG2_GPIO_23_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_24_i` reader - "]
pub type REG2_GPIO_24_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_25_i` reader - "]
pub type REG2_GPIO_25_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_26_i` reader - "]
pub type REG2_GPIO_26_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_27_i` reader - "]
pub type REG2_GPIO_27_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_28_i` reader - "]
pub type REG2_GPIO_28_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_29_i` reader - "]
pub type REG2_GPIO_29_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_30_i` reader - "]
pub type REG2_GPIO_30_I_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_31_i` reader - "]
pub type REG2_GPIO_31_I_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg2_gpio_0_i(&self) -> REG2_GPIO_0_I_R {
        REG2_GPIO_0_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg2_gpio_1_i(&self) -> REG2_GPIO_1_I_R {
        REG2_GPIO_1_I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg2_gpio_2_i(&self) -> REG2_GPIO_2_I_R {
        REG2_GPIO_2_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg2_gpio_3_i(&self) -> REG2_GPIO_3_I_R {
        REG2_GPIO_3_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg2_gpio_4_i(&self) -> REG2_GPIO_4_I_R {
        REG2_GPIO_4_I_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg2_gpio_5_i(&self) -> REG2_GPIO_5_I_R {
        REG2_GPIO_5_I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg2_gpio_6_i(&self) -> REG2_GPIO_6_I_R {
        REG2_GPIO_6_I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg2_gpio_7_i(&self) -> REG2_GPIO_7_I_R {
        REG2_GPIO_7_I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg2_gpio_8_i(&self) -> REG2_GPIO_8_I_R {
        REG2_GPIO_8_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg2_gpio_9_i(&self) -> REG2_GPIO_9_I_R {
        REG2_GPIO_9_I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg2_gpio_10_i(&self) -> REG2_GPIO_10_I_R {
        REG2_GPIO_10_I_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg2_gpio_11_i(&self) -> REG2_GPIO_11_I_R {
        REG2_GPIO_11_I_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg2_gpio_12_i(&self) -> REG2_GPIO_12_I_R {
        REG2_GPIO_12_I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg2_gpio_13_i(&self) -> REG2_GPIO_13_I_R {
        REG2_GPIO_13_I_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg2_gpio_14_i(&self) -> REG2_GPIO_14_I_R {
        REG2_GPIO_14_I_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg2_gpio_15_i(&self) -> REG2_GPIO_15_I_R {
        REG2_GPIO_15_I_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg2_gpio_16_i(&self) -> REG2_GPIO_16_I_R {
        REG2_GPIO_16_I_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg2_gpio_17_i(&self) -> REG2_GPIO_17_I_R {
        REG2_GPIO_17_I_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg2_gpio_18_i(&self) -> REG2_GPIO_18_I_R {
        REG2_GPIO_18_I_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg2_gpio_19_i(&self) -> REG2_GPIO_19_I_R {
        REG2_GPIO_19_I_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg2_gpio_20_i(&self) -> REG2_GPIO_20_I_R {
        REG2_GPIO_20_I_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg2_gpio_21_i(&self) -> REG2_GPIO_21_I_R {
        REG2_GPIO_21_I_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg2_gpio_22_i(&self) -> REG2_GPIO_22_I_R {
        REG2_GPIO_22_I_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg2_gpio_23_i(&self) -> REG2_GPIO_23_I_R {
        REG2_GPIO_23_I_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg2_gpio_24_i(&self) -> REG2_GPIO_24_I_R {
        REG2_GPIO_24_I_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg2_gpio_25_i(&self) -> REG2_GPIO_25_I_R {
        REG2_GPIO_25_I_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg2_gpio_26_i(&self) -> REG2_GPIO_26_I_R {
        REG2_GPIO_26_I_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg2_gpio_27_i(&self) -> REG2_GPIO_27_I_R {
        REG2_GPIO_27_I_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg2_gpio_28_i(&self) -> REG2_GPIO_28_I_R {
        REG2_GPIO_28_I_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg2_gpio_29_i(&self) -> REG2_GPIO_29_I_R {
        REG2_GPIO_29_I_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg2_gpio_30_i(&self) -> REG2_GPIO_30_I_R {
        REG2_GPIO_30_I_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg2_gpio_31_i(&self) -> REG2_GPIO_31_I_R {
        REG2_GPIO_31_I_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "gpio_cfg128\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg128](index.html) module"]
pub struct GPIO_CFG128_SPEC;
impl crate::RegisterSpec for GPIO_CFG128_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg128::R](R) reader structure"]
impl crate::Readable for GPIO_CFG128_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg128::W](W) writer structure"]
impl crate::Writable for GPIO_CFG128_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg128 to value 0"]
impl crate::Resettable for GPIO_CFG128_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
