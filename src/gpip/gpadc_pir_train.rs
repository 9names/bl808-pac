#[doc = "Register `gpadc_pir_train` reader"]
pub struct R(crate::R<GPADC_PIR_TRAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_PIR_TRAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_PIR_TRAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_PIR_TRAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_pir_train` writer"]
pub struct W(crate::W<GPADC_PIR_TRAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_PIR_TRAIN_SPEC>;
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
impl From<crate::W<GPADC_PIR_TRAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_PIR_TRAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pir_extend` reader - "]
pub type PIR_EXTEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pir_extend` writer - "]
pub type PIR_EXTEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_PIR_TRAIN_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pir_cnt_v` reader - "]
pub type PIR_CNT_V_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_13_15` reader - "]
pub type RESERVED_13_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pir_train` reader - "]
pub type PIR_TRAIN_R = crate::BitReader<bool>;
#[doc = "Field `pir_train` writer - "]
pub type PIR_TRAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_PIR_TRAIN_SPEC, bool, O>;
#[doc = "Field `pir_stop` reader - "]
pub type PIR_STOP_R = crate::BitReader<bool>;
#[doc = "Field `reserved_18_31` reader - "]
pub type RESERVED_18_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pir_extend(&self) -> PIR_EXTEND_R {
        PIR_EXTEND_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn pir_cnt_v(&self) -> PIR_CNT_V_R {
        PIR_CNT_V_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_13_15(&self) -> RESERVED_13_15_R {
        RESERVED_13_15_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pir_train(&self) -> PIR_TRAIN_R {
        PIR_TRAIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pir_stop(&self) -> PIR_STOP_R {
        PIR_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn reserved_18_31(&self) -> RESERVED_18_31_R {
        RESERVED_18_31_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pir_extend(&mut self) -> PIR_EXTEND_W<0> {
        PIR_EXTEND_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pir_train(&mut self) -> PIR_TRAIN_W<16> {
        PIR_TRAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_pir_train](index.html) module"]
pub struct GPADC_PIR_TRAIN_SPEC;
impl crate::RegisterSpec for GPADC_PIR_TRAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_pir_train::R](R) reader structure"]
impl crate::Readable for GPADC_PIR_TRAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_pir_train::W](W) writer structure"]
impl crate::Writable for GPADC_PIR_TRAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_pir_train to value 0x0f"]
impl crate::Resettable for GPADC_PIR_TRAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
