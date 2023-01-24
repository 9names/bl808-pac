#[doc = "Register `gpio_cfg0` reader"]
pub struct R(crate::R<GPIO_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg0` writer"]
pub struct W(crate::W<GPIO_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG0_SPEC>;
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
impl From<crate::W<GPIO_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_ie` reader - "]
pub type REG_GPIO_0_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_ie` writer - "]
pub type REG_GPIO_0_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_smt` reader - "]
pub type REG_GPIO_0_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_smt` writer - "]
pub type REG_GPIO_0_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_drv` reader - "]
pub type REG_GPIO_0_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_0_drv` writer - "]
pub type REG_GPIO_0_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_0_pu` reader - "]
pub type REG_GPIO_0_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_pu` writer - "]
pub type REG_GPIO_0_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_pd` reader - "]
pub type REG_GPIO_0_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_pd` writer - "]
pub type REG_GPIO_0_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_oe` reader - "]
pub type REG_GPIO_0_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_oe` writer - "]
pub type REG_GPIO_0_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_func_sel` reader - "]
pub type REG_GPIO_0_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_0_func_sel` writer - "]
pub type REG_GPIO_0_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_13_15` reader - "]
pub type RESERVED_13_15_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> REG_GPIO_0_IE_R {
        REG_GPIO_0_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> REG_GPIO_0_SMT_R {
        REG_GPIO_0_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> REG_GPIO_0_DRV_R {
        REG_GPIO_0_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> REG_GPIO_0_PU_R {
        REG_GPIO_0_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> REG_GPIO_0_PD_R {
        REG_GPIO_0_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&self) -> REG_GPIO_0_OE_R {
        REG_GPIO_0_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> REG_GPIO_0_FUNC_SEL_R {
        REG_GPIO_0_FUNC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_13_15(&self) -> RESERVED_13_15_R {
        RESERVED_13_15_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_ie(&mut self) -> REG_GPIO_0_IE_W<0> {
        REG_GPIO_0_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_smt(&mut self) -> REG_GPIO_0_SMT_W<1> {
        REG_GPIO_0_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_drv(&mut self) -> REG_GPIO_0_DRV_W<2> {
        REG_GPIO_0_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pu(&mut self) -> REG_GPIO_0_PU_W<4> {
        REG_GPIO_0_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pd(&mut self) -> REG_GPIO_0_PD_W<5> {
        REG_GPIO_0_PD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_oe(&mut self) -> REG_GPIO_0_OE_W<6> {
        REG_GPIO_0_OE_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_func_sel(&mut self) -> REG_GPIO_0_FUNC_SEL_W<8> {
        REG_GPIO_0_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg0](index.html) module"]
pub struct GPIO_CFG0_SPEC;
impl crate::RegisterSpec for GPIO_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg0::R](R) reader structure"]
impl crate::Readable for GPIO_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg0::W](W) writer structure"]
impl crate::Writable for GPIO_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg0 to value 0x0b02"]
impl crate::Resettable for GPIO_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b02;
}
