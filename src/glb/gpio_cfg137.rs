#[doc = "Register `gpio_cfg137` reader"]
pub struct R(crate::R<GPIO_CFG137_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG137_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG137_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG137_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg137` writer"]
pub struct W(crate::W<GPIO_CFG137_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG137_SPEC>;
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
impl From<crate::W<GPIO_CFG137_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG137_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg2_gpio_32_o` reader - "]
pub type REG2_GPIO_32_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_32_o` writer - "]
pub type REG2_GPIO_32_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_33_o` reader - "]
pub type REG2_GPIO_33_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_33_o` writer - "]
pub type REG2_GPIO_33_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_34_o` reader - "]
pub type REG2_GPIO_34_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_34_o` writer - "]
pub type REG2_GPIO_34_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_35_o` reader - "]
pub type REG2_GPIO_35_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_35_o` writer - "]
pub type REG2_GPIO_35_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_36_o` reader - "]
pub type REG2_GPIO_36_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_36_o` writer - "]
pub type REG2_GPIO_36_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_37_o` reader - "]
pub type REG2_GPIO_37_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_37_o` writer - "]
pub type REG2_GPIO_37_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_38_o` reader - "]
pub type REG2_GPIO_38_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_38_o` writer - "]
pub type REG2_GPIO_38_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_39_o` reader - "]
pub type REG2_GPIO_39_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_39_o` writer - "]
pub type REG2_GPIO_39_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_40_o` reader - "]
pub type REG2_GPIO_40_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_40_o` writer - "]
pub type REG2_GPIO_40_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_41_o` reader - "]
pub type REG2_GPIO_41_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_41_o` writer - "]
pub type REG2_GPIO_41_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_42_o` reader - "]
pub type REG2_GPIO_42_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_42_o` writer - "]
pub type REG2_GPIO_42_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_43_o` reader - "]
pub type REG2_GPIO_43_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_43_o` writer - "]
pub type REG2_GPIO_43_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_44_o` reader - "]
pub type REG2_GPIO_44_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_44_o` writer - "]
pub type REG2_GPIO_44_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reg2_gpio_45_o` reader - "]
pub type REG2_GPIO_45_O_R = crate::BitReader<bool>;
#[doc = "Field `reg2_gpio_45_o` writer - "]
pub type REG2_GPIO_45_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG137_SPEC, bool, O>;
#[doc = "Field `reserved_14_31` reader - "]
pub type RESERVED_14_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg2_gpio_32_o(&self) -> REG2_GPIO_32_O_R {
        REG2_GPIO_32_O_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg2_gpio_33_o(&self) -> REG2_GPIO_33_O_R {
        REG2_GPIO_33_O_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg2_gpio_34_o(&self) -> REG2_GPIO_34_O_R {
        REG2_GPIO_34_O_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg2_gpio_35_o(&self) -> REG2_GPIO_35_O_R {
        REG2_GPIO_35_O_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg2_gpio_36_o(&self) -> REG2_GPIO_36_O_R {
        REG2_GPIO_36_O_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg2_gpio_37_o(&self) -> REG2_GPIO_37_O_R {
        REG2_GPIO_37_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg2_gpio_38_o(&self) -> REG2_GPIO_38_O_R {
        REG2_GPIO_38_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg2_gpio_39_o(&self) -> REG2_GPIO_39_O_R {
        REG2_GPIO_39_O_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg2_gpio_40_o(&self) -> REG2_GPIO_40_O_R {
        REG2_GPIO_40_O_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg2_gpio_41_o(&self) -> REG2_GPIO_41_O_R {
        REG2_GPIO_41_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg2_gpio_42_o(&self) -> REG2_GPIO_42_O_R {
        REG2_GPIO_42_O_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg2_gpio_43_o(&self) -> REG2_GPIO_43_O_R {
        REG2_GPIO_43_O_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg2_gpio_44_o(&self) -> REG2_GPIO_44_O_R {
        REG2_GPIO_44_O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg2_gpio_45_o(&self) -> REG2_GPIO_45_O_R {
        REG2_GPIO_45_O_R::new(((self.bits >> 13) & 1) != 0)
    }
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
    pub fn reg2_gpio_32_o(&mut self) -> REG2_GPIO_32_O_W<0> {
        REG2_GPIO_32_O_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_33_o(&mut self) -> REG2_GPIO_33_O_W<1> {
        REG2_GPIO_33_O_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_34_o(&mut self) -> REG2_GPIO_34_O_W<2> {
        REG2_GPIO_34_O_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_35_o(&mut self) -> REG2_GPIO_35_O_W<3> {
        REG2_GPIO_35_O_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_36_o(&mut self) -> REG2_GPIO_36_O_W<4> {
        REG2_GPIO_36_O_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_37_o(&mut self) -> REG2_GPIO_37_O_W<5> {
        REG2_GPIO_37_O_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_38_o(&mut self) -> REG2_GPIO_38_O_W<6> {
        REG2_GPIO_38_O_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_39_o(&mut self) -> REG2_GPIO_39_O_W<7> {
        REG2_GPIO_39_O_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_40_o(&mut self) -> REG2_GPIO_40_O_W<8> {
        REG2_GPIO_40_O_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_41_o(&mut self) -> REG2_GPIO_41_O_W<9> {
        REG2_GPIO_41_O_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_42_o(&mut self) -> REG2_GPIO_42_O_W<10> {
        REG2_GPIO_42_O_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_43_o(&mut self) -> REG2_GPIO_43_O_W<11> {
        REG2_GPIO_43_O_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_44_o(&mut self) -> REG2_GPIO_44_O_W<12> {
        REG2_GPIO_44_O_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_gpio_45_o(&mut self) -> REG2_GPIO_45_O_W<13> {
        REG2_GPIO_45_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg137\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg137](index.html) module"]
pub struct GPIO_CFG137_SPEC;
impl crate::RegisterSpec for GPIO_CFG137_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg137::R](R) reader structure"]
impl crate::Readable for GPIO_CFG137_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg137::W](W) writer structure"]
impl crate::Writable for GPIO_CFG137_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg137 to value 0"]
impl crate::Resettable for GPIO_CFG137_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
