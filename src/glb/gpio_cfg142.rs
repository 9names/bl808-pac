#[doc = "Register `gpio_cfg142` reader"]
pub struct R(crate::R<GPIO_CFG142_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFG142_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFG142_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFG142_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_cfg142` writer"]
pub struct W(crate::W<GPIO_CFG142_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFG142_SPEC>;
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
impl From<crate::W<GPIO_CFG142_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFG142_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_gpio_tx_en` reader - "]
pub type CR_GPIO_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_gpio_tx_en` writer - "]
pub type CR_GPIO_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFG142_SPEC, bool, O>;
#[doc = "Field `cr_invert_code0_high` reader - "]
pub type CR_INVERT_CODE0_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `cr_invert_code0_high` writer - "]
pub type CR_INVERT_CODE0_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG142_SPEC, bool, O>;
#[doc = "Field `cr_invert_code1_high` reader - "]
pub type CR_INVERT_CODE1_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `cr_invert_code1_high` writer - "]
pub type CR_INVERT_CODE1_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFG142_SPEC, bool, O>;
#[doc = "Field `reserved_3_6` reader - "]
pub type RESERVED_3_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_code_total_time` reader - "]
pub type CR_CODE_TOTAL_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_code_total_time` writer - "]
pub type CR_CODE_TOTAL_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG142_SPEC, u16, u16, 9, O>;
#[doc = "Field `cr_code0_high_time` reader - "]
pub type CR_CODE0_HIGH_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_code0_high_time` writer - "]
pub type CR_CODE0_HIGH_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG142_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_code1_high_time` reader - "]
pub type CR_CODE1_HIGH_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_code1_high_time` writer - "]
pub type CR_CODE1_HIGH_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFG142_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_tx_en(&self) -> CR_GPIO_TX_EN_R {
        CR_GPIO_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_invert_code0_high(&self) -> CR_INVERT_CODE0_HIGH_R {
        CR_INVERT_CODE0_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_invert_code1_high(&self) -> CR_INVERT_CODE1_HIGH_R {
        CR_INVERT_CODE1_HIGH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn reserved_3_6(&self) -> RESERVED_3_6_R {
        RESERVED_3_6_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn cr_code_total_time(&self) -> CR_CODE_TOTAL_TIME_R {
        CR_CODE_TOTAL_TIME_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_code0_high_time(&self) -> CR_CODE0_HIGH_TIME_R {
        CR_CODE0_HIGH_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_code1_high_time(&self) -> CR_CODE1_HIGH_TIME_R {
        CR_CODE1_HIGH_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_tx_en(&mut self) -> CR_GPIO_TX_EN_W<0> {
        CR_GPIO_TX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_invert_code0_high(&mut self) -> CR_INVERT_CODE0_HIGH_W<1> {
        CR_INVERT_CODE0_HIGH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_invert_code1_high(&mut self) -> CR_INVERT_CODE1_HIGH_W<2> {
        CR_INVERT_CODE1_HIGH_W::new(self)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_code_total_time(&mut self) -> CR_CODE_TOTAL_TIME_W<7> {
        CR_CODE_TOTAL_TIME_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_code0_high_time(&mut self) -> CR_CODE0_HIGH_TIME_W<16> {
        CR_CODE0_HIGH_TIME_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_code1_high_time(&mut self) -> CR_CODE1_HIGH_TIME_W<24> {
        CR_CODE1_HIGH_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpio_cfg142\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfg142](index.html) module"]
pub struct GPIO_CFG142_SPEC;
impl crate::RegisterSpec for GPIO_CFG142_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfg142::R](R) reader structure"]
impl crate::Readable for GPIO_CFG142_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfg142::W](W) writer structure"]
impl crate::Writable for GPIO_CFG142_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_cfg142 to value 0xc8c8_c800"]
impl crate::Resettable for GPIO_CFG142_SPEC {
    const RESET_VALUE: Self::Ux = 0xc8c8_c800;
}
