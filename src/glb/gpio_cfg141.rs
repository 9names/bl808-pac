#[doc = "Register `gpio_cfg141` reader"]
pub struct R(crate::R<GPIO_CFG141_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG141_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG141_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG141_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg141` writer"]
pub struct W(crate::W<GPIO_CFG141_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG141_SPEC>;
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
impl From<crate::W<GPIO_CFG141_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG141_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_32_clr` writer - "]
pub type REG2_GPIO_32_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_33_clr` writer - "]
pub type REG2_GPIO_33_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_34_clr` writer - "]
pub type REG2_GPIO_34_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_35_clr` writer - "]
pub type REG2_GPIO_35_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_36_clr` writer - "]
pub type REG2_GPIO_36_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_37_clr` writer - "]
pub type REG2_GPIO_37_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_38_clr` writer - "]
pub type REG2_GPIO_38_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_39_clr` writer - "]
pub type REG2_GPIO_39_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_40_clr` writer - "]
pub type REG2_GPIO_40_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_41_clr` writer - "]
pub type REG2_GPIO_41_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_42_clr` writer - "]
pub type REG2_GPIO_42_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_43_clr` writer - "]
pub type REG2_GPIO_43_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_44_clr` writer - "]
pub type REG2_GPIO_44_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_45_clr` writer - "]
pub type REG2_GPIO_45_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG141_SPEC, bool, O>;
#[doc = "Field `reserved_14_31` reader - "]
pub type RESERVED_14_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn reserved_14_31(&self) -> RESERVED_14_31_R {
        RESERVED_14_31_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_32_clr(&mut self) -> REG2_GPIO_32_CLR_W<0> {
        REG2_GPIO_32_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_33_clr(&mut self) -> REG2_GPIO_33_CLR_W<1> {
        REG2_GPIO_33_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_34_clr(&mut self) -> REG2_GPIO_34_CLR_W<2> {
        REG2_GPIO_34_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_35_clr(&mut self) -> REG2_GPIO_35_CLR_W<3> {
        REG2_GPIO_35_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_36_clr(&mut self) -> REG2_GPIO_36_CLR_W<4> {
        REG2_GPIO_36_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_37_clr(&mut self) -> REG2_GPIO_37_CLR_W<5> {
        REG2_GPIO_37_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_38_clr(&mut self) -> REG2_GPIO_38_CLR_W<6> {
        REG2_GPIO_38_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_39_clr(&mut self) -> REG2_GPIO_39_CLR_W<7> {
        REG2_GPIO_39_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_40_clr(&mut self) -> REG2_GPIO_40_CLR_W<8> {
        REG2_GPIO_40_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_41_clr(&mut self) -> REG2_GPIO_41_CLR_W<9> {
        REG2_GPIO_41_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_42_clr(&mut self) -> REG2_GPIO_42_CLR_W<10> {
        REG2_GPIO_42_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_43_clr(&mut self) -> REG2_GPIO_43_CLR_W<11> {
        REG2_GPIO_43_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_44_clr(&mut self) -> REG2_GPIO_44_CLR_W<12> {
        REG2_GPIO_44_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_45_clr(&mut self) -> REG2_GPIO_45_CLR_W<13> {
        REG2_GPIO_45_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg141\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg141](index.html) module"]
pub struct GPIO_CFG141_SPEC;
impl crate::RegisterSpec for GPIO_CFG141_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg141::R](R) reader structure"]
impl crate::Readable for GPIO_CFG141_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg141::W](W) writer structure"]
impl crate::Writable for GPIO_CFG141_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg141 to value 0"]
impl crate::Resettable for GPIO_CFG141_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
