#[doc = "Register `gpio_cfg62` reader"]
pub struct R(crate::R<GPIO_CFG62_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG62_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG62_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG62_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg62` writer"]
pub struct W(crate::W<GPIO_CFG62_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG62_SPEC>;
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
impl From<crate::W<GPIO_CFG62_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG62_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_62_ie` reader - "]
pub type REG_GPIO_62_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_62_ie` writer - "]
pub type REG_GPIO_62_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG62_SPEC, bool, O>;
#[doc = "Field `reg_gpio_62_smt` reader - "]
pub type REG_GPIO_62_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_62_smt` writer - "]
pub type REG_GPIO_62_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG62_SPEC, bool, O>;
#[doc = "Field `reg_gpio_62_drv` reader - "]
pub type REG_GPIO_62_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_62_drv` writer - "]
pub type REG_GPIO_62_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG62_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_62_pu` reader - "]
pub type REG_GPIO_62_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_62_pu` writer - "]
pub type REG_GPIO_62_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG62_SPEC, bool, O>;
#[doc = "Field `reg_gpio_62_pd` reader - "]
pub type REG_GPIO_62_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_62_pd` writer - "]
pub type REG_GPIO_62_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG62_SPEC, bool, O>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_62_ie(&self) -> REG_GPIO_62_IE_R {
        REG_GPIO_62_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_62_smt(&self) -> REG_GPIO_62_SMT_R {
        REG_GPIO_62_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_62_drv(&self) -> REG_GPIO_62_DRV_R {
        REG_GPIO_62_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_62_pu(&self) -> REG_GPIO_62_PU_R {
        REG_GPIO_62_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_62_pd(&self) -> REG_GPIO_62_PD_R {
        REG_GPIO_62_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved_6_31(&self) -> RESERVED_6_31_R {
        RESERVED_6_31_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_62_ie(&mut self) -> REG_GPIO_62_IE_W<0> {
        REG_GPIO_62_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_62_smt(&mut self) -> REG_GPIO_62_SMT_W<1> {
        REG_GPIO_62_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_62_drv(&mut self) -> REG_GPIO_62_DRV_W<2> {
        REG_GPIO_62_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_62_pu(&mut self) -> REG_GPIO_62_PU_W<4> {
        REG_GPIO_62_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_62_pd(&mut self) -> REG_GPIO_62_PD_W<5> {
        REG_GPIO_62_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg62\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg62](index.html) module"]
pub struct GPIO_CFG62_SPEC;
impl crate::RegisterSpec for GPIO_CFG62_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg62::R](R) reader structure"]
impl crate::Readable for GPIO_CFG62_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg62::W](W) writer structure"]
impl crate::Writable for GPIO_CFG62_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg62 to value 0x02"]
impl crate::Resettable for GPIO_CFG62_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
