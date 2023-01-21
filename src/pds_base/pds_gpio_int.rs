#[doc = "Register `pds_gpio_int` reader"]
pub struct R(crate::R<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_int` writer"]
pub struct W(crate::W<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_INT_SPEC>;
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
impl From<crate::W<PDS_GPIO_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set1_int_clr` reader - "]
pub type PDS_GPIO_SET1_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set1_int_clr` writer - "]
pub type PDS_GPIO_SET1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set1_int_mode` reader - "]
pub type PDS_GPIO_SET1_INT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set1_int_mode` writer - "]
pub type PDS_GPIO_SET1_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_8_9` reader - "]
pub type RESERVED_8_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set2_int_clr` reader - "]
pub type PDS_GPIO_SET2_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set2_int_clr` writer - "]
pub type PDS_GPIO_SET2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set2_int_mode` reader - "]
pub type PDS_GPIO_SET2_INT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set2_int_mode` writer - "]
pub type PDS_GPIO_SET2_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_16_17` reader - "]
pub type RESERVED_16_17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set3_int_clr` reader - "]
pub type PDS_GPIO_SET3_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set3_int_clr` writer - "]
pub type PDS_GPIO_SET3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set3_int_mode` reader - "]
pub type PDS_GPIO_SET3_INT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set3_int_mode` writer - "]
pub type PDS_GPIO_SET3_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_24_25` reader - "]
pub type RESERVED_24_25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set4_int_clr` reader - "]
pub type PDS_GPIO_SET4_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set4_int_clr` writer - "]
pub type PDS_GPIO_SET4_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `reserved_27` reader - "]
pub type RESERVED_27_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_set4_int_mode` reader - "]
pub type PDS_GPIO_SET4_INT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_set4_int_mode` writer - "]
pub type PDS_GPIO_SET4_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_clr(&self) -> PDS_GPIO_SET1_INT_CLR_R {
        PDS_GPIO_SET1_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_mode(&self) -> PDS_GPIO_SET1_INT_MODE_R {
        PDS_GPIO_SET1_INT_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reserved_8_9(&self) -> RESERVED_8_9_R {
        RESERVED_8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_clr(&self) -> PDS_GPIO_SET2_INT_CLR_R {
        PDS_GPIO_SET2_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_mode(&self) -> PDS_GPIO_SET2_INT_MODE_R {
        PDS_GPIO_SET2_INT_MODE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn reserved_16_17(&self) -> RESERVED_16_17_R {
        RESERVED_16_17_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_clr(&self) -> PDS_GPIO_SET3_INT_CLR_R {
        PDS_GPIO_SET3_INT_CLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_mode(&self) -> PDS_GPIO_SET3_INT_MODE_R {
        PDS_GPIO_SET3_INT_MODE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn reserved_24_25(&self) -> RESERVED_24_25_R {
        RESERVED_24_25_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_clr(&self) -> PDS_GPIO_SET4_INT_CLR_R {
        PDS_GPIO_SET4_INT_CLR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved_27(&self) -> RESERVED_27_R {
        RESERVED_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_mode(&self) -> PDS_GPIO_SET4_INT_MODE_R {
        PDS_GPIO_SET4_INT_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set1_int_clr(&mut self) -> PDS_GPIO_SET1_INT_CLR_W<2> {
        PDS_GPIO_SET1_INT_CLR_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set1_int_mode(&mut self) -> PDS_GPIO_SET1_INT_MODE_W<4> {
        PDS_GPIO_SET1_INT_MODE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set2_int_clr(&mut self) -> PDS_GPIO_SET2_INT_CLR_W<10> {
        PDS_GPIO_SET2_INT_CLR_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set2_int_mode(&mut self) -> PDS_GPIO_SET2_INT_MODE_W<12> {
        PDS_GPIO_SET2_INT_MODE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set3_int_clr(&mut self) -> PDS_GPIO_SET3_INT_CLR_W<18> {
        PDS_GPIO_SET3_INT_CLR_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set3_int_mode(&mut self) -> PDS_GPIO_SET3_INT_MODE_W<20> {
        PDS_GPIO_SET3_INT_MODE_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set4_int_clr(&mut self) -> PDS_GPIO_SET4_INT_CLR_W<26> {
        PDS_GPIO_SET4_INT_CLR_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set4_int_mode(&mut self) -> PDS_GPIO_SET4_INT_MODE_W<28> {
        PDS_GPIO_SET4_INT_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_gpio_int\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_int](index.html) module"]
pub struct PDS_GPIO_INT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_int::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_int::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_int to value 0"]
impl crate::Resettable for PDS_GPIO_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
