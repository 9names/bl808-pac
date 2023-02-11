#[doc = "Register `hbn_pad_ctrl_0` reader"]
pub struct R(crate::R<HBN_PAD_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PAD_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_PAD_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_PAD_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbn_pad_ctrl_0` writer"]
pub struct W(crate::W<HBN_PAD_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PAD_CTRL_0_SPEC>;
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
impl From<crate::W<HBN_PAD_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_PAD_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_aon_pad_ie_smt` reader - "]
pub type REG_AON_PAD_IE_SMT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_aon_pad_ie_smt` writer - "]
pub type REG_AON_PAD_IE_SMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_0_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_9` reader - "]
pub type RESERVED_9_R = crate::BitReader<bool>;
#[doc = "Field `reg_aon_led_sel` reader - "]
pub type REG_AON_LED_SEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_aon_led_sel` writer - "]
pub type REG_AON_LED_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_0_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `reg_en_aon_ctrl_gpio` reader - "]
pub type REG_EN_AON_CTRL_GPIO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_en_aon_ctrl_gpio` writer - "]
pub type REG_EN_AON_CTRL_GPIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_0_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_29_30` reader - "]
pub type RESERVED_29_30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_aon_gpio_iso_mode` reader - "]
pub type REG_AON_GPIO_ISO_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_aon_gpio_iso_mode` writer - "]
pub type REG_AON_GPIO_ISO_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HBN_PAD_CTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> REG_AON_PAD_IE_SMT_R {
        REG_AON_PAD_IE_SMT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved_9(&self) -> RESERVED_9_R {
        RESERVED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_led_sel(&self) -> REG_AON_LED_SEL_R {
        REG_AON_LED_SEL_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_en_aon_ctrl_gpio(&self) -> REG_EN_AON_CTRL_GPIO_R {
        REG_EN_AON_CTRL_GPIO_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn reserved_29_30(&self) -> RESERVED_29_30_R {
        RESERVED_29_30_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_aon_gpio_iso_mode(&self) -> REG_AON_GPIO_ISO_MODE_R {
        REG_AON_GPIO_ISO_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_ie_smt(&mut self) -> REG_AON_PAD_IE_SMT_W<0> {
        REG_AON_PAD_IE_SMT_W::new(self)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_led_sel(&mut self) -> REG_AON_LED_SEL_W<10> {
        REG_AON_LED_SEL_W::new(self)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_aon_ctrl_gpio(&mut self) -> REG_EN_AON_CTRL_GPIO_W<20> {
        REG_EN_AON_CTRL_GPIO_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_gpio_iso_mode(&mut self) -> REG_AON_GPIO_ISO_MODE_W<31> {
        REG_AON_GPIO_ISO_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PAD_CTRL_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pad_ctrl_0](index.html) module"]
pub struct HBN_PAD_CTRL_0_SPEC;
impl crate::RegisterSpec for HBN_PAD_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pad_ctrl_0::R](R) reader structure"]
impl crate::Readable for HBN_PAD_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pad_ctrl_0::W](W) writer structure"]
impl crate::Writable for HBN_PAD_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbn_pad_ctrl_0 to value 0x1800_0000"]
impl crate::Resettable for HBN_PAD_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1800_0000;
}
