#[doc = "Register `touch_int_setting` reader"]
pub struct R(crate::R<TOUCH_INT_SETTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_INT_SETTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_INT_SETTING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_INT_SETTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch_int_setting` writer"]
pub struct W(crate::W<TOUCH_INT_SETTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_INT_SETTING_SPEC>;
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
impl From<crate::W<TOUCH_INT_SETTING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_INT_SETTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_int_clr` reader - "]
pub type TOUCH_INT_CLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_int_clr` writer - "]
pub type TOUCH_INT_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_INT_SETTING_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_int_mask` reader - "]
pub type TOUCH_INT_MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `touch_int_mask` writer - "]
pub type TOUCH_INT_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_INT_SETTING_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_28_30` reader - "]
pub type RESERVED_28_30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_int_en` reader - "]
pub type TOUCH_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_int_en` writer - "]
pub type TOUCH_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH_INT_SETTING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn touch_int_clr(&self) -> TOUCH_INT_CLR_R {
        TOUCH_INT_CLR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn touch_int_mask(&self) -> TOUCH_INT_MASK_R {
        TOUCH_INT_MASK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn reserved_28_30(&self) -> RESERVED_28_30_R {
        RESERVED_28_30_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn touch_int_en(&self) -> TOUCH_INT_EN_R {
        TOUCH_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_int_clr(&mut self) -> TOUCH_INT_CLR_W<0> {
        TOUCH_INT_CLR_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn touch_int_mask(&mut self) -> TOUCH_INT_MASK_W<16> {
        TOUCH_INT_MASK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn touch_int_en(&mut self) -> TOUCH_INT_EN_W<31> {
        TOUCH_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch_int_setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_int_setting](index.html) module"]
pub struct TOUCH_INT_SETTING_SPEC;
impl crate::RegisterSpec for TOUCH_INT_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_int_setting::R](R) reader structure"]
impl crate::Readable for TOUCH_INT_SETTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_int_setting::W](W) writer structure"]
impl crate::Writable for TOUCH_INT_SETTING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch_int_setting to value 0x8000_0000"]
impl crate::Resettable for TOUCH_INT_SETTING_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
