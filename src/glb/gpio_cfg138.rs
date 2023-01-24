#[doc = "Register `gpio_cfg138` reader"]
pub struct R(crate::R<GPIO_CFG138_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG138_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG138_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG138_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg138` writer"]
pub struct W(crate::W<GPIO_CFG138_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG138_SPEC>;
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
impl From<crate::W<GPIO_CFG138_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG138_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_0_set` writer - "]
pub type REG2_GPIO_0_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_1_set` writer - "]
pub type REG2_GPIO_1_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_2_set` writer - "]
pub type REG2_GPIO_2_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_3_set` writer - "]
pub type REG2_GPIO_3_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_4_set` writer - "]
pub type REG2_GPIO_4_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_5_set` writer - "]
pub type REG2_GPIO_5_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_6_set` writer - "]
pub type REG2_GPIO_6_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_7_set` writer - "]
pub type REG2_GPIO_7_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_8_set` writer - "]
pub type REG2_GPIO_8_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_9_set` writer - "]
pub type REG2_GPIO_9_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_10_set` writer - "]
pub type REG2_GPIO_10_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_11_set` writer - "]
pub type REG2_GPIO_11_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_12_set` writer - "]
pub type REG2_GPIO_12_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_13_set` writer - "]
pub type REG2_GPIO_13_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_14_set` writer - "]
pub type REG2_GPIO_14_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_15_set` writer - "]
pub type REG2_GPIO_15_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_16_set` writer - "]
pub type REG2_GPIO_16_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_17_set` writer - "]
pub type REG2_GPIO_17_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_18_set` writer - "]
pub type REG2_GPIO_18_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_19_set` writer - "]
pub type REG2_GPIO_19_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_20_set` writer - "]
pub type REG2_GPIO_20_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_21_set` writer - "]
pub type REG2_GPIO_21_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_22_set` writer - "]
pub type REG2_GPIO_22_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_23_set` writer - "]
pub type REG2_GPIO_23_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_24_set` writer - "]
pub type REG2_GPIO_24_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_25_set` writer - "]
pub type REG2_GPIO_25_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_26_set` writer - "]
pub type REG2_GPIO_26_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_27_set` writer - "]
pub type REG2_GPIO_27_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_28_set` writer - "]
pub type REG2_GPIO_28_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_29_set` writer - "]
pub type REG2_GPIO_29_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_30_set` writer - "]
pub type REG2_GPIO_30_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_31_set` writer - "]
pub type REG2_GPIO_31_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG138_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_0_set(&mut self) -> REG2_GPIO_0_SET_W<0> {
        REG2_GPIO_0_SET_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_1_set(&mut self) -> REG2_GPIO_1_SET_W<1> {
        REG2_GPIO_1_SET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_2_set(&mut self) -> REG2_GPIO_2_SET_W<2> {
        REG2_GPIO_2_SET_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_3_set(&mut self) -> REG2_GPIO_3_SET_W<3> {
        REG2_GPIO_3_SET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_4_set(&mut self) -> REG2_GPIO_4_SET_W<4> {
        REG2_GPIO_4_SET_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_5_set(&mut self) -> REG2_GPIO_5_SET_W<5> {
        REG2_GPIO_5_SET_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_6_set(&mut self) -> REG2_GPIO_6_SET_W<6> {
        REG2_GPIO_6_SET_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_7_set(&mut self) -> REG2_GPIO_7_SET_W<7> {
        REG2_GPIO_7_SET_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_8_set(&mut self) -> REG2_GPIO_8_SET_W<8> {
        REG2_GPIO_8_SET_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_9_set(&mut self) -> REG2_GPIO_9_SET_W<9> {
        REG2_GPIO_9_SET_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_10_set(&mut self) -> REG2_GPIO_10_SET_W<10> {
        REG2_GPIO_10_SET_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_11_set(&mut self) -> REG2_GPIO_11_SET_W<11> {
        REG2_GPIO_11_SET_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_12_set(&mut self) -> REG2_GPIO_12_SET_W<12> {
        REG2_GPIO_12_SET_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_13_set(&mut self) -> REG2_GPIO_13_SET_W<13> {
        REG2_GPIO_13_SET_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_14_set(&mut self) -> REG2_GPIO_14_SET_W<14> {
        REG2_GPIO_14_SET_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_15_set(&mut self) -> REG2_GPIO_15_SET_W<15> {
        REG2_GPIO_15_SET_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_16_set(&mut self) -> REG2_GPIO_16_SET_W<16> {
        REG2_GPIO_16_SET_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_17_set(&mut self) -> REG2_GPIO_17_SET_W<17> {
        REG2_GPIO_17_SET_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_18_set(&mut self) -> REG2_GPIO_18_SET_W<18> {
        REG2_GPIO_18_SET_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_19_set(&mut self) -> REG2_GPIO_19_SET_W<19> {
        REG2_GPIO_19_SET_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_20_set(&mut self) -> REG2_GPIO_20_SET_W<20> {
        REG2_GPIO_20_SET_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_21_set(&mut self) -> REG2_GPIO_21_SET_W<21> {
        REG2_GPIO_21_SET_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_22_set(&mut self) -> REG2_GPIO_22_SET_W<22> {
        REG2_GPIO_22_SET_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_23_set(&mut self) -> REG2_GPIO_23_SET_W<23> {
        REG2_GPIO_23_SET_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_24_set(&mut self) -> REG2_GPIO_24_SET_W<24> {
        REG2_GPIO_24_SET_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_25_set(&mut self) -> REG2_GPIO_25_SET_W<25> {
        REG2_GPIO_25_SET_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_26_set(&mut self) -> REG2_GPIO_26_SET_W<26> {
        REG2_GPIO_26_SET_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_27_set(&mut self) -> REG2_GPIO_27_SET_W<27> {
        REG2_GPIO_27_SET_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_28_set(&mut self) -> REG2_GPIO_28_SET_W<28> {
        REG2_GPIO_28_SET_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_29_set(&mut self) -> REG2_GPIO_29_SET_W<29> {
        REG2_GPIO_29_SET_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_30_set(&mut self) -> REG2_GPIO_30_SET_W<30> {
        REG2_GPIO_30_SET_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_31_set(&mut self) -> REG2_GPIO_31_SET_W<31> {
        REG2_GPIO_31_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg138\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg138](index.html) module"]
pub struct GPIO_CFG138_SPEC;
impl crate::RegisterSpec for GPIO_CFG138_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg138::R](R) reader structure"]
impl crate::Readable for GPIO_CFG138_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg138::W](W) writer structure"]
impl crate::Writable for GPIO_CFG138_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg138 to value 0"]
impl crate::Resettable for GPIO_CFG138_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
