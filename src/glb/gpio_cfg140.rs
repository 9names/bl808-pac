#[doc = "Register `gpio_cfg140` reader"]
pub struct R(crate::R<GPIO_CFG140_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG140_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG140_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG140_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg140` writer"]
pub struct W(crate::W<GPIO_CFG140_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG140_SPEC>;
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
impl From<crate::W<GPIO_CFG140_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG140_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_0_clr` writer - "]
pub type REG2_GPIO_0_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_1_clr` writer - "]
pub type REG2_GPIO_1_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_2_clr` writer - "]
pub type REG2_GPIO_2_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_3_clr` writer - "]
pub type REG2_GPIO_3_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_4_clr` writer - "]
pub type REG2_GPIO_4_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_5_clr` writer - "]
pub type REG2_GPIO_5_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_6_clr` writer - "]
pub type REG2_GPIO_6_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_7_clr` writer - "]
pub type REG2_GPIO_7_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_8_clr` writer - "]
pub type REG2_GPIO_8_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_9_clr` writer - "]
pub type REG2_GPIO_9_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_10_clr` writer - "]
pub type REG2_GPIO_10_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_11_clr` writer - "]
pub type REG2_GPIO_11_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_12_clr` writer - "]
pub type REG2_GPIO_12_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_13_clr` writer - "]
pub type REG2_GPIO_13_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_14_clr` writer - "]
pub type REG2_GPIO_14_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_15_clr` writer - "]
pub type REG2_GPIO_15_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_16_clr` writer - "]
pub type REG2_GPIO_16_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_17_clr` writer - "]
pub type REG2_GPIO_17_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_18_clr` writer - "]
pub type REG2_GPIO_18_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_19_clr` writer - "]
pub type REG2_GPIO_19_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_20_clr` writer - "]
pub type REG2_GPIO_20_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_21_clr` writer - "]
pub type REG2_GPIO_21_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_22_clr` writer - "]
pub type REG2_GPIO_22_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_23_clr` writer - "]
pub type REG2_GPIO_23_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_24_clr` writer - "]
pub type REG2_GPIO_24_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_25_clr` writer - "]
pub type REG2_GPIO_25_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_26_clr` writer - "]
pub type REG2_GPIO_26_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_27_clr` writer - "]
pub type REG2_GPIO_27_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_28_clr` writer - "]
pub type REG2_GPIO_28_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_29_clr` writer - "]
pub type REG2_GPIO_29_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_30_clr` writer - "]
pub type REG2_GPIO_30_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_31_clr` writer - "]
pub type REG2_GPIO_31_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG140_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_0_clr(&mut self) -> REG2_GPIO_0_CLR_W<0> {
        REG2_GPIO_0_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_1_clr(&mut self) -> REG2_GPIO_1_CLR_W<1> {
        REG2_GPIO_1_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_2_clr(&mut self) -> REG2_GPIO_2_CLR_W<2> {
        REG2_GPIO_2_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_3_clr(&mut self) -> REG2_GPIO_3_CLR_W<3> {
        REG2_GPIO_3_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_4_clr(&mut self) -> REG2_GPIO_4_CLR_W<4> {
        REG2_GPIO_4_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_5_clr(&mut self) -> REG2_GPIO_5_CLR_W<5> {
        REG2_GPIO_5_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_6_clr(&mut self) -> REG2_GPIO_6_CLR_W<6> {
        REG2_GPIO_6_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_7_clr(&mut self) -> REG2_GPIO_7_CLR_W<7> {
        REG2_GPIO_7_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_8_clr(&mut self) -> REG2_GPIO_8_CLR_W<8> {
        REG2_GPIO_8_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_9_clr(&mut self) -> REG2_GPIO_9_CLR_W<9> {
        REG2_GPIO_9_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_10_clr(&mut self) -> REG2_GPIO_10_CLR_W<10> {
        REG2_GPIO_10_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_11_clr(&mut self) -> REG2_GPIO_11_CLR_W<11> {
        REG2_GPIO_11_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_12_clr(&mut self) -> REG2_GPIO_12_CLR_W<12> {
        REG2_GPIO_12_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_13_clr(&mut self) -> REG2_GPIO_13_CLR_W<13> {
        REG2_GPIO_13_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_14_clr(&mut self) -> REG2_GPIO_14_CLR_W<14> {
        REG2_GPIO_14_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_15_clr(&mut self) -> REG2_GPIO_15_CLR_W<15> {
        REG2_GPIO_15_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_16_clr(&mut self) -> REG2_GPIO_16_CLR_W<16> {
        REG2_GPIO_16_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_17_clr(&mut self) -> REG2_GPIO_17_CLR_W<17> {
        REG2_GPIO_17_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_18_clr(&mut self) -> REG2_GPIO_18_CLR_W<18> {
        REG2_GPIO_18_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_19_clr(&mut self) -> REG2_GPIO_19_CLR_W<19> {
        REG2_GPIO_19_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_20_clr(&mut self) -> REG2_GPIO_20_CLR_W<20> {
        REG2_GPIO_20_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_21_clr(&mut self) -> REG2_GPIO_21_CLR_W<21> {
        REG2_GPIO_21_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_22_clr(&mut self) -> REG2_GPIO_22_CLR_W<22> {
        REG2_GPIO_22_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_23_clr(&mut self) -> REG2_GPIO_23_CLR_W<23> {
        REG2_GPIO_23_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_24_clr(&mut self) -> REG2_GPIO_24_CLR_W<24> {
        REG2_GPIO_24_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_25_clr(&mut self) -> REG2_GPIO_25_CLR_W<25> {
        REG2_GPIO_25_CLR_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_26_clr(&mut self) -> REG2_GPIO_26_CLR_W<26> {
        REG2_GPIO_26_CLR_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_27_clr(&mut self) -> REG2_GPIO_27_CLR_W<27> {
        REG2_GPIO_27_CLR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_28_clr(&mut self) -> REG2_GPIO_28_CLR_W<28> {
        REG2_GPIO_28_CLR_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_29_clr(&mut self) -> REG2_GPIO_29_CLR_W<29> {
        REG2_GPIO_29_CLR_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_30_clr(&mut self) -> REG2_GPIO_30_CLR_W<30> {
        REG2_GPIO_30_CLR_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_31_clr(&mut self) -> REG2_GPIO_31_CLR_W<31> {
        REG2_GPIO_31_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg140\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg140](index.html) module"]
pub struct GPIO_CFG140_SPEC;
impl crate::RegisterSpec for GPIO_CFG140_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg140::R](R) reader structure"]
impl crate::Readable for GPIO_CFG140_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg140::W](W) writer structure"]
impl crate::Writable for GPIO_CFG140_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg140 to value 0"]
impl crate::Resettable for GPIO_CFG140_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
