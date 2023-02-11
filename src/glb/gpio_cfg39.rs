#[doc = "Register `gpio_cfg39` reader"]
pub struct R(crate::R<GPIO_CFG39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg39` writer"]
pub struct W(crate::W<GPIO_CFG39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG39_SPEC>;
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
impl From<crate::W<GPIO_CFG39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_39_ie` reader - "]
pub type REG_GPIO_39_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_ie` writer - "]
pub type REG_GPIO_39_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_smt` reader - "]
pub type REG_GPIO_39_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_smt` writer - "]
pub type REG_GPIO_39_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_drv` reader - "]
pub type REG_GPIO_39_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_39_drv` writer - "]
pub type REG_GPIO_39_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG39_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_39_pu` reader - "]
pub type REG_GPIO_39_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_pu` writer - "]
pub type REG_GPIO_39_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_pd` reader - "]
pub type REG_GPIO_39_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_pd` writer - "]
pub type REG_GPIO_39_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_oe` reader - "]
pub type REG_GPIO_39_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_oe` writer - "]
pub type REG_GPIO_39_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_func_sel` reader - "]
pub type REG_GPIO_39_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_39_func_sel` writer - "]
pub type REG_GPIO_39_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG39_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_13_15` reader - "]
pub type RESERVED_13_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_39_int_mode_set` reader - "]
pub type REG_GPIO_39_INT_MODE_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_39_int_mode_set` writer - "]
pub type REG_GPIO_39_INT_MODE_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG39_SPEC, u8, u8, 4, O>;
#[doc = "Field `reg_gpio_39_int_clr` reader - "]
pub type REG_GPIO_39_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_int_clr` writer - "]
pub type REG_GPIO_39_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `gpio_39_int_stat` reader - "]
pub type GPIO_39_INT_STAT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_int_mask` reader - "]
pub type REG_GPIO_39_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_int_mask` writer - "]
pub type REG_GPIO_39_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_o` reader - "]
pub type REG_GPIO_39_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_o` writer - "]
pub type REG_GPIO_39_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_set` writer - "]
pub type REG_GPIO_39_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reg_gpio_39_clr` writer - "]
pub type REG_GPIO_39_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG39_SPEC, bool, O>;
#[doc = "Field `reserved_27` reader - "]
pub type RESERVED_27_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_i` reader - "]
pub type REG_GPIO_39_I_R = crate::BitReader<bool>;
#[doc = "Field `reserved_29` reader - "]
pub type RESERVED_29_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_39_mode` reader - "]
pub type REG_GPIO_39_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_39_mode` writer - "]
pub type REG_GPIO_39_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG39_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_39_ie(&self) -> REG_GPIO_39_IE_R {
        REG_GPIO_39_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_39_smt(&self) -> REG_GPIO_39_SMT_R {
        REG_GPIO_39_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_39_drv(&self) -> REG_GPIO_39_DRV_R {
        REG_GPIO_39_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_39_pu(&self) -> REG_GPIO_39_PU_R {
        REG_GPIO_39_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_39_pd(&self) -> REG_GPIO_39_PD_R {
        REG_GPIO_39_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_39_oe(&self) -> REG_GPIO_39_OE_R {
        REG_GPIO_39_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_39_func_sel(&self) -> REG_GPIO_39_FUNC_SEL_R {
        REG_GPIO_39_FUNC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_13_15(&self) -> RESERVED_13_15_R {
        RESERVED_13_15_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_gpio_39_int_mode_set(&self) -> REG_GPIO_39_INT_MODE_SET_R {
        REG_GPIO_39_INT_MODE_SET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_39_int_clr(&self) -> REG_GPIO_39_INT_CLR_R {
        REG_GPIO_39_INT_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_39_int_stat(&self) -> GPIO_39_INT_STAT_R {
        GPIO_39_INT_STAT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_39_int_mask(&self) -> REG_GPIO_39_INT_MASK_R {
        REG_GPIO_39_INT_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_gpio_39_o(&self) -> REG_GPIO_39_O_R {
        REG_GPIO_39_O_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved_27(&self) -> RESERVED_27_R {
        RESERVED_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_gpio_39_i(&self) -> REG_GPIO_39_I_R {
        REG_GPIO_39_I_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reserved_29(&self) -> RESERVED_29_R {
        RESERVED_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_gpio_39_mode(&self) -> REG_GPIO_39_MODE_R {
        REG_GPIO_39_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_ie(&mut self) -> REG_GPIO_39_IE_W<0> {
        REG_GPIO_39_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_smt(&mut self) -> REG_GPIO_39_SMT_W<1> {
        REG_GPIO_39_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_drv(&mut self) -> REG_GPIO_39_DRV_W<2> {
        REG_GPIO_39_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_pu(&mut self) -> REG_GPIO_39_PU_W<4> {
        REG_GPIO_39_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_pd(&mut self) -> REG_GPIO_39_PD_W<5> {
        REG_GPIO_39_PD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_oe(&mut self) -> REG_GPIO_39_OE_W<6> {
        REG_GPIO_39_OE_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_func_sel(&mut self) -> REG_GPIO_39_FUNC_SEL_W<8> {
        REG_GPIO_39_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_int_mode_set(&mut self) -> REG_GPIO_39_INT_MODE_SET_W<16> {
        REG_GPIO_39_INT_MODE_SET_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_int_clr(&mut self) -> REG_GPIO_39_INT_CLR_W<20> {
        REG_GPIO_39_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_int_mask(&mut self) -> REG_GPIO_39_INT_MASK_W<22> {
        REG_GPIO_39_INT_MASK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_o(&mut self) -> REG_GPIO_39_O_W<24> {
        REG_GPIO_39_O_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_set(&mut self) -> REG_GPIO_39_SET_W<25> {
        REG_GPIO_39_SET_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_clr(&mut self) -> REG_GPIO_39_CLR_W<26> {
        REG_GPIO_39_CLR_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_39_mode(&mut self) -> REG_GPIO_39_MODE_W<30> {
        REG_GPIO_39_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg39\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg39](index.html) module"]
pub struct GPIO_CFG39_SPEC;
impl crate::RegisterSpec for GPIO_CFG39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg39::R](R) reader structure"]
impl crate::Readable for GPIO_CFG39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg39::W](W) writer structure"]
impl crate::Writable for GPIO_CFG39_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg39 to value 0x0040_0b02"]
impl crate::Resettable for GPIO_CFG39_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0b02;
}
